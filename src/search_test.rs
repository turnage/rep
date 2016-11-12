#![cfg(test)]

use search;
use std::str::from_utf8;

struct test_case {
    text: &'static str,
    pattern: &'static str,
    lines: Vec<&'static str>,
}

impl test_case {
    fn run(&self) {
        let text_bytes = self.text.as_bytes();
        let pattern_bytes = self.pattern.as_bytes();
        let found_lines = match search::search(text_bytes, pattern_bytes) {
            None => Vec::new(),
            Some(line_bytes) => match from_utf8(line_bytes) {
                Err(_) => Vec::new(),
                Ok(line) => vec![line]
            }
        };

        assert_eq!(&self.lines, &found_lines);
    }
}

#[test]
fn test_search() {
    for case in vec![
        test_case{
            text: "1\n..2..\n3",
            pattern: "2",
            lines: vec!["..2.."],
        },
        test_case{
            text: "1\n..280..\n3",
            pattern: "280",
            lines: vec!["..280.."],
        },
    ].iter() {
        case.run();
    }


}
