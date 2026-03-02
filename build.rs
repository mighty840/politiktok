use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=assets/input.css");
    println!("cargo:rerun-if-changed=tailwind.config.js");
    println!("cargo:rerun-if-changed=src/");

    let status = Command::new("bunx")
        .args([
            "@tailwindcss/cli",
            "-i",
            "assets/input.css",
            "-o",
            "assets/tailwind.css",
            "--minify",
        ])
        .status();

    match status {
        Ok(exit) if exit.success() => {
            println!("cargo:warning=Tailwind CSS built successfully");
        }
        Ok(exit) => {
            println!(
                "cargo:warning=Tailwind CSS build exited with status: {}. \
                 Continuing without Tailwind — run `bunx @tailwindcss/cli` manually.",
                exit
            );
        }
        Err(err) => {
            println!(
                "cargo:warning=Could not run bunx for Tailwind CSS: {}. \
                 Continuing without Tailwind — install bun and run `bunx @tailwindcss/cli` manually.",
                err
            );
        }
    }
}
