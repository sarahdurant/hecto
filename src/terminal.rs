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


pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    stdout: std::io::Stdout,
}

impl Terminal {
    /// # Errors
    /// Returns Err if an underlying crossterm method such as size
    /// errors
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Result<Self, std::io::Error> {
        let (cols, rows) = crossterm::terminal::size()?;
        Ok(Self {
            size: Size {
                width: cols,
                height: rows,
            },
            stdout: stdout(),
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn draw_left_margin(&mut self, margin: &str) -> Result<(), std::io::Error> {
        //stdout.queue(cursor::SavePosition());
        for _ in 0..self.size.height {
            println!("{margin}\r");
        }
        //stdout.queue(cursor::RestorePosition());
        self.stdout.flush()?;
        //println!("{:?}", orig_cursor);

        Ok(())
    }

    pub fn initialize_screen(&mut self) -> Result<(), std::io::Error> {
        self.stdout.queue(crossterm::terminal::Clear(ClearType::All))?;
        self.stdout.queue(cursor::MoveTo(0,0))?;
        self.stdout.flush()?;

        Ok(())
    }
}
