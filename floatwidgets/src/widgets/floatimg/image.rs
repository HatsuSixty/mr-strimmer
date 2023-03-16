use std::process::exit;

use image::io::Reader as ImageReader;
use image::GenericImageView;

pub fn get_image_dimensions(image: String) -> (u32, u32) {
    let img;
    if let Ok(i) = ImageReader::open(image.clone()) {
        if let Ok(d) = i.decode() {
            img = d;
        } else {
            eprintln!("ERROR: Could not decode image `{}`", image);
            exit(1);
        }
    } else {
        eprintln!("[get_image_dimensions] ERROR: Could not open file `{}`", image);
        return (0, 0);
    }

    img.dimensions()
}
