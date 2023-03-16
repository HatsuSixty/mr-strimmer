use std::path::Path;
use std::process::Command;
use std::process::exit;

pub fn floatwebcam() {
    if Path::new("./bin/webcam-rs").is_file() {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg("./bin/webcam-rs")
            .output()
        {
            eprintln!("ERROR: Could not execute `./bin/webcam-rs`: {}", e);
            exit(1);
        }
    } else {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg("webcam-rs")
            .output()
        {
            eprintln!("ERROR: Could not execute `webcam-rs`: {}", e);
            exit(1);
        }
    }
}
