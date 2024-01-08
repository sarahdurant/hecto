use std::time::Duration;
use std::error::Error;

use crossterm::{
    event::{poll, read, Event, KeyModifiers, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::Terminal;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Cursor {
    x: u16,
    y: u16,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor: Cursor,
}

impl Editor {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        
        if let Err(error) = Terminal::initialize_screen(&mut self.terminal) {
            disable_raw_mode()?;
            panic!("{error}");
        }
        self.display_version();

        loop {
            if let Err(error) = Terminal::move_cursor_to(&mut self.terminal, self.cursor.x, self.cursor.y) {
                disable_raw_mode()?;
                panic!("{error}");
            }

            if poll(Duration::from_millis(500))? {
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
                
                let _mod_text = format!("{modifiers:?}\r");
   
                match modifiers {
                    KeyModifiers::CONTROL if key == KeyCode::Char('q') => self.should_quit = true,
                    _ => self.handle_unmodified_keycode(key),
                }

                Ok(())
            },
            Err(_) => Err("Error in read!".into()),
            _ => Ok(()),
        }
    }

    fn handle_unmodified_keycode(&mut self, key: KeyCode) {
        match Self::get_char_from_keycode(key) {
            Some(character) => {
                let key_text = String::from(character);
                let _ = self.print_at_cursor(&key_text);
            },
            None => {
                let wild_card = String::from('?');
                let _ = self.print_at_cursor(&wild_card);
            }
        }
    }

    //todo: convert to Into<char> for KeyCode?
    pub fn get_char_from_keycode(key: KeyCode) -> Option<char> {
        match key {
            KeyCode::Char('a') => Some('a'),
            KeyCode::Char('b') => Some('b'),
            KeyCode::Char('c') => Some('c'),
            KeyCode::Char('d') => Some('d'),
            KeyCode::Char('e') => Some('e'),
            KeyCode::Char('f') => Some('f'),
            KeyCode::Char('g') => Some('g'),
            KeyCode::Char('h') => Some('h'),
            KeyCode::Char('i') => Some('i'),
            KeyCode::Char('j') => Some('j'),
            KeyCode::Char('k') => Some('k'),
            KeyCode::Char('l') => Some('l'),
            KeyCode::Char('m') => Some('m'),
            KeyCode::Char('n') => Some('n'),
            KeyCode::Char('o') => Some('o'),
            KeyCode::Char('p') => Some('p'),
            KeyCode::Char('q') => Some('q'),
            KeyCode::Char('r') => Some('r'),
            KeyCode::Char('s') => Some('s'),
            KeyCode::Char('t') => Some('t'),
            KeyCode::Char('u') => Some('u'),
            KeyCode::Char('v') => Some('v'),
            KeyCode::Char('w') => Some('w'),
            KeyCode::Char('x') => Some('x'),
            KeyCode::Char('y') => Some('y'),
            KeyCode::Char('z') => Some('z'),
            KeyCode::Char('1') => Some('1'),
            KeyCode::Char('2') => Some('2'),
            KeyCode::Char('3') => Some('3'),
            KeyCode::Char('4') => Some('4'),
            KeyCode::Char('5') => Some('5'),
            KeyCode::Char('6') => Some('6'),
            KeyCode::Char('7') => Some('7'),
            KeyCode::Char('8') => Some('8'),
            KeyCode::Char('9') => Some('9'),
            KeyCode::Char('0') => Some('0'),
            KeyCode::Char('~') => Some('~'),
            KeyCode::Char('`') => Some('`'),
            KeyCode::Char('-') => Some('-'),
            KeyCode::Char('=') => Some('='),
            _  => None,
        }
    }

    pub fn display_version(&mut self) {
        let x = 3;
        let y = self.terminal.size().height / 3;
        let welcome_message = format!("Hecto editor -- version {}", VERSION);

        let _ = Terminal::print_at_pos(&mut self.terminal, x, y, &welcome_message);

    }

    fn print_at_cursor(&mut self, text: &str) -> Result<(), std::io::Error> {

        let x = self.cursor.x;
        let y = self.cursor.y;
        let x_max = self.terminal.size().width;
        let text_len = text.len() as u16;
        Terminal::print_at_pos(&mut self.terminal, x, y, &text)?;

        // we move the cursor right the length of the string
        // but handle wrapping at the end of the terminal
        if x + text_len <= x_max {
            self.cursor.x += text_len;
        } else {
            self.cursor.x = 0;
            self.cursor.y += 1;
        }

        Ok(())
    }
    
    pub fn default() -> Self {
        Editor {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize Terminal."),
            cursor: Cursor {x: 0, y: 0},
        }
    }
    
}
