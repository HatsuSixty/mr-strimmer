use super::*;

pub fn floatwebcam() {
    if Path::new("./bin/webcam-rs").is_file() {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg("./bin/webcam-rs")
            .status()
        {
            eprintln!("ERROR: Could not execute `./bin/webcam-rs`: {}", e);
            exit(1);
        }
    } else {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg("webcam-rs")
            .status()
        {
            eprintln!("ERROR: Could not execute `webcam-rs`: {}", e);
            exit(1);
        }
    }
}
