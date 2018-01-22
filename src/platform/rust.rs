use super::Platform;

use std::fs::metadata;
use std::process::Command;

pub struct Rust {}

impl Platform for Rust {
    fn probe(&self) -> bool {
        metadata("./Cargo.toml")
            .map(|data| data.is_file())
            .unwrap_or(false)
    }

    fn build(&self) -> bool {
        println!("building a Rust project");

        let output = Command::new("cargo").arg("build").output().expect(
            "cargo build failed",
        );

        println!("{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("{}", String::from_utf8_lossy(&output.stderr));

        output.status.success()

    }

    fn run(&self) -> bool {
        println!("running a Rust project");

        let output = Command::new("cargo").arg("run").output().expect(
            "cargo run failed",
        );

        println!("{}", String::from_utf8_lossy(&output.stdout));
        println!("{}", String::from_utf8_lossy(&output.stderr));

        output.status.success()
    }
}
