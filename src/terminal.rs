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
    pub fn default() -> Result<Self, std::io::Error> {
        let (cols, rows) = crossterm::terminal::size()?;
        Ok(Self {
            size: Size {
                width: cols,
                height: rows,
            },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}