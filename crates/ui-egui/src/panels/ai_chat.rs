//! Window ▸ AI Chat: chat with a local model (Ollama) or with anything the local bridge
//! exposes — Codex on a ChatGPT plan, Devin over ACP, OpenCode, OpenAI-compatible APIs, any
//! CLI. The bridge is `bridge/agent-bridge.py` in the `effectcraft-ai-chat` repository.
//!
//! Both the scripting Socket API and this panel reach `127.0.0.1` only; the bridge is what
//! reaches cloud APIs and CLIs, and it keeps API keys out of the app. Requests run on a worker
//! thread and are polled every frame, so the interface keeps drawing while a model thinks.

use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

use egui::{Align2, Rect, Sense, pos2, vec2};
use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use serde_json::{Value, json};

use crate::EffectcraftApp;
use crate::i18n::tr;
use crate::theme::Tokens;

/// One line of the transcript.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Line {
    /// `you`, `ai`, `error` or `info`.
    pub kind: String,
    pub text: String,
}

/// One message of the conversation sent to an agent.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// `user`, `assistant` or `system`.
    pub role: String,
    pub content: String,
}

/// An agent the bridge reported (`GET /agents`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub ready: bool,
    #[serde(default)]
    pub note: String,
}

/// AI Chat state (serde, so agents can read it).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AiChat {
    pub input: String,
    pub lines: Vec<Line>,
    pub history: Vec<Message>,
    /// The bridge's agents, last fetched.
    pub agents: Vec<Agent>,
    pub status: String,
}

impl AiChat {
    fn push(&mut self, kind: &str, text: &str) {
        self.lines.push(Line { kind: kind.into(), text: text.to_string() });
        if self.lines.len() > 500 {
            let extra = self.lines.len() - 500;
            self.lines.drain(..extra);
        }
    }
}

/// A finished model request: the reply text and the id of the model that answered it.
pub struct Reply {
    pub agent: String,
    pub result: Result<(String, Option<String>), String>,
}

/// Worker threads behind the panel (not serialized; lives on the app).
#[derive(Default)]
pub struct Runtime {
    reply: Option<Receiver<Reply>>,
    agents: Option<Receiver<Result<Vec<Agent>, String>>>,
    pub busy: bool,
    fetched: bool,
}

/// `http://127.0.0.1:8765/` → `127.0.0.1:8765`.
pub fn host_of(url: &str) -> &str {
    url.trim().trim_start_matches("http://").trim_start_matches("https://").trim_end_matches('/')
}

/// The picker label for an agent id.
fn label_of(selected: &str, agents: &[Agent]) -> String {
    if selected == "ollama" {
        return tr("Ollama (direct)").to_string();
    }
    agents.iter().find(|a| a.id == selected).map(|a| a.label.clone()).unwrap_or_else(|| selected.to_string())
}

#[cfg(not(target_arch = "wasm32"))]
mod http {
    //! One HTTP/1.1 request to a local server, over `std::net` — the app has no HTTP client
    //! dependency, and the panel reaches `127.0.0.1` only. Plain HTTP, like the bridge and
    //! Ollama speak; TLS belongs to the bridge.

    use std::io::{Read, Write};
    use std::net::{TcpStream, ToSocketAddrs};
    use std::time::Duration;

    pub fn json(host: &str, method: &str, path: &str, body: Option<&str>, timeout: Duration) -> Result<serde_json::Value, String> {
        let addr = host.to_socket_addrs().map_err(|e| format!("bad address `{host}`: {e}"))?.next().ok_or_else(|| format!("bad address `{host}`"))?;
        let mut s = TcpStream::connect_timeout(&addr, timeout).map_err(|e| format!("cannot reach {host} ({e})"))?;
        let _ = s.set_read_timeout(Some(timeout));
        let _ = s.set_write_timeout(Some(timeout));
        let body = body.unwrap_or("");
        let head = format!(
            "{method} {path} HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        s.write_all(head.as_bytes()).and_then(|_| s.write_all(body.as_bytes())).map_err(|e| format!("write to {host}: {e}"))?;
        let mut raw = Vec::new();
        let mut buf = [0u8; 8192];
        loop {
            match s.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    raw.extend_from_slice(buf.get(..n).unwrap_or(&buf));
                    if raw.len() > 8_388_608 {
                        return Err("response too large".into());
                    }
                }
                // A server that closes with unread request data resets the connection; whatever
                // arrived is still parsed below (an incomplete response fails there, with the body).
                Err(_) => break,
            }
        }
        let text = String::from_utf8_lossy(&raw).into_owned();
        let (head, body) = text.split_once("\r\n\r\n").ok_or_else(|| format!("no HTTP response from {host} ({} bytes)", text.len()))?;
        let status: u32 = head.split_whitespace().nth(1).and_then(|c| c.parse().ok()).unwrap_or(0);
        let v: serde_json::Value = serde_json::from_str(body).map_err(|e| format!("bad JSON from {host}: {e}"))?;
        if status != 200 {
            let msg = v.get("error").and_then(|e| e.as_str()).unwrap_or("HTTP error");
            return Err(format!("{msg} (HTTP {status})"));
        }
        Ok(v)
    }
}

/// One chat request; `ollama` talks to the local model, anything else to the bridge. The reply
/// carries the id of the model that answered (the bridge resolves `aiChat.agentModel`).
#[cfg(not(target_arch = "wasm32"))]
fn request(agent: &str, messages: &[Message], prefs: &effectcraft_engine::prefs::AiChatPrefs) -> Result<(String, Option<String>), String> {
    let msgs: Vec<Value> = messages.iter().map(|m| json!({"role": m.role, "content": m.content})).collect();
    if agent == "ollama" || agent.is_empty() {
        let body = json!({"model": prefs.model, "stream": false, "messages": msgs}).to_string();
        let v = http::json(host_of(&prefs.ollama_url), "POST", "/api/chat", Some(&body), Duration::from_secs(300))?;
        let reply = v.get("message").and_then(|m| m.get("content")).and_then(Value::as_str).unwrap_or_default();
        Ok((if reply.is_empty() { "(empty reply)".into() } else { reply.to_string() }, Some(prefs.model.clone())))
    } else {
        let mut body = json!({"agent": agent, "messages": msgs});
        if !prefs.agent_model.is_empty() {
            body["model"] = json!(prefs.agent_model);
        }
        let v = http::json(host_of(&prefs.bridge_url), "POST", "/chat", Some(&body.to_string()), Duration::from_secs(600))?;
        if let Some(e) = v.get("error").and_then(Value::as_str) {
            return Err(e.to_string());
        }
        let reply = v.get("reply").and_then(Value::as_str).unwrap_or("(empty reply)").to_string();
        let model = v.get("model").and_then(Value::as_str).map(str::to_string);
        Ok((reply, model))
    }
}

/// Send the input as the next message (spawns a worker thread).
pub fn send(app: &mut EffectcraftApp, ctx: &egui::Context) {
    #[cfg(target_arch = "wasm32")]
    let _ = ctx;
    let text = app.ui.ai_chat.input.trim().to_string();
    if text.is_empty() || app.ai_chat.busy {
        return;
    }
    let prefs = app.session.prefs.ai_chat.clone();
    let agent = if prefs.agent.trim().is_empty() { "ollama".to_string() } else { prefs.agent.clone() };
    let chat = &mut app.ui.ai_chat;
    chat.input.clear();
    chat.push("you", &text);
    chat.history.push(Message { role: "user".into(), content: text });
    chat.status = format!("{} {agent} …", tr("asking"));
    let mut messages = vec![Message { role: "system".into(), content: prefs.system_prompt.clone() }];
    messages.extend(chat.history.iter().cloned());

    if cfg!(target_arch = "wasm32") {
        chat.push("error", "The browser build can't reach local servers; the AI Chat panel runs in the desktop app.");
        chat.status = "not available in the browser".into();
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let (tx, rx) = std::sync::mpsc::channel();
        app.ai_chat.reply = Some(rx);
        app.ai_chat.busy = true;
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let result = request(&agent, &messages, &prefs);
            let _ = tx.send(Reply { agent, result });
            ctx.request_repaint();
        });
    }
}

/// Fetch the bridge's agent list (spawns a worker thread).
pub fn fetch_agents(app: &mut EffectcraftApp, ctx: &egui::Context) {
    #[cfg(target_arch = "wasm32")]
    let _ = ctx;
    if cfg!(target_arch = "wasm32") {
        app.ui.ai_chat.status = "the bridge runs beside the desktop app".into();
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        if app.ai_chat.agents.is_some() {
            return;
        }
        let url = app.session.prefs.ai_chat.bridge_url.clone();
        let (tx, rx) = std::sync::mpsc::channel();
        app.ai_chat.agents = Some(rx);
        let ctx = ctx.clone();
        std::thread::spawn(move || {
            let r = http::json(host_of(&url), "GET", "/agents", None, Duration::from_secs(3))
                .map(|v| v.get("agents").and_then(Value::as_array).cloned().unwrap_or_default())
                .map(|list| list.iter().filter_map(|a| serde_json::from_value::<Agent>(a.clone()).ok()).collect::<Vec<_>>());
            let _ = tx.send(r);
            ctx.request_repaint();
        });
    }
}

/// Collect finished worker replies (called every frame).
pub fn poll(app: &mut EffectcraftApp, ctx: &egui::Context) {
    if let Some(rx) = &app.ai_chat.reply {
        match rx.try_recv() {
            Ok(r) => {
                app.ai_chat.reply = None;
                app.ai_chat.busy = false;
                let chat = &mut app.ui.ai_chat;
                match r.result {
                    Ok((text, model)) => {
                        chat.push("ai", &text);
                        chat.history.push(Message { role: "assistant".into(), content: text });
                        chat.status = match model {
                            Some(m) if !m.is_empty() => format!("ready · {m}"),
                            _ => "ready".into(),
                        };
                    }
                    Err(e) => {
                        // The failed turn doesn't stay in the conversation, so a retry doesn't duplicate it.
                        chat.history.pop();
                        chat.push("error", &e);
                        chat.status = format!("error: {e}");
                    }
                }
            }
            Err(TryRecvError::Empty) => ctx.request_repaint_after(Duration::from_millis(150)),
            Err(TryRecvError::Disconnected) => {
                app.ai_chat.reply = None;
                app.ai_chat.busy = false;
                app.ui.ai_chat.status = "the request thread stopped".into();
            }
        }
    }
    if let Some(rx) = &app.ai_chat.agents {
        match rx.try_recv() {
            Ok(Ok(list)) => {
                app.ai_chat.agents = None;
                let n = list.len();
                app.ui.ai_chat.agents = list;
                app.ui.ai_chat.status = format!("{}: {n}", tr("bridge agents"));
            }
            Ok(Err(e)) => {
                app.ai_chat.agents = None;
                app.ui.ai_chat.status = format!("{} ({e})", tr("bridge off — run bridge/agent-bridge.py"));
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => app.ai_chat.agents = None,
        }
    }
}

pub fn show(app: &mut EffectcraftApp, ui: &mut egui::Ui, rect: Rect) {
    poll(app, ui.ctx());
    let t = app.tokens;

    // First open: try the bridge once.
    if !app.ai_chat.fetched && app.ai_chat.agents.is_none() && app.ui.ai_chat.agents.is_empty() {
        app.ai_chat.fetched = true;
        let ctx = ui.ctx().clone();
        fetch_agents(app, &ctx);
    }

    // Top bar: agent picker and Refresh.
    let bar_h = 26.0;
    let top = Rect::from_min_size(rect.min, vec2(rect.width(), bar_h));
    let agents = app.ui.ai_chat.agents.clone();
    let selected = {
        let a = app.session.prefs.ai_chat.agent.trim();
        if a.is_empty() { "ollama".to_string() } else { a.to_string() }
    };
    let mut pick: Option<String> = None;
    let mut refresh = false;
    {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(top));
        child.horizontal(|ui| {
            ui.label(tr("Agent:"));
            egui::ComboBox::from_id_salt("ai-chat-agent").selected_text(label_of(&selected, &agents)).width(128.0).show_ui(ui, |ui| {
                if ui.selectable_label(selected == "ollama", tr("Ollama (direct)")).clicked() {
                    pick = Some("ollama".into());
                }
                for a in &agents {
                    let text = if a.ready { a.label.clone() } else { format!("{} — {}", a.label, tr("not ready")) };
                    if ui.selectable_label(selected == a.id, text).clicked() {
                        pick = Some(a.id.clone());
                    }
                }
            });
            if ui.button(tr("Refresh")).clicked() {
                refresh = true;
            }
        });
    }
    app.auto.add("aiChat.agent", top, "AI Chat agent");
    if let Some(id) = pick {
        // The `aiChat.agent` pref is the single source of truth (the Settings page edits it too).
        let ctx = ui.ctx().clone();
        let _ = crate::menus::invoke(app, &ctx, "prefs.set", serde_json::json!({"key": "aiChat.agent", "value": id}));
    }
    if refresh {
        let ctx = ui.ctx().clone();
        fetch_agents(app, &ctx);
    }

    // Transcript.
    let input_h = 60.0;
    let status_h = 26.0;
    let body = Rect::from_min_max(pos2(rect.min.x, top.max.y + 2.0), pos2(rect.max.x, rect.max.y - input_h - status_h - 6.0));
    {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body));
        egui::ScrollArea::vertical().id_salt("ai-chat-log").auto_shrink([false, false]).stick_to_bottom(true).show(&mut child, |ui| {
            if app.ui.ai_chat.lines.is_empty() {
                ui.add_space(6.0);
                ui.weak(tr("Ask a local model about your project. Start bridge/agent-bridge.py to reach Codex, Devin, OpenCode and more."));
            }
            for l in &app.ui.ai_chat.lines {
                let color = match l.kind.as_str() {
                    "you" => t.text_faint,
                    "error" => t.danger,
                    _ => t.text,
                };
                ui.colored_label(color, &l.text);
                ui.add_space(3.0);
            }
        });
    }
    app.auto.add("aiChat.transcript", body, "AI Chat transcript");

    // Input.
    let in_rect = Rect::from_min_size(pos2(rect.min.x + 4.0, rect.max.y - input_h - status_h - 4.0), vec2(rect.width() - 8.0, input_h - 6.0));
    let mut send_now = false;
    {
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(in_rect));
        let te = egui::TextEdit::multiline(&mut app.ui.ai_chat.input)
            .id(egui::Id::new("ai-chat-input"))
            .font(Tokens::ui(12.5))
            .desired_width(in_rect.width())
            .desired_rows(2)
            .hint_text(tr("Message… (⌘/Ctrl+Enter sends)"));
        let resp = child.add_sized(in_rect.size(), te);
        if resp.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.command) {
            send_now = true;
        }
    }
    app.auto.add("aiChat.input", in_rect, "AI Chat input");

    // Status and the Settings / Clear / Send buttons.
    let bar = Rect::from_min_size(pos2(rect.min.x, rect.max.y - status_h), vec2(rect.width(), status_h));
    let send_r = Rect::from_min_size(pos2(bar.max.x - 64.0, bar.center().y - 10.0), vec2(56.0, 20.0));
    let clear_r = Rect::from_min_size(pos2(send_r.min.x - 52.0, send_r.min.y), vec2(48.0, 20.0));
    let settings_r = Rect::from_min_size(pos2(clear_r.min.x - 68.0, send_r.min.y), vec2(64.0, 20.0));
    let send_resp = ui.interact(send_r, egui::Id::new("ai-chat-send"), Sense::click());
    let clear_resp = ui.interact(clear_r, egui::Id::new("ai-chat-clear"), Sense::click());
    let settings_resp = ui.interact(settings_r, egui::Id::new("ai-chat-settings"), Sense::click());
    let p = ui.painter().with_clip_rect(rect);
    let accent = if app.ai_chat.busy {
        t.field_border
    } else if send_resp.hovered() {
        t.accent_hover
    } else {
        t.accent
    };
    p.rect_filled(send_r, 3.0, accent);
    p.text(send_r.center(), Align2::CENTER_CENTER, tr("Send"), Tokens::ui(12.0), egui::Color32::WHITE);
    p.rect_stroke(clear_r, 3.0, egui::Stroke::new(1.0, t.field_border), egui::StrokeKind::Inside);
    p.text(clear_r.center(), Align2::CENTER_CENTER, tr("Clear"), Tokens::ui(12.0), t.text);
    p.rect_stroke(settings_r, 3.0, egui::Stroke::new(1.0, t.field_border), egui::StrokeKind::Inside);
    p.text(settings_r.center(), Align2::CENTER_CENTER, tr("Settings"), Tokens::ui(12.0), t.text);
    app.auto.add("aiChat.send", send_r, "Send");
    app.auto.add("aiChat.clear", clear_r, "Clear");
    app.auto.add("aiChat.settings", settings_r, "AI Chat settings");
    let status = app.ui.ai_chat.status.clone();
    if !status.is_empty() {
        let sp = ui.painter().with_clip_rect(Rect::from_min_max(pos2(rect.min.x, bar.min.y), pos2(settings_r.min.x - 4.0, bar.max.y)));
        sp.text(pos2(rect.min.x + 6.0, bar.center().y), Align2::LEFT_CENTER, &status, Tokens::ui(11.0), t.text_faint);
    }
    if settings_resp.clicked() {
        // One click from the panel to its Settings page (also Settings ▸ AI Chat… in the menu bar).
        let ctx = ui.ctx().clone();
        let _ = crate::menus::invoke(app, &ctx, "app.settings", serde_json::json!({"page": "aiChat"}));
    }
    if clear_resp.clicked() {
        app.ui.ai_chat.lines.clear();
        app.ui.ai_chat.history.clear();
        app.ui.ai_chat.status = tr("cleared").into();
    }
    if send_resp.clicked() {
        send_now = true;
    }
    if send_now {
        let ctx = ui.ctx().clone();
        send(app, &ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_of_strips_scheme_and_slash() {
        assert_eq!(host_of("http://127.0.0.1:8765/"), "127.0.0.1:8765");
        assert_eq!(host_of(" 127.0.0.1:11434 "), "127.0.0.1:11434");
    }

    #[test]
    fn push_keeps_the_transcript_bounded() {
        let mut chat = AiChat::default();
        for i in 0..520 {
            chat.push("ai", &format!("line {i}"));
        }
        assert_eq!(chat.lines.len(), 500);
        assert_eq!(chat.lines.first().map(|l| l.text.as_str()), Some("line 20"));
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn http_json_round_trips_against_a_local_server() {
        let (addr, server) = serve_once("HTTP/1.1 200 OK", r#"{"reply":"pong"}"#);
        let v = http::json(&addr.to_string(), "POST", "/chat", Some("{}"), Duration::from_secs(5)).unwrap();
        assert_eq!(v["reply"], "pong");
        server.join().unwrap();
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn http_json_reports_http_errors() {
        let (addr, server) = serve_once("HTTP/1.1 500 Internal Server Error", r#"{"error":"boom"}"#);
        let e = http::json(&addr.to_string(), "POST", "/chat", Some("{}"), Duration::from_secs(5)).unwrap_err();
        assert!(e.contains("boom"), "{e}");
        server.join().unwrap();
    }

    /// A one-shot HTTP server that reads the whole request before replying (closing with unread
    /// request data would reset the connection instead of delivering the response). The thread
    /// returns the raw request it received.
    #[cfg(not(target_arch = "wasm32"))]
    fn serve_once(status_line: &str, body: &str) -> (std::net::SocketAddr, std::thread::JoinHandle<String>) {
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let response = format!("{status_line}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
        let handle = std::thread::spawn(move || {
            let (mut s, _) = listener.accept().unwrap();
            let mut req = Vec::new();
            let mut buf = [0u8; 1024];
            loop {
                if let Some(end) = req.windows(4).position(|w| w == b"\r\n\r\n") {
                    let head = String::from_utf8_lossy(&req[..end]).to_string();
                    let len: usize = head.lines().find_map(|l| l.strip_prefix("Content-Length: ").and_then(|v| v.trim().parse().ok())).unwrap_or(0);
                    if req.len() >= end + 4 + len {
                        break;
                    }
                }
                match s.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => req.extend_from_slice(&buf[..n]),
                }
            }
            let _ = s.write_all(response.as_bytes());
            String::from_utf8_lossy(&req).into_owned()
        });
        (addr, handle)
    }

    /// The bridge gets `agent`, `messages` and — only when a model is chosen in Settings —
    /// `model`; it answers with the id of the model that replied, which the status line shows.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn request_sends_the_chosen_agent_model_to_the_bridge() {
        let (addr, server) = serve_once("HTTP/1.1 200 OK", r#"{"reply":"opus ok","model":"claude-opus-5-5-medium"}"#);
        let prefs =
            effectcraft_engine::prefs::AiChatPrefs { bridge_url: addr.to_string(), agent: "devin".into(), agent_model: "opus".into(), ..Default::default() };
        let (reply, model) = request("devin", &[Message { role: "user".into(), content: "hi".into() }], &prefs).unwrap();
        assert_eq!(reply, "opus ok");
        assert_eq!(model.as_deref(), Some("claude-opus-5-5-medium"));
        let req = server.join().unwrap();
        assert!(req.contains(r#""agent":"devin""#), "{req}");
        assert!(req.contains(r#""model":"opus""#), "{req}");
    }

    /// At Agent Default the request carries no `model`, so the agent keeps its own.
    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn request_omits_the_model_at_agent_default() {
        let (addr, server) = serve_once("HTTP/1.1 200 OK", r#"{"reply":"hi"}"#);
        let prefs = effectcraft_engine::prefs::AiChatPrefs { bridge_url: addr.to_string(), agent: "devin".into(), ..Default::default() };
        let (reply, model) = request("devin", &[Message { role: "user".into(), content: "hi".into() }], &prefs).unwrap();
        assert_eq!(reply, "hi");
        assert_eq!(model, None);
        let req = server.join().unwrap();
        assert!(!req.contains(r#""model""#), "{req}");
    }
}
