use std::fs::{OpenOptions, remove_file};
use std::io::Write;
use std::path::Path;
use std::process::exit;

pub fn download_file(url: String, output: String) {
    if Path::new(output.as_str()).is_file() {
        remove_file(output.as_str()).unwrap();
    }

    let mut dest;
    if let Ok(f) = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(output.clone())
    {
        dest = f;
    } else {
        eprintln!("ERROR: Could not open file `{}`", output);
        exit(1);
    }

    let source;
    if let Ok(s) = reqwest::blocking::get(url.as_str()) {
        if let Ok(t) = s.bytes() {
            source = t;
        } else {
            eprintln!(
                "ERROR: Error while downloading `{}`: Could not get text from response",
                output
            );
            exit(1);
        }
    } else {
        eprintln!("ERROR: Could not download file `{}`", output);
        exit(1);
    }

    if let Err(_) = dest.write_all(&source) {
        eprintln!("ERROR: Could not write to file `{}`", output);
        exit(1);
    }
}
