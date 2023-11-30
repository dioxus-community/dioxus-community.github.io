use std::process::Command;

const INPUT_CSS_PATH: &str = "./src/input.css";
const PUBLIC_DIR: &str = "./public/";

fn main() {
    run_tailwind();
}

fn run_tailwind() {
    #[cfg(windows)]
    let mut command = Command::new("npx.cmd");

    #[cfg(mac)]
    let mut command = Command::new("npx");

    #[cfg(unix)]
    let mut command = Command::new("npx");

    command
        .args([
            "tailwindcss",
            "-i",
            INPUT_CSS_PATH,
            "-o",
            &(PUBLIC_DIR.to_string() + "tailwind.css"),
            "--minify",
        ])
        .spawn()
        .expect("couldn't run tailwind. Please run it manually");
}
