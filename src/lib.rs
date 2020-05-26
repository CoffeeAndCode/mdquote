#![deny(clippy::all, clippy::cargo, clippy::nursery, clippy::pedantic)]

#[allow(clippy::must_use_candidate)]
pub fn quotify(str: &str) -> String {
    let mut result = String::new();

    for (index, line) in str.lines().enumerate() {
        if index != 0 {
            result.push_str("\n");
        }

        result.push_str("> ");
        result.push_str(line);
    }
    result
}

mod tests {
    #[test]
    fn adds_quote_for_single_line() {
        assert_eq!(">  hello", super::quotify(" hello"));
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
        assert_eq!(expected, super::quotify(input));
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
        assert_eq!(expected, super::quotify(input));
    }
}
