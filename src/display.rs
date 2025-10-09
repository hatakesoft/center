//! # CONTENT
//! display printing and related program

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

/// # CONTENT
/// printed color handle
/// # FIELD
/// - background: background color
/// - foreground: foreground color
pub struct Color {
    background: style::Color,
    foreground: style::Color,
}

impl Color {
    /// # CONTENT
    /// set printed color
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// io::Result<()>
    pub fn set(&self) -> io::Result<()> {
        execute!(
            io::stdout(),
            style::SetBackgroundColor(self.background),
            style::SetForegroundColor(self.foreground),
        )
    }
}

/// # CONTENT
/// color theme
/// # FIELD
/// - text: text color
/// - cursor: cursor color
pub struct Theme {
    text: Color,
    cursor: Color,
}

impl Theme {
    /// # CONTENT
    /// set default dark theme
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// Theme seted default dark
    pub fn default_dark() -> Self {
        Self {
            text: Color {
                background: style::Color::AnsiValue(16),
                foreground: style::Color::AnsiValue(231),
            },
            cursor: Color {
                background: style::Color::AnsiValue(231),
                foreground: style::Color::AnsiValue(16),
            },
        }
    }

    /// # CONTENT
    /// set default light theme
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// Theme seted default light
    pub fn default_light() -> Self {
        Self {
            text: Color {
                background: style::Color::AnsiValue(231),
                foreground: style::Color::AnsiValue(16),
            },
            cursor: Color {
                background: style::Color::AnsiValue(16),
                foreground: style::Color::AnsiValue(231),
            },
        }
    }
}

/// # CONTENT
/// display info
/// # FIELD
/// - col: display col
/// - row: display row
/// - cursor_x: cursor x
/// - cursor_y: cursor y
/// - theme: color theme
pub struct Display {
    col: u16,
    row: u16,
    cursor_x: u16,
    cursor_y: u16,
    theme: Theme,
}

impl Display {
    /// # CONTENT
    /// init Display
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// inited Display
    pub fn new() -> Self {
        Self {
            col: 0,
            row: 0,
            cursor_x: 0,
            cursor_y: 0,
            theme: Theme::default_dark(),
        }
    }

    /// # CONTENT
    /// update File::col and File::row
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// Ok(()): ok
    /// Err(...): error
    pub fn resize(&mut self) -> io::Result<()> {
        (self.col, self.row) = terminal::size()?;
        Ok(())
    }

    /// # CONTENT
    /// print display
    /// # ARGUMENT
    /// file_handle: print file content
    /// # RETURN VALUE
    /// Ok(()): ok
    /// Err(...): error
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
                print!("{}", &print_str[0..(self.cursor_x as usize)]);
                self.theme.cursor.set()?;
                print!("{}", &print_str[self.cursor_x as usize]);
                self.theme.text.set()?;
                println!("{}", &print_str[(self.cursor_x as usize) + 1..print_str.len()]);
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
