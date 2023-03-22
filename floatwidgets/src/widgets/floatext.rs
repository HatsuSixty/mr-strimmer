use super::*;

pub fn floatext(font_path: String, text: String, background_image: String) {
    run_from_bin(
        "text-rs".to_string(),
        format!("\"{}\" \"{}\" \"{}\"", font_path, text, background_image),
    );
}
