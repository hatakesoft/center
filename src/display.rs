//! # CONTENT
//! display printing and related program

use {
    std:: {
        io,
    },
    crate:: {
        cmd,
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
    bar: Color,
    center: Color,
    center_col: Color,
    center_row: Color,
    cmd: Color,
    code: Color,
    col_num: Color,
    file_path: Color,
    row_num: Color,
    space: Color,
}

impl Theme {
    /// # CONTENT
    /// get dracula theme
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// Theme
    pub fn dracula() -> Self {
        let bg = style::Color::Rgb {r: 0xF8, g: 0xF8, b: 0xF2};
        let fg = style::Color::Rgb {r: 0x28, g: 0x2A, b: 0x36};
        Self {
            bar: Color {
                background: style::Color::Rgb {r: 0x19, g: 0x1A, b: 0x21},
                foreground: fg,
            },
            center: Color {
                background: fg,
                foreground: bg,
            },
            center_col: Color {
                background: style::Color::Rgb {r: 0x19, g: 0x1A, b: 0x21},
                foreground: fg,
            },
            center_row: Color {
                background: style::Color::Rgb {r: 0x19, g: 0x1A, b: 0x21},
                foreground: fg,
            },
            cmd: Color {
                background: bg,
                foreground: fg,
            },
            code: Color {
                background: bg,
                foreground: fg,
            },
            col_num: Color {
                background: style::Color::Rgb {r: 0x19, g: 0x1A, b: 0x21},
                foreground: style::Color::Rgb {r: 0x62, g: 0x72, b: 0xA4},
            },
            file_path: Color {
                background: style::Color::Rgb {r: 0x44, g: 0x47, b: 0x5A},
                foreground: fg,
            },
            row_num: Color {
                background: style::Color::Rgb {r: 0x19, g: 0x1A, b: 0x21},
                foreground: style::Color::Rgb {r: 0x62, g: 0x72, b: 0xA4},
            },
            space: Color {
                background: style::Color::Rgb {r: 0x21, g: 0x22, b: 0x2C},
                foreground: fg,
            },
        }
    }
}

/// # CONTENT
/// display info
/// # FIELD
/// - col: display col
/// - row: display row
/// - center_x: center pos x
/// - center_y: center pos y
/// - theme: color theme
pub struct Display {
    col: usize,
    row: usize,
    center_x: usize,
    center_y: usize,
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
            center_x: 0,
            center_y: 0,
            theme: Theme::dracula(),
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
        let (col, row) = terminal::size()?;
        self.col = col as usize;
        self.row = row as usize;
        Ok(())
    }

    /// # CONTENT
    /// print display
    /// # ARGUMENT
    /// file_handle: print file content
    /// # RETURN VALUE
    /// Ok(()): ok
    /// Err(...): error
    pub fn print(&self, cmd_handle: &cmd::Cmd, file_handle: &file::File) -> io::Result<()> {
        let file_content = file_handle.get_content();
        let code_top_num = self.center_y as isize - (self.row as isize - 4) / 2 + if (self.row - 4) % 2 == 0 { 1 } else { 0 };
        let code_bottom_num = self.center_y as isize + (self.row as isize - 4) / 2;
        let code_left = code_bottom_num.to_string().len();
        let code_right = self.col;
        let code_top = 2;
        let code_bottom = self.row - 2;
        let code_col = code_right - code_left;
        let code_row = code_bottom - code_top;
        let code_left_num = self.center_x as isize - code_col as isize / 2 + if code_col % 2 == 0 { 1 } else { 0 };
        let code_right_num = self.center_x as isize + code_col as isize / 2;
        execute!(
            io::stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )?;
        if self.center_y >= file_content.len() {
            panic!("center_y requested access outside the range");
        }

        // file path
        self.theme.file_path.set()?;
        if file_handle.path.len() >= self.col {
            println!("{}", file_handle.path);
        } else {
            println!("{}", &file_handle.path[file_handle.path.len() - self.col..]);
        }

        // col number
        self.theme.col_num.set()?;
        for _ in 0..code_left {
            print!(" ");
        }
        if code_left_num <= 0 {
            for _ in 0..code_left {
                print!(" ");
            }
            print!("    "); // skip 0~3
            for i in 1..code_right_num / 5 {
                if i * 5 - 1 == self.center_x as isize {
                    self.theme.center_col.set()?;
                }
                print!("{:<5}", i * 5);
                if i * 5 - 1 == self.center_x as isize {
                    self.theme.col_num.set()?;
                }
            }
        } else {
            for _ in code_left_num..(code_left_num / 5 + 1) * 5 - 1 {
                print!(" ");
            }
            for i in code_left_num / 5 + 1..code_right_num / 5 {
                if i * 5 - 1 == self.center_x as isize {
                    self.theme.center_col.set()?;
                }
                print!("{:<5}", i * 5);
                if i * 5 - 1 == self.center_x as isize {
                    self.theme.col_num.set()?;
                }
            }
        }
        println!("");

        // top space
        self.theme.space.set()?;
        if code_top_num < 0 {
            for _ in 0..code_top_num.abs() {
                println!("");
            }
        }

        let mut i = 0;
        for content in file_content {
            if i > code_row {
                break;
            }

            // row number
            self.theme.row_num.set()?;
            print!("{:>width$}", if code_top_num < 0 { i } else { code_top_num.try_into().unwrap() + i }, width = code_left);

            // left space
            self.theme.space.set()?;
            if code_left_num < 0 {
                for _ in 0..code_left_num.abs() as usize {
                    print!(" ");
                }
            }

            // code
            let print_str = if code_left_num < 0 {
                if content.len() < code_col - code_left_num.abs() as usize {
                    &content
                } else {
                    &content[..code_col - code_left_num.abs() as usize]
                }
            } else {
                if content.len() < code_right_num as usize {
                    &content[code_left_num as usize..]
                } else {
                    &content[code_left_num as usize..code_right_num as usize]
                }
            };
            if i == if code_top_num < 0 { i } else { code_top_num + i } {
                // center
                if self.center_x >= content.len() {
                    panic!("center_x requested access outside the range");
                }
                print!("{}", &print_str[..self.center_x]);
                self.theme.center.set()?;
                print!("{}", &print_str[self.center_x - 1..self.center_x]);
                self.theme.code.set()?;
                println!("{}", &print_str[self.center_x + 1..]);
            } else {
                println!("{}", print_str);
            }
            i += 1;
        }

        // bottom space
        self.theme.space.set()?;
        if file_content.len() < code_bottom_num as usize {
            for _ in file_content.len()..code_bottom_num as usize {
                println!("");
            }
        }

        // bar
        self.theme.bar.set()?;
        println!("{}", if self.col > 23 + (self.center_x + 1).to_string().len() + (self.center_y + 1).to_string().len() + self.col.to_string().len() + self.row.to_string().len() {
            format!("center({}, {})  display({}, {})", self.center_x + 1, self.center_y + 1, self.col, self.row)
        } else if self.col > 10 + (self.center_x + 1).to_string().len() + (self.center_y + 1).to_string().len() {
            format!("center({}, {})", self.center_x + 1, self.center_y + 1)
        } else {
            "".to_string()
        });

        // cmd
        self.theme.cmd.set()?;
        println!("{}", if cmd_handle.buffer.len() <= self.col {
            cmd_handle.buffer
        } else {
            format!("..{}", &cmd_handle.buffer[cmd_handle.buffer.len() - self.col + 2..])
        });

        execute!(
            io::stdout(),
            cursor::MoveTo(0, 0),
        )?;
        Ok(())
    }
}
