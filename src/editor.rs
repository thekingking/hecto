use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Editor{
            should_quit: false,
        }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            if self.should_quit {
                break;
            } else {
                Terminal::refresh_screen()?;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent{
            code,
            modifiers,
            ..
        }) = event {
            match code {
                Char('c') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                Char(c) => print!("{}", c),
                _ => (),
            }
        }
    }
}