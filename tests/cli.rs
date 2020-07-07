mod support;

use std::str;
use support::{binary, binary_with_input};

#[test]
fn single_line_of_text() {
    let process = binary_with_input(b"hello world")
        .wait_with_output()
        .expect("failed to read from stdout");

    assert!(process.status.success());
    assert_eq!("> hello world\n", str::from_utf8(&process.stdout).unwrap());
}

#[test]
fn multiple_lines_of_text() {
    let process = binary_with_input(include_bytes!("./fixtures/multiline.txt"))
        .wait_with_output()
        .expect("failed to read from stdout");

    assert!(process.status.success());
    assert_eq!(
        include_str!("./fixtures/multiline_quoted.txt"),
        str::from_utf8(&process.stdout).unwrap()
    );
}

#[test]
fn help_long_arg() {
    let process = binary()
        .arg("--help")
        .output()
        .expect("failed to execute process");

    assert!(process.status.success());
    assert!(str::from_utf8(&process.stdout)
        .unwrap()
        .contains("Add Markdown quotes to the start of each line."));
}

#[test]
fn version_short_arg() {
    let process = binary()
        .arg("-V")
        .output()
        .expect("failed to execute process");

    assert!(process.status.success());
    assert!(str::from_utf8(&process.stdout)
        .unwrap()
        .contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn version_long_arg() {
    let process = binary()
        .arg("--version")
        .output()
        .expect("failed to execute process");

    assert!(process.status.success());
    assert!(str::from_utf8(&process.stdout)
        .unwrap()
        .contains(env!("CARGO_PKG_VERSION")));
}
