//! Original regressions for runtime gaps found by the local installed-script probes.
use effectcraft_engine::{ScriptRequest, Session};
use serde_json::json;
fn session() -> Session {
    let mut s = Session::default();
    crate::install(&mut s);
    s
}
fn run(s: &mut Session, code: &str) -> crate::Outcome {
    let o = crate::run_code(s, code, "compat.jsx");
    assert!(o.error.is_none(), "{:?}", o.error);
    o
}

#[test]
fn nested_files_and_panel_callbacks_cannot_grant_permissions() {
    let mut s = session();
    let dir = std::env::temp_dir().join(format!("ec-nested-permissions-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    s.path = Some(dir.join("test.ecproj").to_string_lossy().into_owned());
    let file = dir.join("grant.jsx");
    let attempt = "app.run('prefs.set', {key:'scripting.allowScriptsWriteFiles', value:true})";
    std::fs::write(&file, attempt).unwrap();
    let path = serde_json::to_string(&file.to_string_lossy()).unwrap();
    let quoted = serde_json::to_string(attempt).unwrap();
    for code in [format!("$.evalFile({path})"), format!("eval({quoted})"), format!("new Function({quoted})()")] {
        let out = crate::run_code(&mut s, &code, "parent.jsx");
        assert!(out.error.unwrap().message.contains("Scripts can't change what scripts are allowed to do"));
        assert!(!s.prefs.scripting.allow_scripts_write_files);
    }
    let code = format!("var button=this.add('button',undefined,'Permission probe');button.onClick=function(){{{attempt};}};");
    let out = crate::run(&mut s, &ScriptRequest { code: &code, name: "Permission.jsx", console: false, panel_title: Some("Permission") });
    assert!(out.error.is_none(), "{:?}", out.error);
    let out = s.execute("scriptui.click", json!({"widget":"Permission probe"})).unwrap();
    assert_eq!(out["ok"], false, "{out}");
    assert!(out.to_string().contains("Scripts can't change what scripts are allowed to do"));
    assert!(!s.prefs.scripting.allow_scripts_write_files);
    s.execute("scriptui.close", json!({})).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn output_module_collection_is_live_one_based_and_writes_to_the_render_queue() {
    let mut s = session();
    let o = run(
        &mut s,
        r#"
      var c=app.project.items.addComp('Modules',100,100,1,1,24);
      var rq=app.project.renderQueue.items.add(c), modules=rq.outputModules;
      var before=modules.length, second=modules.add();
      second.file=new File('/tmp/ec-second-output.png');
      [modules instanceof OMCollection, [] instanceof OMCollection, before, modules.length,
       modules[0]===undefined, modules[1] instanceof OutputModule, modules[3]===undefined,
       rq.outputModule(2).file.fsName===second.file.fsName]
    "#,
    );
    assert_eq!(o.result, json!([true, false, 1, 2, true, true, true, true]));
}

#[test]
fn file_and_folder_type_tags_preserve_path_coercion() {
    let mut s = session();
    let o = run(
        &mut s,
        r#"var f=new File('/tmp/type-check.ffx'),d=f.parent;
      [Object.prototype.toString.call(f),Object.prototype.toString.call(d),String(f)===f.fsName,
       f instanceof File,d instanceof Folder,Object.keys(f).indexOf('toStringTag')===-1]"#,
    );
    assert_eq!(o.result, json!(["[object File]", "[object Folder]", true, true, true, true]));
}

#[test]
fn callable_layers_and_groups_share_properties_and_preserve_host_prototypes() {
    let mut s = session();
    let o = run(
        &mut s,
        r#"
      var c=app.project.items.addComp('Call shorthand',100,100,1,1,24), l=c.layers.addNull();
      l.name='Renamed';l('ADBE Transform Group')('ADBE Opacity').setValue(37);
      var fx=l('ADBE Effect Parade').addProperty('ADBE Slider Control');
      fx.name='Dial';fx(1).setValue(12);
      [l instanceof AVLayer,l instanceof Layer,l.name,l.transform.opacity.value,
       fx instanceof PropertyGroup,fx instanceof PropertyBase,fx.name,
       l('ADBE Effect Parade')('Dial')(1).value,l('Missing')===null,
       Object.keys(l).indexOf('name')===-1]
    "#,
    );
    assert_eq!(o.result, json!([true, true, "Renamed", 37, true, true, "Dial", 12, true, true]));
}

#[test]
fn native_preset_can_carry_custom_controls_and_feed_expressions() {
    use effectcraft_engine::{
        keyframe::Value,
        project::{GroupKind, PropGroup, Property},
    };
    let mut s = session();
    s.expr = Some(std::sync::Arc::new(effectcraft_expr::Expressions));
    run(&mut s, "var c=app.project.items.addComp('Preset proof',100,100,1,1,24);c.openInViewer();var l=c.layers.addNull();l.selected=true;");
    let mut group = PropGroup::new(1, "Pseudo/Compatibility Probe", "Probe Controls").with(Property::new(
        2,
        "Pseudo/Compatibility Probe-0001",
        "Amount",
        Value::Scalar(24.0),
    ));
    group.kind = GroupKind::Effect { effect: "Pseudo/Compatibility Probe".into() };
    let preset = json!({"effectcraftAnimationPreset":1,"name":"Original control preset","properties":[],"effects":[group]});
    let r = s.execute("anim.applyPreset", json!({"preset":preset})).unwrap();
    assert_eq!(r["applied"], 1);
    let o = run(
        &mut s,
        r#"
      var l=app.project.activeItem.layer(1),fx=l('ADBE Effect Parade')('Probe Controls');
      fx('Amount').setValue(38);
      l.transform.opacity.expression='effect("Probe Controls")("Amount") * 2';
      [fx.isEffect,fx.matchName,fx(1).value,l.transform.opacity.value,
       l('ADBE Effect Parade').canAddProperty('Pseudo/Compatibility Probe')]
    "#,
    );
    // Preset instances work; registering a reusable pseudo-effect is a separate gap.
    assert_eq!(o.result, json!([true, "Pseudo/Compatibility Probe", 38, 76, false]));
}

#[test]
fn custom_group_mouse_sequence_does_not_invoke_its_callback_twice() {
    let mut s = session();
    run(
        &mut s,
        r#"
      var w=new Window('palette','Mouse sequence');var g=w.add('group');g.name='custom';
      g.onClick=function(){writeLn('action');};
      ['mousedown','mouseup','click'].forEach(function(type){
        g.addEventListener(type,function(e){writeLn(e.type);if(e.type==='click')g.onClick();},true);
      });
      g.add('statictext',undefined,'Label');w.show();
    "#,
    );
    for name in ["custom", "Label"] {
        let o = s.execute("scriptui.click", json!({"widget":name})).unwrap();
        assert_eq!(o["ok"], true, "{o}");
        assert_eq!(o["output"].as_str().unwrap().trim(), "mousedown\nmouseup\nclick\naction");
    }
    s.execute("scriptui.close", json!({})).unwrap();
}

#[test]
fn pointer_metadata_reaches_listeners_and_invalid_coordinates_are_rejected() {
    let mut s = session();
    run(
        &mut s,
        r#"var w=new Window('palette','Pointer data');
      var label=w.add('statictext',undefined,'Options');
      label.addEventListener('click',function(e){writeLn(JSON.stringify([e.screenX,e.screenY,e.clientX,e.clientY,e.altKey]));});w.show();"#,
    );
    let r = s.execute("scriptui.click", json!({"widget":"Options","mouse":{"screenX":140,"screenY":250,"clientX":10,"clientY":5,"altKey":true}})).unwrap();
    assert_eq!(r["output"].as_str().unwrap().trim(), "[140,250,10,5,true]");
    assert!(s.execute("scriptui.click", json!({"widget":"Options","mouse":{"screenX":"bad"}})).is_err());
    s.execute("scriptui.close", json!({})).unwrap();
}

#[test]
fn localization_and_legacy_strings() {
    let mut s = session();
    let o = run(
        &mut s,
        r#"$.locale='de_DE'; var obj={en:'English',de:'Deutsch %2 %1'}; var unknown={fi:'Suomi'};
        [localize(obj,'one','two'),localize({en:'fallback'}),localize(unknown)===unknown,
         localize('$$$/Example=Fallback=%1','value'),localize('%1 %2','%2'),
         'abcdef'.substr(-3,2),unescape(escape('å'))]"#,
    );
    assert_eq!(o.result, json!(["Deutsch two one", "fallback", true, "Fallback=value", "%2 %2", "de", "å"]));
}
#[test]
fn short_transform_lookup_changes_the_same_property_and_undoes() {
    let mut s = session();
    let o = run(
        &mut s,
        r#"var c=app.project.items.addComp('C',100,100,1,1,24);var l=c.layers.addNull();
        app.beginUndoGroup('Move');l.property('Position').setValue([12,34,0]);app.endUndoGroup();
        [l.property('ADBE Position').value,l.property(1) instanceof PropertyGroup,l.property('Missing')===null]"#,
    );
    assert_eq!(o.result, json!([[12, 34, 0], true, true]));
    assert!(s.undo());
    assert_ne!(run(&mut s, "app.project.item(1).layer(1).transform.position.value").result, json!([12, 34, 0]));
}
#[test]
fn panel_globals_direct_eval_and_helpers_share_state() {
    let mut s = session();
    let dir = std::env::temp_dir().join(format!("ec-panel-scope-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join("helper.jsx");
    std::fs::write(&file, "shared+=1; var helperValue=8;").unwrap();
    s.path = Some(dir.join("test.ecproj").to_string_lossy().to_string());
    let code = format!(
        r#"var shared=40;var panel=this;$.evalFile({});function receiver(){{return this;}}
        this.add('button',undefined,'Create from shared state').onClick=function(){{app.project.items.addComp('Count '+shared,100,100,1,1,24);}};
        [shared,$.global.shared,$.global.helperValue,this instanceof Panel,eval('this')===panel,(0,eval)('this')===$.global,receiver()===$.global]"#,
        json!(file)
    );
    let out = crate::run(&mut s, &ScriptRequest { code: &code, name: "Parent.jsx", console: false, panel_title: Some("Parent") });
    assert!(out.error.is_none(), "{:?}", out.error);
    assert_eq!(out.result, json!([41, 41, 8, true, true, true, true]));
    let click = s.execute("scriptui.click", json!({"widget":"Create from shared state"})).unwrap();
    assert_eq!(click["ok"], true);
    assert!(s.project.items.values().any(|i| i.name == "Count 41"));
    s.execute("scriptui.close", json!({})).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}
#[test]
fn scriptui_tags_and_custom_items_do_not_break_serialization() {
    let mut s = session();
    let o = run(
        &mut s,
        r#"var w=new Window('palette','Tags');var g=w.add('group');g.items=['user data'];
        w.show();[Object.prototype.toString.call(g),Object.prototype.toString.call(w),g instanceof Group]"#,
    );
    assert_eq!(o.result, json!(["[object Group]", "[object Window]", true]));
    assert_eq!(s.script_ui.windows.len(), 1);
    s.execute("scriptui.close", json!({})).unwrap();
}
#[test]
fn actual_scripting_permission_and_ae_command_ids() {
    let mut s = session();
    let expr = "app.preferences.getPrefAsLong('Main Pref Section v2','Pref_SCRIPTING_FILE_NETWORK_SECURITY')";
    assert_eq!(run(&mut s, expr).result, json!(0));
    s.execute("prefs.set", json!({"key":"scripting.allowScriptsWriteFiles","value":true})).unwrap();
    assert_eq!(run(&mut s, expr).result, json!(1));
    assert_eq!(run(&mut s, "app.findMenuCommandId('New Project')").result, json!(2));
    run(&mut s, "app.project.items.addComp('C',100,100,1,1,24);app.executeCommand(2)");
    assert!(s.project.items.is_empty());
    assert!(crate::run_code(&mut s, "app.executeCommand(999)", "unknown.jsx").error.is_some());
    run(&mut s, "app.executeCommand(3131)");
}
#[test]
fn writable_binary_files_preserve_bytes_and_relative_paths() {
    struct Folders(std::path::PathBuf);
    impl effectcraft_engine::Services for Folders {
        fn read_file(&self, p: &str) -> std::io::Result<Vec<u8>> {
            std::fs::read(p)
        }
        fn write_file(&self, p: &str, b: &[u8]) -> std::io::Result<()> {
            std::fs::write(p, b)
        }
        fn script_folder(&self, _: &str) -> Option<String> {
            Some(self.0.to_string_lossy().to_string())
        }
    }
    let dir = std::env::temp_dir().join(format!("ec-binary-file-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let mut s = session();
    s.services = std::sync::Arc::new(Folders(dir.clone()));
    s.prefs.scripting.allow_scripts_write_files = true;
    let o = run(
        &mut s,
        r#"var f=new File('~/image.bin');f.encoding='BINARY';f.open('w');f.write(String.fromCharCode(0,127,128,255));f.close();
        f.open('r');var x=f.read();f.close();[x.charCodeAt(0),x.charCodeAt(2),x.charCodeAt(3),new File('image.bin').fsName===f.fsName,Folder.userData instanceof Folder,Folder.temp instanceof Folder]"#,
    );
    assert_eq!(o.result, json!([0, 128, 255, true, true, true]));
    assert_eq!(std::fs::read(dir.join("image.bin")).unwrap(), [0, 127, 128, 255]);
    s.prefs.scripting.allow_scripts_write_files = false;
    assert!(crate::run_code(&mut s, "new File('blocked').open('w')", "blocked.jsx").error.is_some());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn source_serialization_roundtrips_values_and_rejects_cycles() {
    let mut s = session();
    let o = run(
        &mut s,
        r#"var sparse=[];sparse.length=3;sparse[1]='middle';
      var a={u:undefined,n:NaN,z:-0,a:sparse,d:new Date(1234),r:/ab+/gi,f:function(x){return x+1;}};
      var b=eval(a.toSource());var cyclic={};cyclic.self=cyclic;var cycle=false;
      try{cyclic.toSource();}catch(e){cycle=e instanceof TypeError;}
      [b.u===undefined,isNaN(b.n),Object.is(b.z,-0),b.a.length,0 in b.a,2 in b.a,b.a[1],
       b.d.getTime(),b.r.source,b.r.global,b.f(2),cycle,Object.keys({}).indexOf('toSource')===-1]"#,
    );
    assert_eq!(o.result, json!([true, true, true, 3, false, false, "middle", 1234, "ab+", true, 3, true, true]));
}

#[test]
fn dynamic_xml_attribute_expressions_support_string_methods() {
    let mut s = session();
    let o = run(
        &mut s,
        r#"var xmlObj=new XML('<entry name="id=example"/>');
       [String(eval('xmlObj.@name')),eval('xmlObj.@name.split("=").pop()')]"#,
    );
    assert_eq!(o.result, json!(["id=example", "example"]));
}

#[test]
fn preferences_persist_across_contexts_without_overriding_permission() {
    let mut s = session();
    run(
        &mut s,
        r#"app.preferences.savePrefAsFloat('Example','gamma',1.25);
       app.preferences.savePrefAsLong('Example','count',3);app.preferences.saveToDisk();"#,
    );
    let o = run(&mut s, "[app.preferences.getPrefAsFloat('Example','gamma'),app.preferences.getPrefAsLong('Example','count')]");
    assert_eq!(o.result, json!([1.25, 3]));
    assert!(
        crate::run_code(&mut s, "app.preferences.savePrefAsLong('Main Pref Section v2','Pref_SCRIPTING_FILE_NETWORK_SECURITY',1)", "permission.jsx")
            .error
            .is_some()
    );
    assert!(!s.prefs.scripting.allow_scripts_write_files);
}

#[test]
fn scriptui_clicks_bubble_from_labels_with_capture_and_cancellation() {
    let mut s = session();
    let o = run(
        &mut s,
        r#"var events=[];var w=new Window('palette','Events');var g=w.add('group');var label=g.add('statictext',undefined,'Navigate');
      w.addEventListener('click',function(e){events.push('capture:'+e.eventPhase);},true);
      g.addEventListener('click',function(e){app.project.items.addComp('Clicked '+e.target.text,100,100,1,1,24);});
      label.addEventListener('click',function(e){events.push('target:'+e.eventPhase);});w.show();"#,
    );
    assert!(o.error.is_none());
    s.execute("scriptui.click", json!({"widget":"Navigate"})).unwrap();
    assert!(s.project.items.values().any(|i| i.name == "Clicked Navigate"));
    // Direct dispatch also verifies phase ordering and cancellation.
    s.execute("scriptui.close", json!({})).unwrap();
    let o = run(
        &mut s,
        r#"var calls=[];var w=new Window('palette','Events');var g=w.add('group');var l=g.add('statictext',undefined,'X');
      w.addEventListener('click',function(e){calls.push('capture:'+e.eventPhase);},true);
      g.addEventListener('click',function(e){calls.push('group:'+e.target.text+':'+e.eventPhase);});
      l.addEventListener('click',function(e){calls.push('target:'+e.eventPhase);});
      var e=ScriptUI.events.createEvent('MouseEvent');e.initEvent('click',true,true);l.dispatchEvent(e);
      var result=calls.slice();calls=[];l.addEventListener('click',function(e){e.stopPropagation();e.preventDefault();});
      var allowed=l.dispatchEvent(e);[result,calls,allowed]"#,
    );
    assert_eq!(o.result, json!([["capture:1", "target:2", "group:X:3"], ["capture:1", "target:2"], false]));
}

#[test]
fn composition_viewer_tracks_and_activates_real_compositions() {
    let mut s = session();
    let o = run(
        &mut s,
        r#"var initially=app.activeViewer===null;
      var a=app.project.items.addComp('A',100,100,1,1,24);var v=a.openInViewer();
      var b=app.project.items.addComp('B',100,100,1,1,24);b.openInViewer();var inactive=!v.active;
      var activated=v.setActive();
      [initially,v instanceof Viewer,v.type===ViewerType.VIEWER_COMPOSITION,inactive,activated,
       app.project.activeItem.name,new Error('reason').description,app.language===Language.ENGLISH]"#,
    );
    assert_eq!(o.result, json!([true, true, true, true, true, "A", "reason", true]));
}

#[test]
fn installed_panel_resolves_its_file_name_and_sibling_helpers() {
    let dir = std::env::temp_dir().join(format!("ec-panel-files-{}", std::process::id()));
    let panels = dir.join("Scripts/ScriptUI Panels");
    std::fs::create_dir_all(&panels).unwrap();
    let parent = panels.join("Parent.jsx");
    std::fs::write(&parent, "$.evalFile('helper.jsx');[sharedFromHelper,$.fileName]").unwrap();
    std::fs::write(panels.join("helper.jsx"), "var sharedFromHelper=17;").unwrap();
    let mut s = session();
    s.config = Some(std::sync::Arc::new(effectcraft_engine::config::DirConfig::new(dir.clone())));
    s.prefs.scripting.allow_scripts_write_files = true;
    let result = s.execute("window.scriptPanel", json!({"name":"Parent.jsx"})).unwrap();
    assert_eq!(result["result"]["result"], json!([17, parent.to_string_lossy()]));
    assert_eq!(s.script_ui.windows[0].script, "Parent.jsx");
    s.execute("scriptui.close", json!({})).unwrap();
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn scriptui_state_images_draw_state_and_toggle_buttons() {
    let mut s = session();
    run(
        &mut s,
        r#"
      var w=new Window('palette','States');
      var icons=ScriptUI.newImage('normal.png','disabled.png','pressed.png','hover.png');
      var b=w.add('iconbutton',undefined,icons,{name:'toggle',toggle:true});
      var g=w.add('group');g.enabled=false;
      g.add('iconbutton',undefined,icons,{name:'disabled'});
      var fallback=w.add('iconbutton',undefined,'normal.png');fallback.enabled=false;
      b.onDraw=function(state){this.graphics.drawOSControl();this.graphics.drawImage(icons,0,0,12,12);};
      w.show();
    "#,
    );
    let state = |s: &Session| s.script_ui.windows[0].root.children[0].clone();
    assert_eq!(state(&s).image.unwrap().src.as_deref(), Some("normal.png"));
    assert!(!s.script_ui.windows[0].root.children[1].children[0].enabled);
    assert!(s.script_ui.windows[0].root.children[2].image.as_ref().unwrap().dimmed);
    assert!(!s.script_ui.windows[0].root.children[1].children[0].image.as_ref().unwrap().dimmed);
    assert_eq!(s.script_ui.windows[0].root.children[1].children[0].image.as_ref().unwrap().src.as_deref(), Some("disabled.png"));
    for (event, hovered, pressed, expected) in [
        ("mouseover", true, false, "hover.png"),
        ("mousedown", true, true, "pressed.png"),
        ("mouseup", true, false, "hover.png"),
        ("mouseout", false, false, "normal.png"),
    ] {
        s.execute("scriptui.pointer", json!({"widget":"toggle","event":event,"hovered":hovered,"pressed":pressed,"focused":true})).unwrap();
        let b = state(&s);
        assert_eq!(b.image.unwrap().src.as_deref(), Some(expected));
        assert!(matches!(&b.draw[1],effectcraft_engine::scriptui::DrawOp::Image {image:Some(i),..} if i.src.as_deref()==Some(expected)));
    }
    s.execute("scriptui.click", json!({"widget":"toggle"})).unwrap();
    assert!(state(&s).checked);
    s.execute("scriptui.click", json!({"widget":"toggle"})).unwrap();
    assert!(!state(&s).checked);
    assert!(s.execute("scriptui.pointer", json!({"widget":"toggle","event":"bad","hovered":true,"pressed":false,"focused":false})).is_err());
    s.execute("scriptui.close", json!({})).unwrap();
}
