use super::*;

pub fn floatimg(image: String) {
    if Path::new("./bin/image-rs").is_file() {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg(format!("./bin/image-rs \"{}\"", image).as_str())
            .status()
        {
            eprintln!("ERROR: Could not execute `./bin/image-rs`: {}", e);
            exit(1);
        }
    } else {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg(format!("image-rs \"{}\"", image).as_str())
            .status()
        {
            eprintln!("ERROR: Could not execute `image-rs`: {}", e);
            exit(1);
        }
    }
}

pub fn change_img(image: String) {
    if Path::new("./bin/image-rs").is_file() {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg(format!("./bin/image-rs --set \"{}\"", image).as_str())
            .status()
        {
            eprintln!("ERROR: Could not execute `./bin/image-rs`: {}", e);
            exit(1);
        }
    } else {
        if let Err(e) = Command::new("sh")
            .arg("-c")
            .arg(format!("image-rs --set \"{}\"", image).as_str())
            .status()
        {
            eprintln!("ERROR: Could not execute `image-rs`: {}", e);
            exit(1);
        }
    }
}
