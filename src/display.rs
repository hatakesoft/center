use {
    std:: {
        io,
    },
    crate:: {
        file,
    },
    crossterm:: {
        cursor,
        execute,
        terminal,
    },
};

pub struct Display {
    col: u16,
    row: u16,
}

impl Display {
    pub fn new() -> Self {
        Self {
            col: 0,
            row: 0,
        }
    }

    pub fn resize(&mut self) -> io::Result<()> {
        (self.col, self.row) = terminal::size()?;
        Ok(())
    }

    pub fn print(&self, file_handle: &file::File) -> io::Result<()> {
        let file_content = file_handle.get_content();
        let mut i = 0;
        execute!(
            io::stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )?;
        for content in file_content {
            if i >= self.row - 1 {
                break;
            } else if usize::from(i) >= file_content.len() {
                println!("~");
            } else if content.len() < self.col.into() {
                println!("{}", content);
            } else {
                println!("{}", &content[0..self.col.into()]);
            }
            i += 1;
        }
        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0),
        )?;
        Ok(())
    }
}
