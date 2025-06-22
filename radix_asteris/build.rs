fn main() {
    let _ = std::process::Command::new("tailwindcss.exe")
        .args(["-i", "input.css", "-o", "assets/tailwind.css"])
        .output();
}
