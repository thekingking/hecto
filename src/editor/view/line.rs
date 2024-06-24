use std::{fmt, ops::Range};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Copy, Debug)]
enum GraphemeWidth {
    Half,
    Full,
}

impl GraphemeWidth {
    /// 根据view显示宽度进行增加
    const fn saturating_add(self, other: usize) -> usize{
        match self {
            Self::Half => other.saturating_add(1),
            Self::Full => other.saturating_add(2),
        }
    }
}

/// 文本显示的unicode
#[derive(Debug)]
struct TextFragment {
    grapheme: String, // 字素
    rendered_width: GraphemeWidth, // 字素在view上占的宽度，有些字符占两格宽，有些占一格宽
    replacement: Option<char>, // 将宽度为0的替换
}

#[derive(Debug)]
pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    /// 将String转换为Line
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Self {
            fragments,
        }
    }

    /// 将字符串转换为 Vec<TextFragment>
    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        // 将line_str转为字素数组
        line_str
            .graphemes(true)
            .map(|grapheme| {
                let (replacement, rendered_width) = Self::replacement_character(grapheme)
                .map_or_else(
                    || {
                        // unicode_width提供的grapheme宽度函数
                        let unicode_width = grapheme.width();
                        let rendered_width = match unicode_width {
                            0 | 1 => GraphemeWidth::Half,
                            _ => GraphemeWidth::Full,
                        };
                        (None, rendered_width)
                    }, 
                    |replacement| (Some(replacement), GraphemeWidth::Half)
                );
                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            }).collect()
    }

    /// 将特殊字符进行替换
    fn replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('_'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                Some('.')
            },
            _ => None,
        }
    }

    /// 获取显示在view上的字素
    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new();
        }
        let mut result = String::new();
        let mut current_pos = 0;
        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(current_pos);
            if current_pos >= range.end {
                break;
            }
            if fragment_end > range.start {
                // 边缘字素显示处理，full字素不能完全显示在视图上（占两格宽，只能显示一半），用~替换
                if fragment_end > range.end || current_pos < range.start {
                    result.push('~');
                } else if let Some(ch) = fragment.replacement {
                    result.push(ch);
                } else {
                    result.push_str(&fragment.grapheme);
                }
            }
            current_pos = fragment_end;
        }
        result
    }

    /// 获取Line中字素个数
    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    /// 获取line中起始位置到当前字素的宽度
    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments.iter().take(grapheme_index).map(|grapheme| {
            match grapheme.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            }
        })
        .sum()
    }

    /// 在line指定位置中插入字符
    pub fn insert_char(&mut self, character: char, grapheme_index: usize) {
        let mut result = String::new();

        for (index, fragment) in self.fragments.iter().enumerate() {
            if index == grapheme_index {
                result.push(character);
            }
            result.push_str(&fragment.grapheme);
        }
        if grapheme_index >= self.fragments.len() {
            result.push(character);
        }
        self.fragments = Self::str_to_fragments(&result);
    }

    /// 删除line中指定位置的字符
    pub fn delete(&mut self, grapheme_index: usize) {
        self.fragments.remove(grapheme_index);
    }

    /// 将另一个line添加当当前line后
    /// 先将两个line转为字符串，再进行合并，然后重新转换为line
    /// （不太理解为什么这样写，直接重用之前的不就行了嘛）
    pub fn append(&mut self, other: &Self) {
        let mut concat = self.to_string();
        concat.push_str(&other.to_string());
        self.fragments = Self::str_to_fragments(&concat);
    }
}

impl fmt::Display for Line {
    /// 当前行内容，将所有字素拼接成当前行并返回
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result: String = self
            .fragments
            .iter()
            .map(|fragment| fragment.grapheme.clone())
            .collect();
        write!(f, "{result}")
    }
}

#[test]
fn test_graphemes() {
    println!("{:?}", Line::from("Control characters:[Escape][Bell]"));
}