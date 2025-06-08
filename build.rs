use std::process::Command;

fn main() {
    Command::new("./tailwindcss-windows-x64.exe")
        .arg("--input")
        .arg("input.css")
        .arg("--output")
        .arg("assets/tailwind.css")
        .arg("--minify")
        .output()
        .expect("Failed to execute tailwindcss-windows-x64.exe");
}
