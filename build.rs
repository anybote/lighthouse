use std::process::Command;

fn main() {
    // only rebuild if these files change
    println!("cargo::rerun-if-changed=tailwind.config.js");
    println!("cargo::rerun-if-changed=assets/styles");
    println!("cargo::rerun-if-changed=templates");

    // build tailwind css output file
    let output = Command::new("sh")
        .arg("-c")
        .arg("tailwindcss -i ./assets/styles/input.css -o ./assets/styles/output.css")
        .output()
        .expect("failed to build tailwind css output file")
        .stdout;

    println!("{output:?}");
}
