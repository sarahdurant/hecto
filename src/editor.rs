use std::time::Duration;
use std::error::Error;

use crossterm::{
    event::{poll, read, Event, KeyModifiers, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        
        if let Err(error) = Terminal::initialize_screen(&mut self.terminal){
            disable_raw_mode()?;
            panic!("{error}");
        }

        loop {
            if poll(Duration::from_millis(500))? {
                // if let Err(error) = Terminal::initialize_screen(&mut self.terminal) {
                //     disable_raw_mode()?;
                //     panic!("{error}");
                // }

                self.display_version();

                if let Err(error) = self.process_keypress() {
                    disable_raw_mode()?;
                    panic!("{error}");
                }
                
                if self.should_quit {
                    println!("Exiting... Goodbye!");
                    break;
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }

    pub fn process_keypress(&mut self) -> Result<(), Box<dyn Error>> {
        match read() {
            Ok(Event::Key(event)) => {
                let key = event.code;
                let modifiers = event.modifiers;
                let key_text = format!("{key:?}\r");
                let mod_text = format!("{modifiers:?}\r");
                
                Terminal::print_at_pos(&mut self.terminal, 10, 10, &key_text)?;
                Terminal::print_at_pos(&mut self.terminal, 10, 11, &mod_text)?;
                
                if key == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL {
                    self.should_quit = true;
                }
                Ok(())
            },
            Err(_) => Err("Error in read!".into()),
            _ => Ok(()),
        }
    }

    pub fn display_version(&mut self) {
        let x = 3;
        let y = self.terminal.size().height / 3;

        let _ = Terminal::print_at_pos(&mut self.terminal, x, y, &VERSION);

    }
    
    pub fn default() -> Self {
        Editor {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize Terminal."),
        }
    }
    
}
