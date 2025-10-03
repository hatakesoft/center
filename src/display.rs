mod cmd;
mod file;

use {
    std:: {
        io,
    },
    crossterm:: {
        terminal,
    },
}

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
        let size = terminal::size()?;
        self.col = size.0;
        self.row = size.1;
        Ok(())
    }

    pub fn print(&self, file_handle: &file::File) {
        let file_content = file_handle.get_content();
        let mut i = 0;
        for content in file_content {
            if i >= content.len() {
                print!("~");
                for _ in 1..self.col {
                    print!(" ");
                }
            } else if content.len() < self.col {
                print!("{}", content);
                for _ in content.len()..self.col {
                    print!(" ");
                }
            } else {
                print!("{}", content[0..self.col]);
            }
            i ++;
        }
    }
}
