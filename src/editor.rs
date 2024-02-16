use std::time::Duration;
use std::error::Error;
use std::env;

use crossterm::{
    event::{poll, read, Event, KeyModifiers, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};

use crate::Terminal;
use crate::Document;
use crate::Row;


const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Cursor {
    x: u16,
    y: u16,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor: Cursor,
    document: Document,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(file_name.clone()).unwrap_or_default()
        } else {
            Document::default()
        };

        Editor {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize Terminal."),
            cursor: Cursor {x: 0, y: 0},
            document,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        
        if let Err(error) = Terminal::initialize_screen(&mut self.terminal) {
            disable_raw_mode()?;
            panic!("{error}");
        }
        self.display_version();

        loop {
            //display the editor's document row by row
            if let Err(error) = self.refresh_screen() {
                disable_raw_mode()?;
                panic!("{error}");
            }

            if self.should_quit {
                //println!("Exiting... Goodbye!");
                break;
            }

            //detect keyboard input and update document/cursor as needed
            if poll(Duration::from_millis(500))? {
                if let Err(error) = self.process_keypress() {
                    disable_raw_mode()?;
                    panic!("{error}");
                }
            }
        }
        disable_raw_mode()?;
        Ok(())
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        
        Terminal::move_cursor_to(&mut self.terminal, 0, 0)?;

        if self.should_quit {
            let _  = Terminal::clear_screen(&mut self.terminal);
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
        }

        // move the cursor to whereever the editor says it's supposed to be
        Terminal::move_cursor_to(&mut self.terminal, self.cursor.x, self.cursor.y)?;

        Ok(())
    }

    fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end);
        println!("{}\r", row)
    }

    fn draw_rows(&mut self) {
        let height = self.terminal.size().height;
        
        for terminal_row in 0..height-1 {
            let _ = Terminal::clear_current_line(&mut self.terminal);

            if let Some(row) = self.document.row(terminal_row.into()) {
                self.draw_row(row);
            } else if terminal_row == height / 3 {
                self.display_version();
            } else {
                println!("~\r");
            }
        }
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


                match key {
                    KeyCode::Left => self.move_cursor_rel(-1, 0),
                    KeyCode::Right => self.move_cursor_rel(1, 0),
                    KeyCode::Up => self.move_cursor_rel(0, -1),
                    KeyCode::Down => self.move_cursor_rel(0, 1),
                    KeyCode::Backspace => {
                        self.move_cursor_rel(-1, 0);
                        self.delete_at_cursor();
                    },
                    KeyCode::Delete => self.delete_at_cursor(),
                    _ => {
                        let wild_card = String::from('?');
                        let _ = self.print_at_cursor(&wild_card);
                    }
                }
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

    fn move_cursor_rel(&mut self, x: i16, y: i16) {
        let x_max = self.terminal.size().width;
        let mut tmp : i16 = self.cursor.x as i16 + x;
        // If you try to move left at left-most, right at right-most,
        // or up at up-most, do nothing. We always let the cursor scroll down
        if tmp <= x_max.try_into().unwrap() && tmp >= 0 {
            self.cursor.x = tmp as u16;
        }

        tmp = self.cursor.y as i16 + y;
        if tmp >=0 {
            self.cursor.y = tmp as u16;
        }  
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

    pub fn delete_at_cursor(&mut self) {
        let _ = Terminal::delete_at_pos(&mut self.terminal, self.cursor.x, self.cursor.y);
    }
}
