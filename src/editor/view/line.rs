use std::{cmp, ops::Range};


pub struct Line {
    line: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            line: String::from(line_str),
        }
    }

    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(range.end, self.line.len());
        self.line.get(start..end).unwrap_or_default().to_string()
    }
}