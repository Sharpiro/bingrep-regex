pub fn get_context_range(start: usize, end: usize, context_lines: usize) -> (usize, usize) {
    const LINE_LENGTH: usize = 16;

    if context_lines == 0 {
        return (start, end);
    }

    let context_size = context_lines * LINE_LENGTH;
    let context_start = start.saturating_sub(context_size);
    let context_start = (context_start as f64 / 16.0).floor() as usize * 16;
    let context_end = end.saturating_add(context_size);
    let context_end = (context_end as f64 / 16.0).ceil() as usize * 16 - 1;

    (context_start, context_end)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(clippy::indexing_slicing)]
mod test {
    use super::*;
    use crate::filter::{filter, parse_buffer};

    #[test]
    fn context_0_test() {
        let buffer = parse_buffer(
            "
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            FF 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            ",
        );
        let &(start, end) = filter(r"\xFF", &buffer).unwrap().first().unwrap();
        let (start, end) = get_context_range(start, end, 0);

        dbg!(&buffer[start..end]);

        assert_eq!(start, 16);
        assert_eq!(end, 17);
    }

    #[test]
    fn context_1_test() {
        let buffer = parse_buffer(
            "
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            FF 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            ",
        );
        let &(start, end) = filter(r"\xFF", &buffer).unwrap().first().unwrap();
        let (start, end) = get_context_range(start, end, 1);

        dbg!(&buffer[start..end]);

        assert_eq!(start, 0);
        assert_eq!(end, 47);
    }

    #[test]
    fn context_4_test() {
        let buffer = parse_buffer(
            "
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            FF 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            ",
        );
        let &(start, end) = filter(r"\xFF", &buffer).unwrap().first().unwrap();
        let (start, end) = get_context_range(start, end, 4);

        dbg!(&buffer[start..end]);

        assert_eq!(start, 0);
        assert_eq!(end, 111);
    }

    #[test]
    fn context_2_offset_start_test() {
        let buffer = parse_buffer(
            "
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  FF 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            ",
        );
        let &(start, end) = filter(r"\xFF", &buffer).unwrap().first().unwrap();
        let (start, end) = get_context_range(start, end, 1);

        println!("{:#?}", &buffer[start..end]);

        assert_eq!(start, 16);
        assert_eq!(end, 63);
    }
}
