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
    /// get one dark theme
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// Theme
    pub fn one_dark() -> Self {
        let bg = style::Color::Rgb {r: 0x28, g: 0x2C, b: 0x34};
        let fg = style::Color::Rgb {r: 0xAB, g: 0xB2, b: 0xBF};
        Self {
            bar: Color {
                background: style::Color::Rgb {r: 0x21, g: 0x25, b: 0x2B},
                foreground: style::Color::Rgb {r: 0x9D, g: 0xA5, b: 0xB4},
            },
            center: Color {
                background: style::Color::Rgb {r: 0x52, g: 0x8B, b: 0xFF},
                foreground: fg,
            },
            center_col: Color {
                background: bg,
                foreground: style::Color::Rgb {r: 0xAB, g: 0xB2, b: 0xBF},
            },
            center_row: Color {
                background: bg,
                foreground: style::Color::Rgb {r: 0xAB, g: 0xB2, b: 0xBF},
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
                background: bg,
                foreground: style::Color::Rgb {r: 0x63, g: 0x6D, b: 0x83},
            },
            file_path: Color {
                background: style::Color::Rgb {r: 0x21, g: 0x25, b: 0x2B},
                foreground: style::Color::Rgb {r: 0x9D, g: 0xA5, b: 0xB4},
            },
            row_num: Color {
                background: bg,
                foreground: style::Color::Rgb {r: 0x63, g: 0x6D, b: 0x83},
            },
            space: Color {
                background: style::Color::Rgb {r: 0x21, g: 0x25, b: 0x2B},
                foreground: style::Color::Rgb {r: 0x21, g: 0x25, b: 0x2B},
            },
        }
    }

    /// # CONTENT
    /// get one light theme
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// Theme
    pub fn one_light() -> Self {
        let bg = style::Color::Rgb {r: 0xFA, g: 0xFA, b: 0xFA};
        let fg = style::Color::Rgb {r: 0x38, g: 0x3A, b: 0x42};
        Self {
            bar: Color {
                background: style::Color::Rgb {r: 0xEA, g: 0xEA, b: 0xEB},
                foreground: style::Color::Rgb {r: 0x42, g: 0x42, b: 0x43},
            },
            center: Color {
                background: style::Color::Rgb {r: 0x52, g: 0x6F, b: 0xFF},
                foreground: bg,
            },
            center_col: Color {
                background: bg,
                foreground: fg,
            },
            center_row: Color {
                background: bg,
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
                background: bg,
                foreground: style::Color::Rgb {r: 0x9D, g: 0x9D, b: 0x9F},
            },
            file_path: Color {
                background: style::Color::Rgb {r: 0xEA, g: 0xEA, b: 0xEB},
                foreground: style::Color::Rgb {r: 0x42, g: 0x42, b: 0x43},
            },
            row_num: Color {
                background: bg,
                foreground: style::Color::Rgb {r: 0x9D, g: 0x9D, b: 0x9F},
            },
            space: Color {
                background: style::Color::Rgb {r: 0xEA, g: 0xEA, b: 0xEB},
                foreground: style::Color::Rgb {r: 0xEA, g: 0xEA, b: 0xEB},
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
    pub center_x: usize,
    pub center_y: usize,
    pub theme: Theme,
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
            theme: Theme::one_light(),
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
        let file_path = match &file_handle.path {
            Some(n) => n,
            None => "".to_string(),
        };
        self.theme.file_path.set()?;
        if file_path.len() <= self.col {
            print!("{}", file_path);
            for _ in file_path.len()..self.col {
                print!(" ");
            }
            println!("");
        } else {
            println!("..{}", &file_path[file_path.len() - self.col + 2..]);
        }

        // col number
        self.theme.col_num.set()?;
        for _ in 0..code_left {
            print!(" ");
        }
        if code_left_num <= 0 {
            for _ in 0..code_left_num.abs() as usize {
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
        self.theme.col_num.set()?;
        for _ in 0..code_right_num as usize % 5 {
            print!(" ");
        }
        println!("");

        // top space
        if code_top_num < 0 {
            for _ in 0..code_top_num.abs() {
                self.theme.row_num.set()?;
                for _ in 0..code_left {
                    print!(" ");
                }
                self.theme.space.set()?;
                for _ in 0..self.col - code_left {
                    print!(" ");
                }
                println!("");
            }
        }

        let mut i: usize = 0;
        for content in file_content {
            if i > code_bottom_num as usize {
                break;
            }

            // row number
            if self.center_y == if code_top_num < 0 { i } else { code_top_num as usize + i } {
                self.theme.center_row.set()?;
            } else {
                self.theme.row_num.set()?;
            }
            print!("{:>width$}", if code_top_num < 0 { i } else { code_top_num as usize + i } + 1, width = code_left);

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
                    let mut _c = content.clone();
                    for _ in content.len()..code_col - code_left_num.abs() as usize {
                        _c.push(' ');
                    }
                    _c
                } else {
                    content[..code_col - code_left_num.abs() as usize].to_string()
                }
            } else {
                if content.len() < code_right_num as usize {
                    let mut _c = content[code_left_num as usize..].to_string();
                    for _ in content.len()..code_right_num as usize {
                        _c.push(' ');
                    }
                    _c
                } else {
                    content[code_left_num as usize..code_right_num as usize].to_string()
                }
            };
            if self.center_y == if code_top_num < 0 { i } else { code_top_num as usize + i } {
                // center
                if self.center_x < content.len() {
                    self.theme.code.set()?;
                    print!("{}", &print_str[..self.center_x]);
                    self.theme.center.set()?;
                    print!("{}", &print_str[self.center_x..self.center_x + 1]);
                    self.theme.code.set()?;
                    println!("{}", &print_str[self.center_x + 1..]);
                } else if self.center_x > content.len() {
                    panic!("center_x requested access outside the range");
                }
            } else {
                self.theme.code.set()?;
                println!("{}", print_str);
            }
            i += 1;
        }

        // bottom space
        if file_content.len() < code_bottom_num as usize {
            for _ in file_content.len()..code_bottom_num as usize {
                self.theme.row_num.set();
                for _ in 0..code_left {
                    print!(" ");
                }
                self.theme.space.set()?;
                for _ in code_left..self.col {
                    print!(" ");
                }
                println!("");
            }
        }

        // bar
        self.theme.bar.set()?;
        println!("{}", if self.col > 23 + (self.center_x + 1).to_string().len() + (self.center_y + 1).to_string().len() + self.col.to_string().len() + self.row.to_string().len() {
            let mut _s = format!("center({}, {})  display({}, {})", self.center_x + 1, self.center_y + 1, self.col, self.row);
            for _ in 23 + (self.center_x + 1).to_string().len() + (self.center_y + 1).to_string().len() + self.col.to_string().len() + self.row.to_string().len()..self.col {
                _s.push(' ');
            }
            _s
        } else if self.col > 10 + (self.center_x + 1).to_string().len() + (self.center_y + 1).to_string().len() {
            let mut _s = format!("center({}, {})", self.center_x + 1, self.center_y + 1);
            for _ in 10 + (self.center_x + 1).to_string().len() + (self.center_y + 1).to_string().len()..self.col {
                _s.push(' ');
            }
            _s
        } else {
            let mut _s = String::new();
            for _ in 0..self.col {
                _s.push(' ');
            }
            _s
        });

        // cmd
        self.theme.cmd.set()?;
        println!("{}", if cmd_handle.buffer.len() <= self.col {
            let mut _c = cmd_handle.buffer.clone();
            for _ in cmd_handle.buffer.len()..self.col {
                _c.push(' ');
            }
            _c
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
