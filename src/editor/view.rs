use super::{
    editorcommand::{Direction, EditorCommand}, terminal::{Position, Size, Terminal}
};
use std::cmp;

mod buffer;
mod line;
use buffer::Buffer;
use line::Line;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub grapheme_index: usize, // Line数组中字素下标，当前行第几个字素
    pub line_index: usize, // 行坐标，即在第几行
}

pub struct View {
    buffer: Buffer, // 存放读取文件内容
    needs_redraw: bool, // 是否需要重新渲染
    size: Size, // terminal尺寸
    text_location: Location, // 光标在文本中的位置
    scroll_offset: Position, // 光标在view中相对text的偏移量
}

impl View {
    /// 对自定义EditorCommand进行处理
    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
        }
    }

    /// 将文件内容加载到buffer并重新渲染Terminal
    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    /// terminal大小发生变化，对size进行修改，移动光标，重新渲染view
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_text_location_into_view();
        self.needs_redraw = true;
    }

    // region: Rendering

    /// 渲染整个窗口，如果buffer中有内容，在渲染buffer中内容，否则渲染默认欢迎内容
    pub fn render(&mut self) {
        if !self.needs_redraw {
            return ;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return ;
        }
        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;
        // y轴偏移量
        let top = self.scroll_offset.row;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                // 根据偏移量和Terminal宽度从buffer中截取需要渲染到view中的内容
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row, &line.get_visible_graphemes(left..right));
            } else if current_row == vertical_center && self.buffer.is_empty() {
                // 当buffer为空且当前为正中时渲染欢迎内容
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                // 空白行
                Self::render_line(current_row, "~");
            }
        }
        // 每次渲染完之后关闭重复渲染
        self.needs_redraw = false;
    }

    /// 渲染指定行内容
    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render lines");
    }

    /// 自定义buffer为空时显示内容，显示版本信息
    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_div(2).saturating_sub(1)) / 2;
        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }
    // end region

    // region: Scrolling

    /// 光标垂直移动
    fn scroll_vertically(&mut self, to: usize) {
        let Size { height, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };
        self.needs_redraw = self.needs_redraw || offset_changed;
    }

    /// 光标水平移动
    fn scroll_horizontally(&mut self, to: usize) {
        let Size { width, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.col {

            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };
        self.needs_redraw = self.needs_redraw || offset_changed;
    }

    // 由文本中的坐标定位视图中的坐标
    fn scroll_text_location_into_view(&mut self) {
        let Position { row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }
    // end region

    // region: Location and Position

    /// 获取光标在view中的相对位置位置，即view的显示坐标
    pub fn crate_position(&self) -> Position {
        self.text_location_to_position().saturating_sub(self.scroll_offset)
    }

    /// 将文本中的位置Location 转换为 Position
    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { row, col }
    }
    // end region

    // region: text location movement

    /// 文本中光标位置移动，然后将光标在terminal中进行移动
    #[allow(clippy::arithmetic_side_effects)]
    fn move_text_location(&mut self, direction: &Direction) {
        let Size { height, .. } = self.size;
        match direction {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::Home => self.move_to_start_of_line(),
            Direction::End => self.move_to_end_of_line(),
            Direction::PageUP => self.move_up(height.saturating_sub(1)),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
        }
        self.scroll_text_location_into_view();
    }

    /// crate 向上移动step行
    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    /// crate 向下移动step行
    #[allow(clippy::arithmetic_side_effects)]
    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_line();
        self.snap_to_valid_grapheme();
    }

    /// 向左移动一格
    #[allow(clippy::arithmetic_side_effects)]
    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    /// 向右移动一格
    fn move_right(&mut self) {
        let line_width = self.buffer.lines.get(self.text_location.line_index).map_or(0, Line::grapheme_count);
        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_down(1);
            self.move_to_start_of_line();
        }
    }

    /// 移动至当前行行首
    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    /// 移动至当前行行末
    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_index = self.buffer.lines.get(self.text_location.line_index).map_or(0, Line::grapheme_count);
    }

    /// 保证列坐标是合法的
    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self.buffer.lines.get(self.text_location.line_index).map_or(0, |line| {
            cmp::min(line.grapheme_count(), self.text_location.grapheme_index)
        })
    }

    /// 保证当前行坐标是合法的
    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = cmp::min(self.text_location.line_index, self.buffer.height() - 1);
    }
}

impl Default for View {
    /// 实现view的default，默认初始化View，之后可能会改
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }
}