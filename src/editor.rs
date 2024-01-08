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
                let key_text = String::from(Self::get_char_from_keycode(key));
                let _mod_text = format!("{modifiers:?}\r");
                
   
                match modifiers {
                    KeyModifiers::CONTROL if key == KeyCode::Char('q') => self.should_quit = true,
                    _ => self.print_at_cursor(&key_text).expect("print failed"),
                }

                Ok(())
            },
            Err(_) => Err("Error in read!".into()),
            _ => Ok(()),
        }
    }

    pub fn get_char_from_keycode(key: KeyCode) -> char {
        match key {
            KeyCode::Char('a') => 'a',
            KeyCode::Char('b') => 'b',
            KeyCode::Char('c') => 'c',
            KeyCode::Char('d') => 'd',
            KeyCode::Char('e') => 'e',
            KeyCode::Char('f') => 'f',
            KeyCode::Char('g') => 'g',
            KeyCode::Char('h') => 'h',
            KeyCode::Char('i') => 'i',
            KeyCode::Char('j') => 'j',
            KeyCode::Char('k') => 'k',
            KeyCode::Char('l') => 'l',
            KeyCode::Char('m') => 'm',
            KeyCode::Char('n') => 'n',
            KeyCode::Char('o') => 'o',
            KeyCode::Char('p') => 'p',
            KeyCode::Char('q') => 'q',
            KeyCode::Char('r') => 'r',
            KeyCode::Char('s') => 's',
            KeyCode::Char('t') => 't',
            KeyCode::Char('u') => 'u',
            KeyCode::Char('v') => 'v',
            KeyCode::Char('w') => 'w',
            KeyCode::Char('x') => 'x',
            KeyCode::Char('y') => 'y',
            KeyCode::Char('z') => 'z',
            KeyCode::Char('1') => '1',
            KeyCode::Char('2') => '2',
            KeyCode::Char('3') => '3',
            KeyCode::Char('4') => '4',
            KeyCode::Char('5') => '5',
            KeyCode::Char('6') => '6',
            KeyCode::Char('7') => '7',
            KeyCode::Char('8') => '8',
            KeyCode::Char('9') => '9',
            KeyCode::Char('0') => '0',
            KeyCode::Char('~') => '0',
            KeyCode::Char('`') => '`',
            KeyCode::Char('-') => '-',
            KeyCode::Char('=') => '=',
            _  => '?'
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
