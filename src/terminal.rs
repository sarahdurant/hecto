use std::{
    io::{stdout, Write},
};
use crossterm::{
    QueueableCommand, cursor,
    terminal::{ClearType},
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
        for _ in 0..self.size.height {
            println!("{margin}\r");
        }
        
        Ok(())
    }

    pub fn initialize_screen(&mut self) -> Result<(), std::io::Error> {
        self.stdout.queue(crossterm::cursor::Hide)?;
        self.stdout.queue(cursor::SavePosition)?;
        self.stdout.queue(crossterm::terminal::Clear(ClearType::All))?;
        self.stdout.queue(cursor::MoveTo(0,0))?;

        Terminal::draw_left_margin(self, &String::from("~"))?;

        self.stdout.queue(cursor::RestorePosition)?;
        self.stdout.queue(crossterm::cursor::Show)?;
        self.stdout.flush()?;

        Ok(())
    }

    pub fn print_at_pos(&mut self, x: u16, y: u16, text: &str) -> Result<(), std::io::Error> {
        let width = self.size.width;
        let text_len = std::cmp::min(text.len(), width.into());

        self.stdout.queue(crossterm::cursor::Hide)?;
        self.stdout.queue(cursor::SavePosition)?;
        self.stdout.queue(cursor::MoveTo(x,y))?;

        println!("{}", &text[..text_len]);

        self.stdout.queue(cursor::RestorePosition)?;
        self.stdout.queue(crossterm::cursor::Show)?;
        self.stdout.flush()?;

        Ok(())
    }
}
