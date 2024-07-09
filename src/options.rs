// @todo: align to 0x10
pub fn get_context_range(start: usize, end: usize, context_lines: usize) -> (usize, usize) {
    const LINE_LENGTH: usize = 16;

    let context_size = context_lines * LINE_LENGTH;
    let display_start = start.saturating_sub(context_size);
    let display_end = end.saturating_add(context_size);

    (display_start, display_end)
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
            01 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            ",
        );
        let &(start, end) = filter("\x01", &buffer).unwrap().first().unwrap();
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
            01 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            ",
        );
        let &(start, end) = filter("\x01", &buffer).unwrap().first().unwrap();
        let (start, end) = get_context_range(start, end, 1);

        dbg!(&buffer[start..end]);

        assert_eq!(start, 0);
        assert_eq!(end, 33);
    }

    #[test]
    fn context_4_test() {
        let buffer = parse_buffer(
            "
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            01 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            ",
        );
        let &(start, end) = filter("\x01", &buffer).unwrap().first().unwrap();
        let (start, end) = get_context_range(start, end, 4);

        dbg!(&buffer[start..end]);

        assert_eq!(start, 0);
        assert_eq!(end, 97);
    }
}
