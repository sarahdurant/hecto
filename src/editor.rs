use std::time::Duration;
use std::error::Error;

use crossterm::{
    event::{poll, read, Event, KeyModifiers, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::Terminal;

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

        Terminal::draw_left_margin(&mut self.terminal, &String::from("~"))?;

        loop {
            if poll(Duration::from_millis(500))? {
                if let Err(error) = Terminal::initialize_screen(&mut self.terminal) {
                    disable_raw_mode()?;
                    panic!("{error}");
                }

                Terminal::draw_left_margin(&mut self.terminal, &String::from("~"))?;

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
                
                println!("{key:?}\r");         
                println!("{modifiers:?}\r");         
                if key == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL {
                    self.should_quit = true;
                }
                Ok(())
            },
            Err(_) => Err("Error in read!".into()),
            _ => Ok(()),
        }
    }
    
    pub fn default() -> Self {
        Editor {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize Terminal."),
        }
    }
    
}
