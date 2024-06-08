use std::{cmp, ops::Range};


pub struct Line {
    pub line: String,
}

impl Line {
    /// 将String转换为Line
    pub fn from(line_str: &str) -> Self {
        Self {
            line: String::from(line_str),
        }
    }

    /// 截取部分Line
    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(range.end, self.line.len());
        self.line.get(start..end).unwrap_or_default().to_string()
    }

    /// line的长度
    pub fn len(&self) -> usize {
        self.line.len()
    }
}