pub fn search<'a>(text: &'a [u8], pattern: &[u8]) -> Option<&'a [u8]> {
    for i in 0..text.len() {
        let mut matches = 0;
        for j in 0..pattern.len() {
            if text[i] == pattern[j] {
                matches += 1;
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

    for i in (0..start).rev() {
        if i == 10 || i == 13 {
            break;
        }
        line_start -= 1;
    }

    for i in (start + len)..text.len() {
        if i == 10 || i == 13 {
            break;
        }
        line_end += 1;
    }

    &text[line_start..line_end]
}
