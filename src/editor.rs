use std::{
  io::{self},
  time::{Duration},
};
use crossterm::{
  event::{poll, read, Event, KeyModifiers, KeyCode},
  terminal::{disable_raw_mode, enable_raw_mode},
};

pub struct Editor {}

impl Editor {
  pub fn run(&self) -> io::Result<()> {
    enable_raw_mode()?;

    loop {
        if poll(Duration::from_millis(500))?{
            if let Event::Key(event) = read()? {
                let key = event.code;
                let modifiers = event.modifiers;
                
                println!("{:?}\r", key);         
                println!("{:?}\r", modifiers);         
                if key == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL {
                    println!("Exiting... Good-bye!");
                    break;
                }
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
  }

  pub fn default() -> Self {
    Editor{}
  }

}