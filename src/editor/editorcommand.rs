use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::convert::TryFrom;

use super::terminal::Size;

pub enum Direction {
    PageUP,
    PageDown,
    Home,
    End,
    Up,
    Down,
    Left,
    Right,
}

/// 自定义Editor命令
pub enum EditorCommand {
    Move(Direction),    // 移动
    Resize(Size),       // 窗口大小发生变化
    Quit,               // 退出
    Insert(char),         // 按键输入内容
}

#[allow(clippy::as_conversions)]
impl TryFrom<Event> for EditorCommand {
    type Error = String;

    /// 将crossterm中时间转换成自定义的EditorCommand
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUP)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::Char(ch), KeyModifiers::NONE | KeyModifiers::SHIFT) => Ok(Self::Insert(ch)),
                _ => Err(format!("Key Code not supported: {code:?}")),
            },
            Event::Resize(width_u16, height_u16) => Ok(Self::Resize(Size {
                height: height_u16 as usize,
                width: width_u16 as usize,
            })),
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}