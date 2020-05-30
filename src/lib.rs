#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    unsafe_code
)]

use std::io::{BufRead, Read, Result, Write};

/// Using a provided reader and writer, will prepend "< " to the start of each
/// line. A newline will be added at the end of the input regardless if one is
/// provided or not.
///
/// # Errors
///
/// Will return `std::io::Err` if any of the `write_all()` methods fail or if
/// `lines()` returns an error.
pub fn add_quotes<T: BufRead + Read, U: Write>(input: T, mut output: U, fast: bool) -> Result<()> {
    if fast {
        quotify_fast(input, &mut output)?;
    } else {
        output.write_all(quotify_slow(input).as_bytes())?;
    }
    output.write_all(b"\n")
}

fn quotify_fast<T: BufRead + Read, U: Write>(input: T, output: &mut U) -> Result<()> {
    for (index, line) in input.lines().enumerate() {
        if index != 0 {
            output.write_all(b"\n")?;
        }

        let line = line?;
        output.write_all(b"> ")?;
        output.write_all(line.as_bytes())?;
    }

    Ok(())
}

fn quotify_slow<T: BufRead + Read>(input: T) -> String {
    let mut result = String::new();

    for (index, line) in input.lines().enumerate() {
        if index != 0 {
            result.push_str("\n");
        }

        if let Ok(line) = line {
            result.push_str("> ");
            result.push_str(&line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    #[test]
    fn adds_quote_for_single_line() {
        assert_eq!(">  hello", super::quotify_slow(Cursor::new(" hello")));
    }

    #[test]
    fn adds_quotes_for_multiple_line() {
        let input = "First line
Second line.

Final
line.";
        let expected = "> First line
> Second line.
> \n> Final
> line.";
        assert_eq!(expected, super::quotify_slow(Cursor::new(input)));
    }

    #[test]
    fn does_not_add_quote_on_final_empty_line() {
        let input = "First line
Second line.

Final
line.
";
        let expected = "> First line
> Second line.
> \n> Final
> line.";
        assert_eq!(expected, super::quotify_slow(Cursor::new(input)));
    }
}
