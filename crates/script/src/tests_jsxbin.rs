use crate::{install, run_code};
use effectcraft_engine::{ScriptRequest, Session};
use serde_json::json;
#[path = "../../jsxbin/tests/fixtures.rs"]
mod fixtures;

fn fixture(name: &str) -> &'static str {
    fixtures::CASES.iter().find(|x| x.0 == name).unwrap().2
}
fn run(code: &str) -> crate::Outcome {
    let mut s = Session::default();
    install(&mut s);
    run_code(&mut s, code, "test.jsx")
}

#[test]
fn compiled_and_source_scripts_have_the_same_results() {
    for (name, source, encoded) in fixtures::CASES {
        if name.contains("block_function")
            || name.contains("e4x")
            || name.contains("xml_")
            || name.contains("legacy_")
            || name.contains("guarded")
            || name.contains("for_each")
            || name.contains("panel_this")
            || name.contains("eval_payload")
            || name.contains("string_keys")
            || name.contains("try_guarded")
        {
            continue;
        }
        let a = run(source);
        let b = run(encoded);
        assert_eq!(a.error.is_some(), b.error.is_some(), "{name}: source {:?}, compiled {:?}", a.error, b.error);
        assert_eq!(a.result, b.result, "{name}: {:?}", b.error);
        assert_eq!(a.output, b.output, "{name}");
    }
}

#[test]
fn compiled_legacy_constructs_execute() {
    for (name, expected) in [
        ("original-fixtures_legacy_for_each", json!(12)),
        ("original-fixtures_legacy_xml_literal", json!("1")),
        ("original-fixtures_legacy_xml_constructor", json!(1)),
        ("original-fixtures_legacy_namespace", json!("urn:rig")),
        ("extended_guarded_chain", json!("two")),
        ("extended_guarded_finally", json!("f3")),
        ("extended_for_each_member", json!(3)),
        ("extended_string_keys", json!("3:7")),
        ("extended_e4x_filter", json!(1)),
        ("extended_e4x_qname", json!("yes")),
        ("extended_e4x_qname_attr", json!("yes")),
        ("extended_e4x_dynamic_attr", json!("1")),
        ("extended_e4x_add", json!(2)),
        ("extended_e4x_typeof", json!("xml")),
        ("source_e4x_list", json!(2)),
        ("source_e4x_namespace", json!("urn:rig")),
        ("regressions_xml_type_variable", json!("xml")),
        ("regressions_xml_plus_assign", json!(2)),
        ("regressions_xml_ns_dynamic", json!("v")),
        ("regressions_block_function", json!(7)),
    ] {
        let out = run(fixture(name));
        assert!(out.error.is_none(), "{name}: {:?}", out.error);
        assert_eq!(out.result, expected, "{name}");
    }
}

#[test]
fn compiled_eval_preserves_direct_scope_and_indirect_global_scope() {
    let encoded = serde_json::to_string(fixture("extended_eval_payload")).unwrap();
    let direct = run(&format!("(function(){{var local=31;var result=eval({encoded});return [local,added,result];}})()"));
    assert!(direct.error.is_none(), "{:?}", direct.error);
    assert_eq!(direct.result, json!([33, 9, 42]));
    let indirect = run(&format!("var local=1; (function(){{var local=30;var e=eval;e({encoded});return [local,$.global.local,$.global.added];}})()"));
    assert!(indirect.error.is_none(), "{:?}", indirect.error);
    assert_eq!(indirect.result, json!([30, 3, 9]));
    assert_eq!(run("eval(42)").result, json!(42));
    let shadow = run("(function(eval){return eval('x');})(function(x){return 'shadow '+x;})");
    assert_eq!(shadow.result, json!("shadow x"));
}

#[test]
fn compiled_panel_binds_this_and_retains_callbacks() {
    let mut s = Session::default();
    install(&mut s);
    let req = ScriptRequest { code: fixture("extended_panel_this"), name: "Test.jsxbin", console: false, panel_title: Some("Compiled") };
    let out = crate::run(&mut s, &req);
    assert!(out.error.is_none(), "{:?}", out.error);
    assert_eq!(s.script_ui.windows.len(), 1);
    let out = s.execute("scriptui.click", json!({"widget":"Compiled Button"})).unwrap();
    assert_eq!(out["ok"], true, "{out}");
    assert!(s.project.items.values().any(|i| i.name == "Compiled Callback"));
    s.execute("scriptui.close", json!({})).unwrap();
}

#[test]
fn dynamic_bad_input_and_nesting_return_errors() {
    let text = "(".repeat(120) + "0" + &")".repeat(120);
    let out = run(&format!("eval({})", serde_json::to_string(&text).unwrap()));
    assert!(out.error.unwrap().message.contains("nested too deeply"));
    let out = run("eval('@JSXBIN@ES@2.0@broken')");
    assert!(out.error.unwrap().message.contains("JSXBIN at byte"));
}

#[test]
fn compiled_eval_file_restores_file_name_and_honors_permissions() {
    let dir = std::env::temp_dir().join(format!("ec-jsxbin-evalfile-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("child.jsxbin");
    std::fs::write(&file, fixture("original-fixtures_arithmetic")).unwrap();
    let path = serde_json::to_string(&file.to_string_lossy()).unwrap();
    let mut s = Session::default();
    install(&mut s);
    let blocked = run_code(&mut s, &format!("$.evalFile({path})"), "parent.jsx");
    assert!(blocked.error.unwrap().message.contains("Unable to access"));
    s.path = Some(dir.join("test.ecproj").to_string_lossy().into());
    let out = run_code(&mut s, &format!("var result=$.evalFile(new File({path}));[result,$.fileName]"), "parent.jsx");
    assert!(out.error.is_none(), "{:?}", out.error);
    assert_eq!(out.result, json!([33, "parent.jsx"]));
    std::fs::write(&file, "@JSXBIN@ES@2.0@broken").unwrap();
    let out = run_code(&mut s, &format!("try{{$.evalFile({path});}}catch(e){{}} $.fileName"), "parent.jsx");
    assert_eq!(out.result, json!("parent.jsx"));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn compiled_inputs_work_in_commands_and_the_persistent_console() {
    let mut s = Session::default();
    install(&mut s);
    let code = fixture("original-fixtures_arithmetic");
    for name in ["script", "Demo.JSXBIN"] {
        let out = s.execute("script.run", json!({"code":code,"name":name})).unwrap();
        assert_eq!(out["result"], 33);
    }
    let code = fixture("extended_eval_payload");
    crate::run(&mut s, &ScriptRequest { code: "var local=31", name: "console", console: true, panel_title: None });
    let out = crate::run(&mut s, &ScriptRequest { code, name: "console", console: true, panel_title: None });
    assert_eq!(out.result, json!(42));
    let out = crate::run(&mut s, &ScriptRequest { code: "local+added", name: "console", console: true, panel_title: None });
    assert_eq!(out.result, json!(42));
    let encoded = serde_json::to_string(fixture("extended_eval_payload")).unwrap();
    let out = run(&format!("var local=1;new Function({encoded})();[local,typeof added]"));
    assert!(out.error.is_none(), "{:?}", out.error);
    // Function's local declarations do not leak into the global environment.
    assert_eq!(out.result, json!([1, "undefined"]));
}

#[test]
fn installed_compiled_scripts_and_panels_follow_normal_lifecycle() {
    use effectcraft_engine::config::{ConfigStore, MemoryConfig};
    let store = std::sync::Arc::new(MemoryConfig::default());
    let mut s = Session { config: Some(store.clone()), ..Default::default() };
    install(&mut s);
    let dir = std::env::temp_dir().join(format!("ec-jsxbin-installed-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let script = dir.join("Arithmetic.jsxbin");
    std::fs::write(&script, fixture("original-fixtures_arithmetic")).unwrap();
    let panel = dir.join("Compiled.jsxbin");
    std::fs::write(&panel, fixture("extended_panel_this")).unwrap();
    s.execute("file.installScript", json!({"path":script})).unwrap();
    s.execute("file.installScriptUIPanel", json!({"path":panel})).unwrap();
    let out = s.execute("file.runScript", json!({"name":"Arithmetic.jsxbin"})).unwrap();
    assert_eq!(out["result"], 33);
    let out = s.execute("file.runScript", json!({"path":script})).unwrap();
    assert_eq!(out["result"], 33);
    s.execute("window.scriptPanel", json!({"name":"Compiled.jsxbin"})).unwrap();
    assert_eq!(s.script_ui.windows.len(), 1);
    let mut restored = Session { config: Some(store.clone()), ..Default::default() };
    install(&mut restored);
    let out = restored.execute("window.restoreScriptPanels", json!({})).unwrap();
    assert_eq!(out["opened"], json!(["Compiled.jsxbin"]));
    assert_eq!(restored.script_ui.windows.len(), 1);
    assert!(store.read("Scripts/Arithmetic.jsxbin").is_some());
    s.execute("scriptui.close", json!({})).unwrap();
    restored.execute("scriptui.close", json!({})).unwrap();
    let wrapped = format!("eval({})", serde_json::to_string(fixture("original-fixtures_arithmetic")).unwrap());
    let out = run_code(&mut s, &wrapped, "Wrapped.jsxbin");
    assert_eq!(out.result, json!(33));
    std::fs::remove_dir_all(dir).unwrap();
}
