use std::io::Error;
use std::fs::read_to_string;
use super::line::Line;
use super::Location;

#[derive(Default, Debug)]
pub struct Buffer {
    pub lines: Vec<Line>,
}

impl Buffer {
    /// 将文件内容加载到buffer
    pub fn load(file_name: &str) -> Result<Self, Error>{
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self { lines })
    }

    /// 判断buffer是否为空
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// 文本最大高度
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// 在line中插入字符
    pub fn insert_char(&mut self, character: char, at: Location) {
        // 超出边界范围，直接退出，不予修改
        if at.line_index > self.lines.len() {
            return;
        }
        // 在新一行添加字符
        if at.line_index == self.lines.len() {
            self.lines.push(Line::from(&character.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
        }
    }

    /// 删除当前行指定位置的字符
    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get_mut(at.line_index) {
            line.delete(at.grapheme_index);
        }
    }
}

#[test]
fn test_load() {
    let buffer = Buffer::load("text.txt").unwrap();
    let lines = buffer.lines;
    let line = &lines[6];
    print!("lines: {}", line.get_visible_graphemes(0..153));
    println!("len: {}", lines.len());
}