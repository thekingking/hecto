use std::ops::Range;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Clone, Copy)]
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
struct TextFragment {
    grapheme: String, // 字素
    rendered_width: GraphemeWidth, // 字素在view上占的宽度，有些字符占两格宽，有些占一格宽
    replacement: Option<char>, // 将宽度为0的替换
}

pub struct Line {
    fragments: Vec<TextFragment>,
}

impl Line {
    /// 将String转换为Line
    pub fn from(line_str: &str) -> Self {
        // 将line_str转为字素数组
        let fragments = line_str.graphemes(true).map(|grapheme| {
            let unicode_width = grapheme.width();
            // 根据字素宽度确定渲染宽度
            let rendered_width = match unicode_width {
                0 | 1 => GraphemeWidth::Half,
                _ => GraphemeWidth::Full,
            };
            // 将0宽的字素替换成 .
            let replacement = match unicode_width {
                0 => Some('.'),
                _ => None,
            };

            TextFragment {
                grapheme: grapheme.to_string(),
                rendered_width,
                replacement,
            }
        })
        .collect();
        Self {
            fragments,
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
}

#[test]
fn test_graphemes() {
    let s = "hello, world";
    println!("{}", s.graphemes(true).collect::<Vec<&str>>().get(0..3).unwrap().join(""))
}