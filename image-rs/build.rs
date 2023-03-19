use std::fs::rename;
use std::process::Command;

fn main() {
    Command::new("curl")
        .arg("-fLo")
        .arg("stb_image.h")
        .arg("https://raw.githubusercontent.com/nothings/stb/master/stb_image.h")
        .output()
        .unwrap();

    rename("stb_image.h", "stb_image.c").unwrap();

    Command::new("gcc")
        .arg("-DSTB_IMAGE_IMPLEMENTATION")
        .arg("-c")
        .arg("stb_image.c")
        .arg("-o")
        .arg("stb_image.o")
        .output()
        .unwrap();

    println!("cargo:rustc-link-arg=stb_image.o");
    println!("cargo:rustc-link-arg=-lm");
}
