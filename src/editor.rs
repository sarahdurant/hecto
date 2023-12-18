use std::{
    io::{stdout, Write},
    time::{Duration},
    error::Error,
};
use crossterm::{
    QueueableCommand, cursor,
    event::{poll, read, Event, KeyModifiers, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, ClearType},
};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        
        loop {
            if poll(Duration::from_millis(500))?{
                if let Err(error) = Editor::refresh_screen(&stdout) {
                    disable_raw_mode()?;
                    panic!("{error}");
                }
                
                if let Err(error) = self.process_keypress() {
                    disable_raw_mode()?;
                    panic!("{error}");
                }
                
                if self.should_quit {
                    println!("Exiting... Goodbye!");
                    break;
                }

                self.draw_rows();
                stdout.queue(cursor::MoveTo(0,0))?;
                
                stdout.flush()?;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
    
    fn refresh_screen(mut stdout: &std::io::Stdout) -> Result<(), std::io::Error> {
        stdout.queue(crossterm::terminal::Clear(ClearType::All))?;
        stdout.queue(cursor::MoveTo(0,0))?;
        stdout.flush()?;
        Ok(())
    }

    fn draw_rows(&self) {
        for _ in 0..24 {
            println!("~\r");
        }
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
        Editor{ should_quit: false}
    }
    
}
