use std::fs::canonicalize;
use std::io::Write;
use std::process::{Child, Command, Stdio};

pub fn binary_with_input(stdin: &str) -> Child {
    let binary_name = env!("CARGO_PKG_NAME");
    let command_path =
        canonicalize(format!("./target/debug/{}", binary_name)).expect("binary not found");
    let mut command = Command::new(command_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    command
        .stdin
        .as_mut()
        .expect("failed to open stdin")
        .write_all(stdin.as_bytes())
        .expect("failed to write to stdin");
    command
}
