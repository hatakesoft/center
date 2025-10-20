//! # CONTENT
//! cmd-related programs

use {
    crate:: {
        display,
        file,
    }
};

const CMD_APPEND:                    &str = "a";
const CMD_BACKSPACE_LEFT:            &str = "b";
const CMD_BACKSPACE_RIGHT:           &str = "B";
const CMD_CENTER_DOWN:               &str = "j";
const CMD_CENTER_END_FILE:           &str = "J";
const CMD_CENTER_END_LINE:           &str = "K";
const CMD_CENTER_LEFT:               &str = "h";
const CMD_CENTER_RIGHT:              &str = "k";
const CMD_CENTER_START_FILE:         &str = "I";
const CMD_CENTER_START_LINE:         &str = "H";
const CMD_CENTER_UP:                 &str = "i";
const CMD_NEWLINE:                   &str = "n";
const CMD_QUIT:                      &str = "q";
const CMD_REPLACE:                   &str = "r";
const CMD_THEME_CHANGE_TO_ONE_DARK:  &str = "t:one_dark";
const CMD_THEME_CHANGE_TO_ONE_LIGHT: &str = "t:one_light";

#[derive(Clone)]
enum Mode {
    Append,
    Default,
    Replace,
}

/// # CONTENT
/// cmd info
/// # FIELD
/// - cmd: cmd history
pub struct Cmd {
    history: String,
    pub buffer: String,
    mode: Mode,
}

impl Cmd {
    /// # CONTENT
    /// init Cmd
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// inited Cmd
    pub fn new() -> Self {
        Self {
            history: String::new(),
            buffer: String::new(),
            mode: Mode::Default,
        }
    }

    fn check(&mut self, cmd_msg: &str, key: char) -> bool {
        let mut start = self.buffer.clone();
        start.push(key);
        if cmd_msg.to_string().starts_with(&start) {
            self.buffer.push(key);
            if cmd_msg.to_string() == start {
                self.history += &self.buffer;
                self.buffer = String::new();
                return true;
            }
        }
        false
    }

    /// # CONTENT
    /// execute cmd
    /// # ARGUMENT
    /// - key: enterd key event
    /// # RETURN VALUE
    /// - true: continue program
    /// - false: quit program
    pub fn key(&mut self, key: char, display_handle: &mut display::Display, file_handle: &mut file::File) -> bool {
        match self.mode.clone() {
            Mode::Append => {
                self.buffer.push(key);
                if key == '\n' {
                    file_handle.content[display_handle.center_y].remove(display_handle.center_x);
                    self.history += &self.buffer;
                } else {
                    file_handle.content[display_handle.center_y].insert(display_handle.center_x, key);
                    display_handle.center_x += 1;
                }
            }
            Mode::Default => {
                if key == '\n' { // reset buffer
                    self.buffer = String::new();
                } else if self.check(CMD_APPEND, key) {
                    file_handle.content[display_handle.center_y].insert(display_handle.center_x, ' ');
                    self.mode = Mode::Append;
                } else if self.check(CMD_BACKSPACE_LEFT, key) {
                    file_handle.content[display_handle.center_y].remove(display_handle.center_x);
                    if display_handle.center_x > 0 {
                        display_handle.center_x -= 1;
                    }
                } else if self.check(CMD_BACKSPACE_RIGHT, key) {
                    file_handle.content[display_handle.center_y].remove(display_handle.center_x);
                    if display_handle.center_x > file_handle.content[display_handle.center_y].len() {
                        display_handle.center_x = file_handle.content[display_handle.center_y].len();
                    }
                } else if self.check(CMD_CENTER_DOWN, key) {
                    if display_handle.center_y != file_handle.content.len() {
                        display_handle.center_y += 1;
                        if display_handle.center_x > file_handle.content[display_handle.center_y].len() {
                            display_handle.center_x = file_handle.content[display_handle.center_y].len();
                        }
                    }
                } else if self.check(CMD_CENTER_END_FILE, key) {
                    display_handle.center_y = file_handle.content.len() - 1;
                    display_handle.center_x = file_handle.content[display_handle.center_y].len();
                } else if self.check(CMD_CENTER_END_LINE, key) {
                    display_handle.center_x = file_handle.content[display_handle.center_y].len();
                } else if self.check(CMD_CENTER_LEFT, key) {
                    if display_handle.center_x != 0 {
                        display_handle.center_x -= 1;
                    }
                } else if self.check(CMD_CENTER_RIGHT, key) {
                    if display_handle.center_x < file_handle.content[display_handle.center_y].len() {
                        display_handle.center_x += 1;
                    }
                } else if self.check(CMD_CENTER_START_FILE, key) {
                    display_handle.center_x = 0;
                    display_handle.center_y = 0;
                } else if self.check(CMD_CENTER_START_LINE, key) {
                    display_handle.center_x = 0;
                } else if self.check(CMD_CENTER_UP, key) {
                    if display_handle.center_y != 0 {
                        display_handle.center_y -= 1;
                        if display_handle.center_x >= file_handle.content[display_handle.center_y].len() {
                            display_handle.center_x = file_handle.content[display_handle.center_y].len() - 1;
                        }
                    }
                } else if self.check(CMD_NEWLINE, key) {
                    file_handle.content.insert(display_handle.center_y + 1, file_handle.content[display_handle.center_y][display.center_x..]);
                    file_handle.content[display_handle.center_y] = file_handle.content[display_handle.center_y][..display_handle.center_x];
                    display_handle.center_x = 0;
                    display_handle.center_y += 1;
                } else if self.check(CMD_QUIT, key) {
                    return false;
                } else if self.check(CMD_REPLACE, key) {
                    let mut row: Vec<char> = file_handle.content[display_handle.center_y].chars().collect();
                    row[display_handle.center_x] = ' ';
                    file_handle.content[display_handle.center_y] = row.into_iter().collect();
                    self.mode = Mode::Replace;
                } else if self.check(CMD_THEME_CHANGE_TO_ONE_DARK, key) {
                    display_handle.theme = display::Theme::one_dark();
                } else if self.check(CMD_THEME_CHANGE_TO_ONE_LIGHT, key) {
                    display_handle.theme = display::Theme::one_light();
                }
            }
            Mode::Replace => {
                if key == '\n' {
                    file_handle.content[display_handle.center_y].remove(display_handle.center_x);
                    self.buffer.push(key);
                    self.history += &self.buffer;
                } else {
                    let mut row: Vec<char> = file_handle.content[display_handle.center_y].chars().collect();
                    row[display_handle.center_x] = key;
                    file_handle.content[display_handle.center_y] = row.into_iter().collect();
                    self.buffer.push(key);
                    self.history += &self.buffer;
                }
                self.mode = Mode::Default;
            }
        }

        true
    }
}
