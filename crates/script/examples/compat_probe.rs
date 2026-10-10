//! Runs one local compatibility input in a disposable headless session. Execute
//! under an OS sandbox when probing third-party scripts; never uses user settings.
use effectcraft_engine::{ScriptRequest, Services, Session};
use serde_json::json;
use std::{io, path::PathBuf, sync::Arc};
struct ProbeFiles {
    scratch: Option<PathBuf>,
    write_root: Option<PathBuf>,
}
impl Services for ProbeFiles {
    fn read_file(&self, path: &str) -> io::Result<Vec<u8>> {
        std::fs::read(path)
    }
    fn write_file(&self, path: &str, data: &[u8]) -> io::Result<()> {
        let target = PathBuf::from(path);
        let resolved = if target.exists() {
            target.canonicalize()?
        } else {
            target
                .parent()
                .ok_or_else(|| io::Error::other("missing parent"))?
                .canonicalize()?
                .join(target.file_name().ok_or_else(|| io::Error::other("missing filename"))?)
        };
        if self.write_root.as_ref().is_some_and(|root| resolved.starts_with(root)) {
            std::fs::write(target, data)
        } else {
            Err(io::Error::new(io::ErrorKind::PermissionDenied, "compatibility probe only writes inside its scratch folder"))
        }
    }
    fn store_file(&self, path: &str, data: &[u8]) -> io::Result<()> {
        self.write_file(path, data)
    }
    fn script_folder(&self, name: &str) -> Option<String> {
        if name == "appPackage" {
            return self.write_root.as_ref().map(|p| p.join("EffectCraft.app").to_string_lossy().to_string());
        }
        self.scratch.as_ref().map(|p| p.join("folders").join(name).to_string_lossy().to_string())
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).ok_or("usage: compat_probe SCRIPT [--panel] [--fixture] [--scratch DIR]")?;
    if std::fs::metadata(path)?.len() > 16 * 1024 * 1024 {
        return Err("input size limit exceeded".into());
    }
    let source = std::fs::read_to_string(path)?;
    let scratch = args
        .iter()
        .position(|a| a == "--scratch")
        .map(|i| args.get(i + 1).ok_or("missing scratch directory"))
        .transpose()?
        .map(PathBuf::from)
        .map(|p| p.canonicalize())
        .transpose()?;
    let allow_writes = scratch.is_some();
    let write_root = args
        .iter()
        .position(|a| a == "--write-root")
        .map(|i| args.get(i + 1).ok_or("missing write root"))
        .transpose()?
        .map(PathBuf::from)
        .map(|p| p.canonicalize())
        .transpose()?
        .or_else(|| scratch.clone());
    let mut s = Session { services: Arc::new(ProbeFiles { scratch, write_root }), ..Default::default() };
    s.prefs.scripting.allow_scripts_write_files = allow_writes;
    effectcraft_script::install(&mut s);
    if args.iter().any(|s| s == "--fixture") {
        let setup = "var c=app.project.items.addComp('Compatibility Probe',640,360,1,5,24);c.openInViewer();var a=c.layers.addNull();a.name='Null';var b=c.layers.addSolid([0.3,0.5,0.7],'Solid',640,360,1);b.selected=true;";
        let out = effectcraft_script::run_code(&mut s, setup, "fixture.jsx");
        if let Some(e) = out.error {
            return Err(e.message.into());
        }
    }
    let req = ScriptRequest { code: &source, name: path, console: false, panel_title: args.iter().any(|s| s == "--panel").then_some("Compatibility Probe") };
    let outcome = effectcraft_script::run(&mut s, &req);
    let mut actions = vec![];
    for (i, arg) in args.iter().enumerate() {
        if arg == "--click" {
            let widget = args.get(i + 1).ok_or("missing click label")?;
            actions.push(json!({"widget":widget,"result":s.execute("scriptui.click",json!({"widget":widget})).map_err(|e|e.to_string())}));
        }
    }
    println!(
        "{}",
        json!({"outcome":outcome.to_json(),"actions":actions,"windows":s.script_ui.windows.len(),"ui":s.script_ui.windows,"items":s.project.items.len(),"layers":s.project.items.keys().filter_map(|id|s.project.comp(*id)).map(|c|c.layers.len()).sum::<usize>(),"writesAllowed":s.prefs.scripting.allow_scripts_write_files})
    );
    Ok(())
}
