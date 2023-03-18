use super::*;

pub fn floatimg(image: String) {
    run_from_bin("image-rs".to_string(), format!("\"{}\"", image));
}

pub fn change_img(image: String) {
    run_from_bin("image-rs".to_string(), format!("--set \"{}\"", image));
}
