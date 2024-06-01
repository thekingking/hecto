use super::terminal::{
    Terminal,
    Size
};
use std::io::Error;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View;

impl View {
    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width as usize;
        let len = welcome_message.len();
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len)) / 2;

        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn draw_empty_rows() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }

    pub fn render() -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        Terminal::clear_line()?;
        Terminal::print("hello, world!\r\n")?;
        for current_row in 1..height {
            Terminal::clear_line()?;
            #[allow(clippy::integer_division)]
            if current_row == height / 3 {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_rows()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }
}