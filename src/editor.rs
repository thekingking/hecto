use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use std::{
    env, 
    io::Error,
    panic::{set_hook, take_hook},
};

mod terminal;
mod view;
mod editorcommand;
use terminal::Terminal;
use view::View;
use editorcommand::EditorCommand;

pub struct Editor {
    should_quit: bool,
    view: View,
}

impl Editor {
    // 创建Editor，修改panic，读取命令行参数，初始化属性，加载文件内容到buffer中
    pub fn new() -> Result<Self, Error> {
        // 对panic进行修改，保证panic之后能够对terminal进行正确关闭
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        // 初始化Editor中数据
        Terminal::initialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        // 读取命令行参数，将对应文件中数据加载到Editor中
        if let Some(first_arg) = args.get(1) {
            view.load(&first_arg);
        }
        Ok(Self {
            should_quit: false,
            view,
        })
    }

    /// Editor核心运行函数
    /// 循环读取事件，渲染view
    pub fn run(&mut self) {
        loop {
            // 先加载screen中内容，再读取事件
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    /// 加载screen内容
    /// 先隐藏光标，然后渲染view，最后显示光标，整个事件放在queue中，最后一次执行所有操作
    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        let _ = Terminal::move_caret_to(self.view.get_position());
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    /// 对event进行处理
    #[allow(clippy::needless_pass_by_value)]
    fn evaluate_event(&mut self, event: Event) {
        // 仅当事件为需要进行处理的预设事件时才进行事件处理
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };
        if should_process {
            // 将crossterm中的event转换为自定义的EditorCommand
            match EditorCommand::try_from(event) {
                Ok(command) => {
                    if matches!(command, EditorCommand::Quit) {
                        // 退出Editor
                        self.should_quit = true;
                    } else {
                        // 其他事件处理
                        self.view.handle_command(command);
                    }
                }
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not handle command: {err:?}");
                    }
                }
            }
        }
    }
}

impl Drop for Editor {
    /// 在退出Editor时对Terminal进行正确关闭，对应new中初始化Terminal
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Goodbye. \r\n");
        }
    }
}