use std::{cmp, ops::Range};
use unicode_segmentation::UnicodeSegmentation;


pub struct Line {
    line: String,
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
        self.line.graphemes(true).collect::<Vec<&str>>().get(start..end).unwrap().join("")
    }

    /// line的长度
    pub fn len(&self) -> usize {
        self.line.len()
    }
}

#[test]
fn test_graphemes() {
    let s = "hello, world";
    println!("{}", s.graphemes(true).collect::<Vec<&str>>().get(0..3).unwrap().join(""))
}