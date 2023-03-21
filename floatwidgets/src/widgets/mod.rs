use std::path::Path;
use std::process::Command;
use std::process::exit;

pub mod floatimg;
pub mod floatwebcam;
pub mod floatext;

pub fn run_from_bin(exe: String, args: String) {
    if Path::new(format!("./bin/{}", exe).as_str()).is_file() {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg(format!("./bin/{} {}", exe, args).as_str())
            .status()
        {
            eprintln!("ERROR: Could not execute `./bin/{}`: {}", exe, e);
            exit(1);
        }
    } else {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg(format!("{} {}", exe, args).as_str())
            .status()
        {
            eprintln!("ERROR: Could not execute `{}`: {}", exe, e);
            exit(1);
        }
    }
}
