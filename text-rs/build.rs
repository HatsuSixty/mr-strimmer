use std::process::Command;

fn main() {
    Command::new("curl")
        .arg("-fLo")
        .arg("Cantarell.ttf")
        .arg("https://archive.org/download/cantarell-bold/Cantarell-Bold.ttf")
        .output()
        .unwrap();
}
