//! German preferences and dialogs through the real frame loop, with an isolated in-memory store.
use std::sync::Arc;

use effectcraft_engine::Session;
use effectcraft_engine::config::MemoryConfig;
use effectcraft_ui_egui::{Dialog, EffectcraftApp, menus};
use egui::{Event, PointerButton, Shape, pos2};
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use serde_json::json;

fn click_at(h: &mut Harness<'_, EffectcraftApp>, at: egui::Pos2) {
    h.input_mut().events.push(Event::PointerMoved(at));
    for pressed in [true, false] {
        h.input_mut().events.push(Event::PointerButton { pos: at, button: PointerButton::Primary, pressed, modifiers: Default::default() });
        h.step();
    }
    h.run_steps(3);
}

fn click(h: &mut Harness<'_, EffectcraftApp>, id: &str) {
    let e = h.state().auto.find(id).unwrap_or_else(|| panic!("missing {id}")).clone();
    click_at(h, pos2(e.rect[0] + e.rect[2] / 2.0, e.rect[1] + e.rect[3] / 2.0));
}

fn invoke(h: &mut Harness<'_, EffectcraftApp>, id: &str, params: serde_json::Value) {
    let ctx = h.ctx.clone();
    menus::invoke(h.state_mut(), &ctx, id, params).unwrap();
    h.run_steps(3);
}

fn texts(h: &Harness<'_, EffectcraftApp>) -> Vec<String> {
    fn collect(shape: &Shape, out: &mut Vec<String>) {
        match shape {
            Shape::Text(t) => out.push(t.galley.job.text.clone()),
            Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, out);
                }
            }
            _ => {}
        }
    }
    let mut out = vec![];
    for shape in &h.output().shapes {
        collect(&shape.shape, &mut out);
    }
    out
}

#[test]
fn language_picker_switches_live_and_reloads_german_in_a_new_app() {
    let store = Arc::new(MemoryConfig::default());
    let mut session = Session { config: Some(store.clone()), ..Default::default() };
    session.execute("prefs.set", json!({"key":"general.language", "value":"en"})).unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1280.0, 800.0)).build_eframe(|_| EffectcraftApp::new(session));
    h.run_steps(3);
    invoke(&mut h, "app.settings", json!({"page":"general"}));
    assert!(texts(&h).iter().any(|s| s == "Language:"));
    click(&mut h, "settings.general.language");
    let at = h.query_by_label("Deutsch").unwrap().rect().center();
    click_at(&mut h, at);
    assert_eq!(h.state().session.prefs.general.language, "de");
    assert!(texts(&h).iter().any(|s| s == "Sprache:"));
    assert!(h.query_by_label("Abbrechen").is_some());
    assert!(h.state().auto.find("settings.general.language").is_some());

    click(&mut h, "settings.general.language");
    let at = h.query_by_label("English").unwrap().rect().center();
    click_at(&mut h, at);
    assert_eq!(h.state().session.prefs.general.language, "en");
    assert!(texts(&h).iter().any(|s| s == "Language:"));
    assert!(h.query_by_label("Cancel").is_some());

    click(&mut h, "settings.general.language");
    let at = h.query_by_label("Deutsch").unwrap().rect().center();
    click_at(&mut h, at);
    click(&mut h, "settings.ok");
    assert_eq!(h.state().dialog, None);
    drop(h);
    let mut restarted = Session { config: Some(store), ..Default::default() };
    restarted.load_settings();
    let mut h = Harness::builder().with_size(egui::vec2(1280.0, 800.0)).build_eframe(|_| EffectcraftApp::new(restarted));
    h.run_steps(3);
    invoke(&mut h, "app.settings", json!({"page":"general"}));
    assert_eq!(h.state().session.prefs.general.language, "de");
    assert!(texts(&h).iter().any(|s| s == "Sprache:"));
}

#[test]
fn german_composition_import_and_animation_forms_keep_ids_and_project_names() {
    let mut s = Session::default();
    s.execute("prefs.set", json!({"key":"general.language", "value":"de"})).unwrap();
    s.execute("comp.new", json!({"name":"File", "width":320,"height":180})).unwrap();
    s.execute("layer.newSolid", json!({"name":"Layer"})).unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1280.0, 800.0)).build_eframe(|_| EffectcraftApp::new(s));
    h.run_steps(3);
    assert!(texts(&h).iter().any(|s| s == "Komposition File"));
    assert!(texts(&h).iter().any(|s| s == "Eigenschaften: Layer"));
    invoke(&mut h, "comp.settings", json!({}));
    assert!(texts(&h).iter().any(|s| s == "Kompositionsname:"));
    assert!(h.state().auto.find("dialog.comp.name").is_some());
    assert!(texts(&h).iter().any(|s| s == "File"));
    h.state_mut().dialog = None;
    // The import command opens its parameter dialog before it reads a file.
    invoke(&mut h, "file.import", json!({"paths":["File.psd"]}));
    assert_eq!(h.state().dialog, Some(Dialog::Form));
    assert!(texts(&h).iter().any(|s| s == "Photoshop-Datei importieren"));
    assert!(texts(&h).iter().any(|s| s == "Importart"));
    assert!(h.state().auto.find("form.field.importAs").is_some());
    h.state_mut().dialog = None;
    invoke(&mut h, "layer.timeStretch", json!({}));
    assert!(h.query_by_label("Abbrechen").is_some());
    assert!(texts(&h).iter().any(|s| s == "Dehnungsfaktor:"));
    let comp_id = h.state().session.active_comp_id().unwrap();
    assert_eq!(h.state().session.project.item(comp_id).unwrap().name, "File");
}

#[test]
fn german_command_palette_searches_the_visible_menu_wording() {
    let mut s = Session::default();
    s.execute("prefs.set", json!({"key":"general.language", "value":"de"})).unwrap();
    s.execute("comp.new", json!({"name":"German palette"})).unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1280.0, 800.0)).build_eframe(|_| EffectcraftApp::new(s));
    h.run_steps(3);
    invoke(&mut h, "app.commandPalette", json!({"query":"Speichern"}));
    assert!(texts(&h).iter().any(|s| s.contains("Datei ▸ Speichern")));
    assert_eq!(h.state().dialog, Some(Dialog::CommandPalette));
}

#[test]
fn german_blend_mode_picker_dispatches_the_original_enum_value() {
    let mut s = Session::default();
    s.execute("prefs.set", json!({"key":"general.language", "value":"de"})).unwrap();
    s.execute("comp.new", json!({"name":"File"})).unwrap();
    let layer = s.execute("layer.newSolid", json!({"name":"Layer"})).unwrap()["layer"].as_u64().unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1500.0, 1000.0)).build_eframe(|_| EffectcraftApp::new(s));
    h.run_steps(3);
    invoke(&mut h, "timeline.switchesModes", json!({}));
    click(&mut h, &format!("timeline.layer.{layer}.mode"));
    let at = h.query_by_label("Multiplizieren").unwrap().rect().center();
    click_at(&mut h, at);
    let comp = h.state().session.active_comp().unwrap();
    assert_eq!(comp.layers[0].blend_mode, effectcraft_engine::color::BlendMode::Multiply);
    assert_eq!(comp.layers[0].name, "Layer");
}

#[test]
fn german_panel_menu_captions_keep_the_original_viewer_action() {
    use effectcraft_ui_egui::dock::PanelKind;

    let mut s = Session::default();
    s.execute("prefs.set", json!({"key":"general.language", "value":"de"})).unwrap();
    s.execute("comp.new", json!({"name":"File"})).unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1280.0, 800.0)).build_eframe(|_| EffectcraftApp::new(s));
    h.run_steps(3);
    // Open the same popup state as the panel header, then exercise its real buttons.
    h.ctx.data_mut(|d| d.insert_temp(egui::Id::new("panel-menu"), (PanelKind::Timeline, pos2(400.0, 100.0))));
    h.run_steps(3);
    for label in ["Shy-Ebenen ausblenden", "Frame-Überblendung aktivieren", "Bewegungsunschärfe aktivieren"] {
        assert!(h.query_by_label(label).is_some(), "missing {label}");
    }
    h.ctx.data_mut(|d| d.insert_temp(egui::Id::new("panel-menu"), (PanelKind::Composition, pos2(400.0, 100.0))));
    h.run_steps(3);
    let before = h.state().ui.viewer.safe_margins;
    let at = h.query_by_label("Titel-/Aktionsschutzbereiche").unwrap().rect().center();
    click_at(&mut h, at);
    assert_eq!(h.state().ui.viewer.safe_margins, !before);
    assert!(h.ctx.data(|d| d.get_temp::<(PanelKind, egui::Pos2)>(egui::Id::new("panel-menu"))).is_none());
}
