#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use anyhow::Result;

pub fn filter(pattern: &str, buffer: &[u8]) -> Result<Vec<(usize, usize)>> {
    let regex = regex::bytes::RegexBuilder::new(pattern)
        .unicode(false)
        .build()?;
    // let re = regex::bytes::Regex::new(pattern).unwrap();
    // let re = regex::Regex::new(pattern).unwrap();

    // @todo: inefficient
    // let buffer: String = buffer.iter().map(|&v| format!("{v:02x}")).collect();

    let mut matches = vec![];
    for regex_match in regex.find_iter(buffer) {
        let start = regex_match.start();
        let end = regex_match.end();
        matches.push((start, end));
    }

    Ok(matches)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {

    use super::*;
    use iter_tools::Itertools;
    use pretty_hex::{HexConfig, PrettyHex};

    #[test]
    fn test_filter_no_unicode() {
        let pattern = r"\x01.{31}\x02.{31}";

        let buffer = "
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            01 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  86 18 45 00
            02 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  87 19 46 01
        ";

        let buffer = buffer
            .split_whitespace()
            .filter(|v| !v.is_empty())
            .map(|v| {
                let temp = u8::from_str_radix(v, 16).unwrap();
                temp
            })
            .collect_vec();

        let cfg = HexConfig {
            title: true,
            ascii: false,
            width: 16,
            group: 4,
            chunk: 1,
            display_offset: 0x00,
            ..HexConfig::default()
        };
        println!("{:?}", buffer.hex_conf(cfg));

        let &(start, stop) = filter(pattern, &buffer).unwrap().first().unwrap();

        assert_eq!(start, 32);
        assert_eq!(stop, 96);
    }
}
