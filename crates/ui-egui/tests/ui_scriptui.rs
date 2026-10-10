//! M13.1 UI: ScriptUI dialogs, palettes and dockable panels drawn from the session's script
//! windows (clicks reach the script's handlers), the branching History panel, and the Window /
//! File ▸ Scripts menus listing scripts (egui_kittest, UI logic only).

use effectcraft_ui_egui::EffectcraftApp;
use effectcraft_ui_egui::dock::PanelKind;
use egui::{Event, Rect, pos2, vec2};
use egui_kittest::Harness;
use serde_json::json;

fn harness() -> Harness<'static, EffectcraftApp> {
    let mut s = effectcraft_host::session();
    s.execute("comp.new", json!({"name": "Main", "width": 320, "height": 180, "duration": 4})).unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(|_| EffectcraftApp::new(s));
    h.run_steps(3);
    h
}

fn rect(h: &Harness<'_, EffectcraftApp>, id: &str) -> Rect {
    let e = h.state().auto.find(id).unwrap_or_else(|| panic!("no {id}"));
    Rect::from_min_size(pos2(e.rect[0], e.rect[1]), vec2(e.rect[2], e.rect[3]))
}

fn click(h: &mut Harness<'_, EffectcraftApp>, id: &str) {
    let c = rect(h, id).center();
    h.input_mut().events.push(Event::PointerMoved(c));
    for pressed in [true, false] {
        h.input_mut().events.push(Event::PointerButton { pos: c, button: egui::PointerButton::Primary, pressed, modifiers: Default::default() });
    }
    h.run_steps(3);
}

fn layer_names(h: &Harness<'_, EffectcraftApp>) -> Vec<String> {
    h.state().session.active_comp().map(|c| c.layers.iter().map(|l| l.name.clone()).collect()).unwrap_or_default()
}

#[test]
fn script_windows_are_drawn_and_clickable() {
    let mut h = harness();
    let code = r#"
      var w = new Window("palette", "Quick Solid");
      var n = w.add("edittext", undefined, "Card");
      n.characters = 10;
      var go = w.add("button", undefined, "Add", { name: "add" });
      go.onClick = function () { app.project.activeItem.layers.addSolid([0, 1, 0], n.text, 50, 50, 1); };
      w.show();
    "#;
    let r = h.state_mut().session.execute("script.run", json!({"code": code, "name": "quick.jsx"})).unwrap();
    assert_eq!(r["ok"], true, "{r}");
    h.run_steps(3);
    let win = h.state().session.script_ui.windows[0].id;
    // Every control has an automation id (by id and by name).
    for id in [format!("scriptui.{win}"), format!("scriptui.{win}.1"), format!("scriptui.{win}.2"), format!("scriptui.{win}.add")] {
        assert!(h.state().auto.find(&id).is_some(), "missing {id}");
    }
    click(&mut h, &format!("scriptui.{win}.add"));
    assert_eq!(layer_names(&h), ["Card"]);
    // A modal dialog: the script waits in show() until OK is clicked.
    let code = r#"
      var d = new Window("dialog", "Confirm");
      d.add("statictext", undefined, "Make a null?");
      var g = d.add("group");
      g.add("button", undefined, "Cancel", { name: "cancel" });
      g.add("button", undefined, "OK", { name: "ok" });
      if (d.show() === 1) app.project.activeItem.layers.addNull().name = "Yes";
    "#;
    let r = h.state_mut().session.execute("script.run", json!({"code": code, "name": "confirm.jsx"})).unwrap();
    assert_eq!(r["waiting"], true, "{r}");
    h.run_steps(3);
    let d = h.state().session.script_ui.windows.iter().find(|w| w.modal).unwrap().id;
    click(&mut h, &format!("scriptui.{d}.ok"));
    assert_eq!(layer_names(&h), ["Yes", "Card"]);
    assert!(h.state().session.script_ui.windows.iter().all(|w| !w.modal));
    // Window ▸ <panel>: the sample ScriptUI panel docks as its own tab.
    h.state_mut().session.execute("window.scriptPanel", json!({"name": "Layer Tools.jsx"})).unwrap();
    h.run_steps(3);
    let panel = h.state().session.script_ui.windows.iter().find(|w| w.script == "Layer Tools.jsx").unwrap().id;
    assert!(h.state().ui.dock.contains(PanelKind::ScriptPanel(panel)), "the panel docks");
    assert!(h.state().auto.find(&format!("scriptui.{panel}")).is_some());
    // The menus list scripts and panels.
    let cx = effectcraft_engine::menus::DynCtx::default();
    let (panels, _) = effectcraft_engine::menus::dynamic(&h.state().session, "scriptPanels", &cx);
    assert!(panels.iter().any(|e| e.label == "Layer Tools.jsx" && e.command == "window.scriptPanel"));
    let (scripts, _) = effectcraft_engine::menus::dynamic(&h.state().session, "scripts", &cx);
    assert!(scripts.iter().any(|e| e.label == "Rename Layers.jsx" && e.command == "file.runScript"));
    // Closing the panel's tab closes its script window.
    h.state_mut().close_panel(PanelKind::ScriptPanel(panel));
    h.run_steps(2);
    assert!(h.state().session.script_ui.window(panel).is_none());
    h.state_mut().session.execute("scriptui.close", json!({})).unwrap();
}

/// A ScriptUI panel open when the app quit opens again, docked, at the next launch (the
/// frontends call `window.restoreScriptPanels` before building the app).
#[test]
fn panels_open_at_quit_dock_again_at_launch() {
    use effectcraft_engine::config::{ConfigStore, MemoryConfig};
    let store = std::sync::Arc::new(MemoryConfig::default());
    store.write("Scripts/ScriptUI Panels/Hello Panel.jsx", "this.add('button', undefined, 'Hi', { name: 'hi' });").unwrap();
    store.write(effectcraft_engine::commands::scripts::OPEN_PANELS_FILE, r#"{"open": ["Hello Panel.jsx"]}"#).unwrap();
    let mut s = effectcraft_host::session();
    s.config = Some(store.clone());
    s.execute("window.restoreScriptPanels", json!({})).unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(|_| EffectcraftApp::new(s));
    h.run_steps(3);
    let panel = h.state().session.script_ui.windows.iter().find(|w| w.script == "Hello Panel.jsx").expect("the panel's window").id;
    assert!(h.state().ui.dock.contains(PanelKind::ScriptPanel(panel)), "the panel docks");
    assert!(h.state().auto.find(&format!("scriptui.{panel}.hi")).is_some(), "its controls are drawn");
    // Closing its tab forgets it for the next launch.
    h.state_mut().close_panel(PanelKind::ScriptPanel(panel));
    h.run_steps(2);
    assert_eq!(store.read(effectcraft_engine::commands::scripts::OPEN_PANELS_FILE).as_deref(), Some(r#"{"open":[]}"#));
}

/// The bundled Ease Presets ScriptUI panel (Window ▸ Ease Presets.jsx) docks, and clicking a
/// preset in its list eases the selected keyframes in one undo step.
#[test]
fn ease_presets_panel_eases_the_selected_keys_from_its_list() {
    use effectcraft_engine::keyframe::{Ease, Interp};
    use egui_kittest::kittest::Queryable;
    let mut h = harness();
    let s = &mut h.state_mut().session;
    let l = s.execute("layer.newSolid", json!({"name": "Box", "color": "#e04020", "width": 80, "height": 80})).unwrap()["layer"].as_u64().unwrap();
    for (t, v) in [(0.0, 0.0), (1.0, 100.0)] {
        s.execute("prop.addKey", json!({"layer": l, "path": "transform/opacity", "time": t, "value": v})).unwrap();
    }
    s.execute("prop.select", json!({"layer": l, "path": "transform/opacity"})).unwrap();
    let cx = effectcraft_engine::menus::DynCtx::default();
    let (panels, _) = effectcraft_engine::menus::dynamic(s, "scriptPanels", &cx);
    assert!(panels.iter().any(|e| e.label == "Ease Presets.jsx"), "listed in the Window menu");
    s.execute("window.scriptPanel", json!({"name": "Ease Presets.jsx"})).unwrap();
    h.run_steps(3);
    let panel = h.state().session.script_ui.windows.iter().find(|w| w.script == "Ease Presets.jsx").unwrap().id;
    assert!(h.state().ui.dock.contains(PanelKind::ScriptPanel(panel)), "the panel docks");
    h.state_mut().ui.maximized = Some(PanelKind::ScriptPanel(panel));
    h.run_steps(3);
    let steps = h.state().session.history.undo.len();
    let item = h.query_by_label("Ease In-Out").expect("the Ease In-Out preset").rect().center();
    h.input_mut().events.push(Event::PointerMoved(item));
    for pressed in [true, false] {
        h.input_mut().events.push(Event::PointerButton { pos: item, button: egui::PointerButton::Primary, pressed, modifiers: Default::default() });
    }
    h.run_steps(3);
    let op = h.state().session.active_comp().unwrap().layer(effectcraft_engine::project::LayerId(l)).unwrap().props.prop("transform/opacity").unwrap().clone();
    let eased = Ease { speed: 0.0, influence: 0.5 };
    assert_eq!((op.keys[0].out_interp, op.keys[0].out_ease[0]), (Interp::Bezier, eased));
    assert_eq!((op.keys[1].in_interp, op.keys[1].in_ease[0]), (Interp::Bezier, eased));
    assert_eq!(h.state().session.history.undo.len(), steps + 1);
}

#[test]
fn history_panel_jumps_between_branches() {
    let mut h = harness();
    for n in ["A", "B"] {
        h.state_mut().session.execute("layer.newSolid", json!({"name": n, "color": "#ffffff", "width": 10, "height": 10})).unwrap();
    }
    h.state_mut().session.undo();
    h.state_mut().session.execute("layer.newSolid", json!({"name": "C", "color": "#ffffff", "width": 10, "height": 10})).unwrap();
    assert_eq!(layer_names(&h), ["C", "A"]);
    h.state_mut().show_panel(PanelKind::History);
    h.run_steps(3);
    // Original, New Composition, A, C, and B on its branch.
    let states = h.state().session.history_tree();
    assert_eq!(states.len(), 5);
    let b = states.iter().find(|n| n.depth == 1).unwrap().index;
    click(&mut h, &format!("history.state.{b}"));
    assert_eq!(layer_names(&h), ["B", "A"]);
    let c = h.state().session.history_tree().iter().find(|n| n.depth == 1).unwrap().index;
    click(&mut h, &format!("history.state.{c}"));
    assert_eq!(layer_names(&h), ["C", "A"]);
}

/// onDraw fills of concave paths and real images (drawImage, image controls), rendered.
#[test]
fn on_draw_concave_fills_and_images_render() {
    let dir = std::env::temp_dir().join(format!("effectcraft-scriptui-img-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let png = dir.join("green.png");
    image::RgbaImage::from_pixel(8, 8, image::Rgba([0, 255, 0, 255])).save(&png).unwrap();
    let png = png.canonicalize().unwrap().to_string_lossy().replace('\\', "/");
    let mut s = effectcraft_host::session();
    s.execute("comp.new", json!({"name": "Main", "width": 320, "height": 180, "duration": 4})).unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).with_pixels_per_point(1.0).wgpu().build_eframe(|_| EffectcraftApp::new(s));
    h.run_steps(3);
    let code = format!(
        r#"
      var w = new Window("palette", "Paint");
      var c = w.add("group");
      c.preferredSize = [120, 80];
      c.onDraw = function () {{
        var g = this.graphics;
        g.newPath();
        // An L: the top-right quarter is a notch (a convex-only fill would cover it).
        g.moveTo(0, 0); g.lineTo(60, 0); g.lineTo(60, 40); g.lineTo(120, 40); g.lineTo(120, 80); g.lineTo(0, 80); g.closePath();
        g.fillPath(g.newBrush(g.BrushType.SOLID_COLOR, [1, 0, 0, 1]));
        g.drawImage(ScriptUI.newImage(File("{png}")), 90, 5, 20, 20);
      }};
      var im = w.add("image", undefined, File("{png}"));
      im.preferredSize = [40, 40];
      var dim=w.add("image",undefined,File("{png}"));dim.enabled=false;dim.preferredSize=[40,40];
      w.show();
    "#
    );
    let r = h.state_mut().session.execute("script.run", json!({"code": code, "name": "paint.jsx"})).unwrap();
    assert_eq!(r["ok"], true, "{r}");
    h.run_steps(4);
    let win = h.state().session.script_ui.windows[0].clone();
    let g = &win.root.children[0];
    assert!(matches!(&g.draw[1], effectcraft_engine::scriptui::DrawOp::Image { image: Some(i), .. } if i.src.as_deref() == Some(png.as_str())));
    assert_eq!(win.root.children[1].image.as_ref().and_then(|i| i.src.clone()).as_deref(), Some(png.as_str()));
    let img = h.render().expect("render");
    if let Ok(d) = std::env::var("EC_SNAPSHOT_DIR") {
        img.save(format!("{d}/scriptui_paint.png")).unwrap();
    }
    let px = |p: egui::Pos2| img.get_pixel(p.x as u32, p.y as u32).0;
    let gr = rect(&h, &format!("scriptui.{}.{}", win.id, g.id));
    let red = |c: [u8; 4]| c[0] > 200 && c[1] < 60 && c[2] < 60;
    let green = |c: [u8; 4]| c[1] > 200 && c[0] < 60 && c[2] < 60;
    assert!(red(px(gr.min + vec2(20.0, 20.0))), "inside the L: {:?}", px(gr.min + vec2(20.0, 20.0)));
    assert!(red(px(gr.min + vec2(100.0, 60.0))), "the L's foot");
    assert!(!red(px(gr.min + vec2(75.0, 30.0))), "the notch stays empty: {:?}", px(gr.min + vec2(75.0, 30.0)));
    assert!(green(px(gr.min + vec2(100.0, 15.0))), "drawImage: {:?}", px(gr.min + vec2(100.0, 15.0)));
    let ir = rect(&h, &format!("scriptui.{}.{}", win.id, win.root.children[1].id));
    assert!(green(px(ir.center())), "image control: {:?}", px(ir.center()));
    let disabled = rect(&h, &format!("scriptui.{}.{}", win.id, win.root.children[2].id));
    let dim = px(disabled.center());
    assert!(dim[1] < 200 && dim[1] > dim[0] + 40, "disabled normal-image fallback is dimmed: {dim:?}");
    h.state_mut().session.execute("scriptui.close", json!({})).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn label_and_image_clicks_reach_group_listeners_once() {
    let mut h = harness();
    let r = h
        .state_mut()
        .session
        .execute(
            "script.run",
            json!({"code":r#"
      var w=new Window('palette','Group buttons');var g=w.add('group');
      g.add('statictext',undefined,'Add from label',{name:'label'});
      var image=g.add('image',undefined,undefined,{name:'image'});image.preferredSize=[32,32];
      g.addEventListener('click',function(e){app.project.activeItem.layers.addNull().name=e.target.type;},true);
      w.show();"#}),
        )
        .unwrap();
    assert_eq!(r["ok"], true, "{r}");
    h.run_steps(3);
    let win = h.state().session.script_ui.windows[0].id;
    click(&mut h, &format!("scriptui.{win}.label"));
    assert_eq!(layer_names(&h), ["statictext"]);
    click(&mut h, &format!("scriptui.{win}.image"));
    assert_eq!(layer_names(&h), ["image", "statictext"]);
    h.state_mut().session.execute("scriptui.close", json!({})).unwrap();
}

#[test]
fn custom_mouse_down_tabs_and_group_buttons_accept_real_pointer_input() {
    let mut h = harness();
    let r = h
        .state_mut()
        .session
        .execute(
            "script.run",
            json!({"code":r#"
      var w=new Window('palette','Custom mouse controls');
      var tab=w.add('group');
      tab.add('statictext',undefined,'Tab',{name:'tab'});
      tab.addEventListener('mousedown',function(e){
        e.stopPropagation();app.project.activeItem.layers.addNull().name='Tab opened';
      },true);
      var go=w.add('group');go.name='go';go.preferredSize=[140,24];
      go.onClick=function(){app.project.activeItem.layers.addNull().name='Applied once';};
      go.addEventListener('click',function(e){go.onClick();},true);
      go.onDraw=function(){var g=this.graphics;g.rectPath(0,0,140,24);g.fillPath(g.newBrush(g.BrushType.SOLID_COLOR,[0.3,0.3,0.3,1]));};
      var options=w.add('statictext',undefined,'Options',{name:'options'});
      options.addEventListener('click',function(e){
        if(!isFinite(e.screenX)||!isFinite(e.clientX)||e.clientX<=0)throw Error('Missing pointer coordinates');
        var p=new Window('palette','Options opened');p.add('statictext',undefined,'Popup ready');
        p.location=[e.screenX-e.clientX,e.screenY-e.clientY];p.show();
      });
      w.show();
    "#}),
        )
        .unwrap();
    assert_eq!(r["ok"], true, "{r}");
    h.run_steps(3);
    let win = h.state().session.script_ui.windows[0].id;
    click(&mut h, &format!("scriptui.{win}.tab"));
    assert_eq!(layer_names(&h), ["Tab opened"]);
    click(&mut h, &format!("scriptui.{win}.go"));
    assert_eq!(layer_names(&h), ["Applied once", "Tab opened"]);
    click(&mut h, &format!("scriptui.{win}.options"));
    assert!(h.state().session.script_ui.windows.iter().any(|w| w.title == "Options opened" && w.visible));
    for id in h.state().session.script_ui.windows.iter().map(|w| w.id).collect::<Vec<_>>() {
        h.state_mut().session.execute("scriptui.close", json!({"window":id})).unwrap();
    }
}

/// Opt-in local compatibility probe. Supply a licensed local script, a disposable
/// root, and JSON control labels in EC_SCRIPT_PROBE, EC_SCRIPT_SCRATCH,
/// EC_SCRIPT_CLICKS. No third-party source or images are part of this test.
#[test]
#[ignore = "requires a local script and an externally sandboxed disposable profile"]
fn local_script_pointer_probe() {
    use effectcraft_engine::{ScriptRequest, Services};
    use std::{io, path::PathBuf, sync::Arc};
    struct Files(PathBuf);
    impl Services for Files {
        fn read_file(&self, path: &str) -> io::Result<Vec<u8>> {
            std::fs::read(path)
        }
        fn write_file(&self, path: &str, bytes: &[u8]) -> io::Result<()> {
            let target = PathBuf::from(path);
            let resolved = if target.exists() {
                target.canonicalize()?
            } else {
                target
                    .parent()
                    .ok_or_else(|| io::Error::other("no parent"))?
                    .canonicalize()?
                    .join(target.file_name().ok_or_else(|| io::Error::other("no filename"))?)
            };
            if !resolved.starts_with(&self.0) {
                return Err(io::Error::other("outside scratch"));
            }
            std::fs::write(target, bytes)
        }
        fn store_file(&self, path: &str, bytes: &[u8]) -> io::Result<()> {
            self.write_file(path, bytes)
        }
        fn script_folder(&self, name: &str) -> Option<String> {
            Some(self.0.join("folders").join(name).to_string_lossy().into())
        }
    }
    fn find(w: &effectcraft_engine::scriptui::Widget, label: &str) -> Option<u32> {
        if !w.visible {
            return None;
        }
        if w.text == label || w.help_tip == label {
            return Some(w.id);
        }
        w.children.iter().find_map(|c| find(c, label))
    }
    let path = std::env::var("EC_SCRIPT_PROBE").unwrap();
    let root = PathBuf::from(std::env::var("EC_SCRIPT_SCRATCH").unwrap()).canonicalize().unwrap();
    for name in ["home", "current", "temp", "desktop", "myDocuments", "userData", "appData", "commonFiles", "startup", "appPackage"] {
        std::fs::create_dir_all(root.join("folders").join(name)).unwrap();
    }
    let labels: Vec<String> = serde_json::from_str(&std::env::var("EC_SCRIPT_CLICKS").unwrap()).unwrap();
    let mut s = effectcraft_host::session();
    s.execute("comp.new", json!({"name":"Main","width":320,"height":180,"duration":4})).unwrap();
    let builder = Harness::builder().with_size(vec2(1400.0, 900.0)).with_pixels_per_point(1.0);
    let builder = if std::env::var_os("EC_SCRIPT_RENDER").is_some() { builder.wgpu() } else { builder };
    let mut h = builder.build_eframe(|_| EffectcraftApp::new(s));
    h.run_steps(3);
    h.state_mut().session.services = Arc::new(Files(root.clone()));
    h.state_mut().session.prefs.scripting.allow_scripts_write_files = true;
    let fixture = effectcraft_host::script::run_code(
        &mut h.state_mut().session,
        "var c=app.project.activeItem;var l=c.layers.addSolid([0.3,0.5,0.7],'Probe solid',320,180,1);l.selected=true;l.transform.opacity.selected=true;",
        "fixture.jsx",
    );
    assert!(fixture.error.is_none(), "{fixture:?}");
    let source = std::fs::read_to_string(&path).unwrap();
    let outcome = effectcraft_host::script::run(
        &mut h.state_mut().session,
        &ScriptRequest { code: &source, name: &path, console: false, panel_title: Some("Local pointer probe") },
    );
    assert!(outcome.error.is_none(), "{outcome:?}");
    let win = h.state().session.script_ui.windows[0].id;
    h.state_mut().show_panel(PanelKind::ScriptPanel(win));
    h.run_steps(3);
    for label in labels {
        let (window, widget) = h
            .state()
            .session
            .script_ui
            .windows
            .iter()
            .filter(|w| w.visible)
            .find_map(|w| find(&w.root, &label).map(|id| (w.id, id)))
            .unwrap_or_else(|| panic!("missing visible control: {label}"));
        click(&mut h, &format!("scriptui.{window}.{widget}"));
        println!("{}", json!({"clicked":label,"widget":widget,"layers":layer_names(&h)}));
    }
    std::fs::write(
        root.join("pointer-result.json"),
        serde_json::to_vec_pretty(&json!({"ui":h.state().session.script_ui.windows,"layers":layer_names(&h),"project":h.state().session.project})).unwrap(),
    )
    .unwrap();
    if std::env::var_os("EC_SCRIPT_RENDER").is_some() {
        h.render().unwrap().save(root.join("pointer-result.png")).unwrap();
    }
    h.state_mut().session.execute("scriptui.close", json!({"window":win})).unwrap();
}

/// The same original source is used for local AE observation; only our own pixels are saved.
#[test]
fn scriptui_custom_buttons_redraw_during_hover_press_and_release() {
    for (format, code) in
        [("jsx", include_str!("../../script/tests/fixtures/button-states.jsx")), ("jsxbin", include_str!("../../script/tests/fixtures/button-states.jsxbin"))]
    {
        use effectcraft_engine::scriptui::DrawOp;
        let s = effectcraft_host::session();
        let mut h = Harness::builder().with_size(vec2(900.0, 700.0)).with_pixels_per_point(1.0).wgpu().build_eframe(|_| EffectcraftApp::new(s));
        h.run_steps(3);
        let result = h.state_mut().session.execute("script.run", json!({"code":code,"name":format!("button-states.{format}")})).unwrap();
        assert_eq!(result["ok"], true, "{result}");
        h.run_steps(4);
        let win = h.state().session.script_ui.windows[0].id;
        let tabs = &h.state().session.script_ui.windows[0].root.children[4];
        assert!(tabs.children[0].children[0].bounds[1] >= tabs.bounds[1] + 24.0, "tab content overlaps its header");
        let custom = format!("scriptui.{win}.custom");
        let point = rect(&h, &custom).center();
        let color = |h: &Harness<'_, EffectcraftApp>| {
            let w = &h.state().session.script_ui.windows[0].root.children[1];
            match w.draw.first() {
                Some(DrawOp::Fill { color: Some(c), .. }) => *c,
                _ => panic!("custom fill missing: {:?}", w.draw),
            }
        };
        let save = |h: &mut Harness<'_, EffectcraftApp>, name: &str| {
            if let Ok(dir) = std::env::var("EC_SNAPSHOT_DIR") {
                let img = h.render().unwrap();
                img.save(format!("{dir}/scriptui-states-{format}-{name}.png")).unwrap();
            }
        };
        assert_eq!(color(&h), [0.2, 0.35, 0.65, 1.0]);
        let image = h.render().unwrap();
        let px = image.get_pixel(point.x as u32, point.y as u32).0;
        for (actual, expected) in px.into_iter().zip([51u8, 89, 166, 255]) {
            assert!(actual.abs_diff(expected) <= 1, "{format}: {px:?}");
        }
        save(&mut h, "normal");
        h.input_mut().events.push(Event::PointerMoved(point));
        h.run_steps(3);
        assert_eq!(color(&h), [0.25, 0.6, 0.35, 1.0]);
        save(&mut h, "hover");
        h.input_mut().events.push(Event::PointerButton { pos: point, button: egui::PointerButton::Primary, pressed: true, modifiers: Default::default() });
        h.run_steps(3);
        assert_eq!(color(&h), [0.75, 0.25, 0.2, 1.0]);
        save(&mut h, "pressed");
        h.input_mut().events.push(Event::PointerButton { pos: point, button: egui::PointerButton::Primary, pressed: false, modifiers: Default::default() });
        h.run_steps(3);
        assert_eq!(color(&h), [0.25, 0.6, 0.35, 1.0]);
        h.input_mut().events.push(Event::PointerMoved(pos2(2.0, 2.0)));
        h.run_steps(3);
        assert_eq!(color(&h), [0.2, 0.35, 0.65, 1.0]);
        let normal_id = format!("scriptui.{win}.normal");
        let normal_rect = rect(&h, &normal_id);
        let normal_point = normal_rect.center();
        let pixels = |h: &mut Harness<'_, EffectcraftApp>| {
            let image = h.render().unwrap();
            image::imageops::crop_imm(&image, normal_rect.min.x as u32, normal_rect.min.y as u32, normal_rect.width() as u32, normal_rect.height() as u32)
                .to_image()
                .into_raw()
        };
        let normal_pixels = pixels(&mut h);
        h.input_mut().events.push(Event::PointerMoved(normal_point));
        h.run_steps(3);
        let hover_pixels = pixels(&mut h);
        assert_ne!(normal_pixels, hover_pixels, "{format}: native button has no hover feedback");
        h.input_mut().events.push(Event::PointerButton {
            pos: normal_point,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: Default::default(),
        });
        h.run_steps(3);
        assert_ne!(hover_pixels, pixels(&mut h), "{format}: native button has no pressed feedback");
        h.input_mut().events.push(Event::PointerButton {
            pos: normal_point,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: Default::default(),
        });
        h.run_steps(3);
        click(&mut h, &format!("scriptui.{win}.check"));
        assert!(!h.state().session.script_ui.windows[0].root.children[2].children[0].checked);
        let tabs = &h.state().session.script_ui.windows[0].root.children[4];
        let second = tabs.children[1].id;
        let disabled = tabs.children[2].id;
        click(&mut h, &format!("scriptui.{win}.{second}"));
        assert_eq!(h.state().session.script_ui.windows[0].root.children[4].active_tab, 1);
        click(&mut h, &format!("scriptui.{win}.{disabled}"));
        assert_eq!(h.state().session.script_ui.windows[0].root.children[4].active_tab, 1);
        save(&mut h, "selected");
        // Effective disablement includes the parent, including agent activation.
        let err = h.state_mut().session.execute("scriptui.click", json!({"window":win,"widget":"disabled"})).unwrap_err();
        assert!(err.to_string().contains("disabled"));
        h.state_mut().session.execute("scriptui.close", json!({"window":win})).unwrap();
    }
}
