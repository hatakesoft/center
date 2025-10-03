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
        style,
        terminal,
    },
};

pub struct Color {
    background: style::Color,
    foreground: style::Color,
}

impl Color {
    pub fn set(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            style::SetBackgroundColor(self.background),
            style::SetForegroundColor(self.foreground),
        )
    }
}

pub struct Theme {
    text: Color,
    cursor: Color,
}

impl Theme {
    pub fn default_dark() -> Self {
        Self {
            text: Color {
                background: style::Color::Ansi(16),
                foreground: style::Color::Ansi(231),
            },
            cursor: Color {
                background: style::Color::Ansi(231),
                foreground: style::Color::Ansi(16),
            },
        }
    }

    pub fn default_light() -> Self {
        Self {
            text: Color {
                background: style::Color::Ansi(231),
                foreground: style::Color::Ansi(16),
            },
            cursor: Color {
                background: style::Color::Ansi(16),
                foreground: style::Color::Ansi(231),
            },
        }
    }
}

pub struct Display {
    col: u16,
    row: u16,
    cursor_x: u16,
    cursor_y: u16,
    theme: Theme,
}

impl Display {
    pub fn new() -> Self {
        Self {
            col: 0,
            row: 0,
            cursor_x: 0,
            cursor_y: 0,
            theme: Theme::default_dark(),
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
        self.theme.text.set()?;
        for content in file_content {
            if i >= self.row - 1 {
                break;
            }
            let print_str = if usize::from(i) >= file_content.len() {
                "~"
            } else if content.len() < self.col.into() {
                &content
            } else {
                &content[0..self.col.into()]
            };
            if i == self.cursor_y {
                print!("{}", print_str[0..self.cursor_x]);
                self.theme.cursor.set()?;
                print!("{}", print_str[cursor_x..cursor_x + 1]);
                self.theme.text.set()?;
                println!("{}", print_str[self.cursor_x + 1..print_str.len()]);
            } else {
                println!("{}", print_str);
            }
            i += 1;
        }
        execute!(
            io::stdout(),
            cursor::MoveTo(self.cursor_x, self.cursor_y),
        )?;
        Ok(())
    }
}
