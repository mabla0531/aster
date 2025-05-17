use std::process::Command;

fn main() {
    Command::new("/home/matthew/dev/aster/tailwindcss-linux-x64")
        .arg("--input")
        .arg("input.css")
        .arg("--output")
        .arg("assets/tailwind.css")
        .arg("--minify")
        .output()
        .expect("Failed to execute tailwindcss-linux-x64");
}