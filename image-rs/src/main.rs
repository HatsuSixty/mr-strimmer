use std::collections::VecDeque;
use std::env::args;
use std::process::exit;

mod download;
mod image;
mod parser;
mod video;

use video::*;

fn usage(myself: String, error: bool) {
    let help = format!(
        r#"USAGE: {} [OPTIONS] <URL>
  URL: The url of the image that is going to be displayed
  OPTIONS:
    --help       Shows this help and exits with 0 exit code
    --set <URL>  Change the currently displaying image to the image of the URL <url>"#,
        myself
    );
    if !error {
        println!("{}", help);
    } else {
        eprintln!("{}", help);
    }
}

fn main() {
    let mut args: VecDeque<String> = args().collect();

    let myself = args.pop_front().unwrap();

    let mut change = false;
    let mut file_path: Option<String> = None;

    while let Some(arg) = args.pop_front() {
        match arg.as_str() {
            "--set" => change = true,
            "--help" => {
                usage(myself, false);
                exit(0);
            }
            _ => file_path = Some(arg),
        }
    }

    if let Some(file) = file_path {
        if !change {
            floatimg(file);
        } else {
            change_img(file);
        }
    } else {
        eprintln!("ERROR: No input file was provided");
        usage(myself, true);
        exit(1);
    }
}
