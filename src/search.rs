pub fn search<'a>(text: &'a [u8], pattern: &[u8]) -> Option<&'a [u8]> {
    for i in 0..text.len() {
        let mut matches = 0;
        for j in 0..pattern.len() {
            if text[i + j] == pattern[j] {
                matches += 1;
            } else {
                break;
            }
        }
        if matches == pattern.len() {
            return Some(line_of(text, i, pattern.len()));
        }
    }
    None
}

fn line_of<'a>(text: &'a [u8], start: usize, len: usize) -> &'a [u8] {
    let mut line_start = start;
    let mut line_end = start + len;

    while line_end < text.len() && !is_newline(text[line_end]) {
        line_end += 1;
    }

    while line_start > 0 && !is_newline(text[line_start - 1]) {
        line_start -= 1;
    }

    &text[line_start..line_end]
}

fn is_newline(c: u8) -> bool {
    let newline_ordinal = 10;
    let carriage_return_ordinal = 13;
    c == newline_ordinal || c == carriage_return_ordinal
}
