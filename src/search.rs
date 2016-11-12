pub fn search<'a>(text: &'a [u8], pattern: &[u8]) -> Option<&'a [u8]> {
    // frame is the anchor point of the current matching frame. A matching frame
    // is defined by the alignment of the searched text (text) and the search
    // pattern (pattern).
    //
    // The first frame is at pattern.len() - 1 in text, This lines up the
    // beginning of the pattern with the beginning of the text:
    //
    //     text:    enemy ticket fans
    //     pattern: ticket
    //
    // If frame were incremented by 1, the alignment would be:
    //
    //     text:    enemy ticket fans
    //     pattern:  ticket
    let mut frame = pattern.len() - 1;

    // Iterate over all the matching frames. This loop will only shift the
    // matching frame to the right along the searched text, never left.
    while frame < text.len() {
        let mut i = frame;
        let mut j = pattern.len();

        // Compare the pattern to the text in the matching frame starting at the
        // last byte in the frame and iterating backward.
        while j > 0 && text[i] == pattern[j - 1] {
            i -= 1;
            j -= 1;
        }

        // If we reached the end of the pattern, the entire matching frame
        // matched, so we record this match.
        if j == 0 {
            return Some(line_of(text, i + 1, pattern.len()));
        }

        frame += 1;
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
