use crossterm::cursor::{MoveTo, Hide, Show};
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::style::Print;
use std::io::{stdout, Write};

pub struct Terminal {}

impl Terminal {
    /// 开启
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)?;
        stdout().flush()?;
        Ok(())
    }

    /// 关闭
    pub fn terminate() -> Result<(), std::io::Error> {
        Self::clear_screen()?;
        print!("GoodBye.\r\n");
        stdout().flush()?;
        disable_raw_mode()?;
        Ok(())
    }

    /// 刷新终端
    pub fn refresh_screen() -> Result<(), std::io::Error> {
        Self::hide_cursor()?;
        Self::draw_rows()?;
        Self::show_cursor()?;
        Self::move_cursor_to(0, 0)?;
        stdout().flush()?;
        Ok(())
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        queue!(stdout(), Clear(ClearType::All))?;
        Ok(())
    }

    fn move_cursor_to(x: u16, y: u16) -> Result<(), std::io::Error> {
        queue!(stdout(), MoveTo(x, y))?;
        Ok(())
    }

    fn hide_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Hide)?;
        Ok(())
    }

    fn show_cursor() -> Result<(), std::io::Error> {
        queue!(stdout(), Show)?;
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let height = size()?.1;
        for current_row in 0..height {
            queue!(stdout(), Print("~"))?;
            if current_row + 1 < height {
                queue!(stdout(), Print("\r\n"))?;
            }
        }
        Ok(())
    }
}
