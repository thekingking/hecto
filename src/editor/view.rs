use super::{
    editorcommand::{Direction, EditorCommand}, terminal::{Position, Size, Terminal}
};
use std::cmp;

mod buffer;
mod location;
mod line;
use buffer::Buffer;
use line::Line;
use location::Location;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {
    buffer: Buffer, // 存放读取文件内容
    needs_redraw: bool, // 是否需要重新渲染
    size: Size, // terminal尺寸
    location: Location, // 光标在文本中的位置
    scroll_offset: Location, // 光标在view中相对text的偏移量
}

impl View {
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
        let top = self.scroll_offset.y;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                // 根据偏移量和Terminal宽度从buffer中截取需要渲染到view中的内容
                let left = self.scroll_offset.x;
                let right = self.scroll_offset.x.saturating_add(width);
                Self::render_line(current_row, &line.get(left..right));
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
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

    /// 将文件内容加载到buffer并重新渲染Terminal
    pub fn load(&mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    /// 获取当前光标的相对位置，也即是在terminal中的position
    pub fn get_position(&self) -> Position {
        self.location.subtract(&self.scroll_offset).into()
    }

    /// 对自定义EditorCommand进行处理
    pub fn handle_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Quit => {}
        }
    }

    /// 文本中光标位置移动，然后将光标在terminal中进行移动
    #[allow(clippy::arithmetic_side_effects)]
    fn move_text_location(&mut self, direction: &Direction) {
        let Location { mut x, mut y } = self.location;
        let Size { height, .. } = self.size;
        match direction {
            Direction::Up => {
                y = y.saturating_sub(1);
            },
            Direction::Down => {
                y = cmp::min(y.saturating_add(1), self.buffer.lines.len().saturating_sub(1));
            },
            Direction::Left => {
                if x == 0 && y != 0 {
                    y = y.saturating_sub(1);
                    x = self.buffer.lines.get(y).map_or(0, Line::len).saturating_sub(1);
                } else {
                    x = x.saturating_sub(1);
                }
            },
            Direction::Right => {
                if x == self.buffer.lines.get(y).map_or(0, Line::len).saturating_sub(1) {
                    y = y.saturating_add(1) % self.buffer.lines.len();
                    x = 0;
                } else {
                    x = x.saturating_add(1);
                }
            },
            Direction::Home => {
                x = 0;
            },
            Direction::End => {
                x = self.buffer.lines.get(y).map_or(0, Line::len);
            },
            Direction::PageUP => {
                y = y.saturating_sub(height.saturating_sub(1));
            },
            Direction::PageDown => {
                y = cmp::min(y.saturating_add(height), self.buffer.lines.len().saturating_sub(height)).saturating_sub(1);
            },
        }
        x = self.buffer.lines.get(y).map_or(0, |line| cmp::min(x, line.len().saturating_sub(1)));
        self.location = Location { x, y };
        self.scroll_location_into_view();
    }

    /// view中光标移动，若光标移出view范围，则进行偏移并重新渲染view
    fn scroll_location_into_view(&mut self) {
        let Location { x, y } = self.location;
        let Size { width, height} = self.size;
        let mut offset_changed = false;

        if y < self.scroll_offset.y {
            self.scroll_offset.y = y;
            offset_changed = true;
        } else if y >= self.scroll_offset.y.saturating_add(height) {
            self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
            offset_changed = true;
        }

        if x < self.scroll_offset.x {
            self.scroll_offset.x = x;
            offset_changed = true;
        } else if x >= self.scroll_offset.x.saturating_add(width) {
            self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
            offset_changed = true;
        }
        self.needs_redraw = offset_changed;
    }

    /// terminal大小发生变化，对size进行修改，移动光标，重新渲染view
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.scroll_location_into_view();
        self.needs_redraw = true;
    }
}

impl Default for View {
    /// 实现view的default，默认初始化View，之后可能会改
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            needs_redraw: true,
            size: Terminal::size().unwrap_or_default(),
            location: Location::default(),
            scroll_offset: Location::default(),
        }
    }
}