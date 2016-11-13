// search searches text.

use std::cmp;

// Guide contains information to guide the string search derived from
// preprocessing the search pattern.
//
// The search is broken into matching frames. A matching frame is defined by the
// alignment of the searched text (text) and the search pattern (pattern).
//
// The first frame is at pattern.len() - 1 in text, This lines up the beginning
// of the pattern with the beginning of the text:
//
//     frame:        |
//     text:    enemy ticket fans
//     pattern: ticket
//
// If frame were incremented by 1, the alignment would be:
//
//     frame:         |
//     text:    enemy ticket fans
//     pattern:  ticket
//
// To search this way blindly, we would have to check the matching frame for
// every index in the searched text, subtract the first 0..(pattern.len() - 2)
// in which the search pattern couldn't fit.
//
// This guide contains tables used to determine which matching frames we can
// safely skip because we know there are no potential matches in them.
struct Guide {
    // mismatch_skip is a table such that, given any u8 value v which is in
    // the text and does not match its aligned index in the search pattern,
    // mismatch_skip[v] returns the safe distance to skip ahead from the
    // comparison index.
    //
    // There are four cases.
    //
    // Case One: The value exists in the search pattern, behind the current
    // index.
    //
    //     compare:   |
    //     frame:     |
    //     text:    abxlafbc
    //     pattern: xla
    //
    // The search will find a mismatch because 'x' != 'a', but instead of just
    // shifting the matching frame once to the right, we can skip to a matching
    // frame which lines up the 'x' with the next 'x' in the search pattern.
    //
    // mismatch_skip['x'] for this search pattern yields 2, so the next matching
    // frame would be 2 places to the right of the index at which the mismatch
    // occurred:
    //
    //     frame:       |
    //     text:    abxlafbc
    //     pattern:   xla
    //
    // Case Two: The value exists in the search pattern, ahead of the current
    // index.
    //
    //     compare:   |
    //     frame:       |
    //     text:    abaahfbc
    //     pattern:  xlah
    //
    // mismatch_skip['a'] for this search pattern yields 1, but moving the
    // matching frame 1 index ahead of the comparison will move it backward, and
    // could potentially trap the search algorithm in a loop. This case is why
    // we have to check the result of this table against a floor; it is always
    // safe to move the matching frame one index to the right in case of a
    // mismatch.
    //
    // Case Three: The value is not in the search pattern.
    //
    // In this case, mismatch_skip[] yields pattern.len(), because there is
    // no possible match in any matching frame containing the value.
    //
    // Case Four: The value is the last value in the search pattern.
    //
    // In this case, mismatch_skip[] yields pattern.len(), because all matching
    // frames aligning the last character and the mismatched character have
    // already been tested or skipped.
    mismatch_skip: [usize; 256],

    // matched_suffix_skip is a table such that, given any index j in the search
    // pattern, yields the safe distance to skip ahead from the comparison
    // index.
    //
    // This table is useful when the text in the matching frame shares some
    // suffix with the search pattern.
    //
    // There are two cases:
    //
    // Case One: The shared suffix is also the prefix of the search pattern.
    //
    //     compare:    |
    //     frame:         |
    //     text:    lolgabcxabc
    //     pattern: abcxabc
    //
    // matched_suffix_skip[j = 4] for this search pattern yields 7, because
    // 'abc', the shared suffix, is also the prefix, so matching frames in
    // between their alignment cannot be valid.
    //
    // Case Two: The shared suffix is repeated elsewhere in the search pattern.
    //
    //     compare:     |
    //     frame:         |
    //     text:    lolgxbcxabc
    //     pattern: xbcxabc
    //
    // matched_suffix_skip[j = 5] for this search pattern yields 6, because
    // 'bc', the shared suffix, is repeated elsewhere in the search pattern with
    // a different character to its left.
    matched_suffix_skip: Vec<usize>
}

impl Guide {
    fn skip(&self, v: u8, i: usize) -> usize {
        cmp::max(self.mismatch_skip[v as usize], self.matched_suffix_skip[i])
    }

    fn from(pattern: &[u8]) -> Guide {
        Guide{
            mismatch_skip: Guide::build_mismatch_skips(pattern),
            matched_suffix_skip: Guide::build_matched_suffix_skips(pattern),
        }
    }

    fn build_mismatch_skips(pattern: &[u8]) -> [usize; 256] {
        let mut skips: [usize; 256] = [pattern.len(); 256];
        for i in 0..(pattern.len() - 1) {
            skips[pattern[i] as usize] = pattern.len() - 1 - i;
        }
        skips
    }

    fn build_matched_suffix_skips(pattern: &[u8]) -> Vec<usize> {
        let end = pattern.len() - 1;
        let mut skips = vec![0; pattern.len()];

        // next_prefix is the skip to the next instance of the prefix in the
        // search pattern.
        let mut next_prefix = end;
        for i in (0..end).rev() {
            if Guide::is_prefix(pattern, &pattern[i + 1..]) {
                next_prefix = i + 1;
            }
            // Set the skip for a given index to the skip required to align the
            // matching frame with the next instance of the search pattern's
            // prefix.
            skips[i] = next_prefix + end - i
        }

        for i in 0..pattern.len() - 1 {
            let suflen = Guide::suffix_len(pattern, &pattern[1..i+1]);
            if pattern[i - suflen] != pattern[end - suflen] {
                skips[end - suflen] = suflen + end - i;
            }
        }

        skips
    }

    // is_prefix returns whether text begins with prefix.
    fn is_prefix(text: &[u8], prefix: &[u8]) -> bool {
        (0..prefix.len()).all(|i| i < text.len() && text[i] == prefix[i])
    }

    // suffix_len returns the length of the suffix a and b share.
    fn suffix_len(a: &[u8], b: &[u8]) -> usize {
        (0..cmp::min(a.len(), b.len()))
            .take_while(|i| a[a.len() - 1 - i] == b[b.len() - 1 - i])
            .count()
    }
}

pub fn search<'a>(text: &'a [u8], pattern: &[u8]) -> Option<&'a [u8]> {
    let guide = Guide::from(pattern);
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

        frame += guide.skip(text[i], j - 1);
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
