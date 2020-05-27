mod support;

use std::str;
use support::binary_with_input;

#[test]
fn single_line_of_text() {
    let process = binary_with_input("hello world")
        .wait_with_output()
        .expect("failed to read from stdout");

    assert!(process.status.success());
    assert_eq!("> hello world\n", str::from_utf8(&process.stdout).unwrap());
}

#[test]
fn multiple_lines_of_text() {
    let process = binary_with_input(include_str!("./fixtures/multiline.txt"))
        .wait_with_output()
        .expect("failed to read from stdout");

    assert!(process.status.success());
    assert_eq!(
        include_str!("./fixtures/multiline_quoted.txt"),
        str::from_utf8(&process.stdout).unwrap()
    );
}
