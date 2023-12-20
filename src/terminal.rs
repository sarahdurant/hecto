pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size
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
        })
    }

    #[must_use]
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn draw_left_margin(&self, margin: &str) {
        for _ in 0..self.size.height {
            println!("{margin}\r");
        }
    }
}