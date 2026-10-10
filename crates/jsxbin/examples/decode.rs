use std::{fs, process::ExitCode};
fn main() -> ExitCode {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: decode file.jsxbin");
        return ExitCode::FAILURE;
    };
    let result = fs::read_to_string(path).map_err(|e| e.to_string()).and_then(|s| effectcraft_jsxbin::decode(&s).map_err(|e| e.to_string()));
    match result {
        Ok(d) => {
            print!("{}", d.source);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
