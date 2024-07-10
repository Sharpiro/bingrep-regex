use iter_tools::Itertools;

pub fn parse_binary_syntax(pattern: &str) -> String {
    let mut last_hex_char: Option<char> = None;
    let mut new_chars = vec![];
    for c in pattern.chars() {
        if c == ' ' {
            continue;
        }

        if !c.is_ascii_hexdigit() {
            if let Some(last_hex_char) = last_hex_char.take() {
                new_chars.push(last_hex_char);
            }
            new_chars.push(c);
            continue;
        };

        if let Some(last_hex_char) = last_hex_char.take() {
            new_chars.push('\\');
            new_chars.push('x');
            new_chars.push(last_hex_char);
            new_chars.push(c);
            continue;
        }

        last_hex_char = Some(c);
    }

    if let Some(last_hex_char) = last_hex_char {
        new_chars.push(last_hex_char);
    }

    let binary_pattern = new_chars.iter().join("");
    binary_pattern
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spaced_test() {
        let pattern = "ff 26 15 d3";

        let raw = parse_binary_syntax(pattern);

        assert_eq!(raw, r"\xff\x26\x15\xd3");
    }

    #[test]
    fn dot_test() {
        let pattern = "53 44 .";

        let raw = parse_binary_syntax(pattern);

        assert_eq!(raw, r"\x53\x44.");
    }

    #[test]
    fn xxd_test() {
        let pattern = "ff26 15d3 a91b 53e5";

        let raw = parse_binary_syntax(pattern);

        assert_eq!(raw, r"\xff\x26\x15\xd3\xa9\x1b\x53\xe5");
    }

    #[test]
    fn not_xxd_test() {
        let pattern = "ff26 1Zd3 a91b 53e5";

        let raw = parse_binary_syntax(pattern);

        assert_eq!(raw, r"\xff\x261Z\xd3\xa9\x1b\x53\xe5");
    }

    #[test]
    fn last_char_hex_test() {
        let pattern = "ab5";

        let raw = parse_binary_syntax(pattern);

        assert_eq!(raw, r"\xab5");
    }

    #[test]
    fn last_char_test() {
        let pattern = "az5";

        let raw = parse_binary_syntax(pattern);

        assert_eq!(raw, r"az5");
    }

    #[test]
    fn or_test() {
        let pattern = "dead be [ef|ed]";

        let raw = parse_binary_syntax(pattern);

        assert_eq!(raw, r"\xde\xad\xbe[\xef|\xed]");
    }
}
