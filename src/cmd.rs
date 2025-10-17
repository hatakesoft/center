//! # CONTENT
//! cmd-related programs

use {
    crate:: {
        display,
        file,
    }
};

const CMD_CENTER_DOWN: &str = "j";
const CMD_CENTER_LEFT: &str = "h";
const CMD_CENTER_RIGHT: &str = "k";
const CMD_CENTER_UP: &str = "i";
const CMD_QUIT: &str = "q";

/// # CONTENT
/// cmd info
/// # FIELD
/// - cmd: cmd history
pub struct Cmd {
    history: String,
    pub buffer: String,
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
    pub fn key(&mut self, key: char, display_handle: &mut display::Display, file_handle: &file::File) -> bool {
        if self.check(CMD_CENTER_DOWN, key) {
            if display_handle.center_y != file_handle.content.len() {
                display_handle.center_y += 1;
                if display_handle.center_x >= file_handle.content[display_handle.center_y].len() {
                    display_handle.center_x = file_handle.content[display_handle.center_y].len() - 1;
                }
            }
        } else if self.check(CMD_CENTER_LEFT, key) {
            if display_handle.center_x < file_handle.content[display_handle.center_y].len() - 1 {
                display_handle.center_x += 1;
            }
        } else if self.check(CMD_CENTER_RIGHT, key) {
            if display_handle.center_x != 0 {
                display_handle.center_x -= 1;
            }
        } else if self.check(CMD_CENTER_UP, key) {
            if display_handle.center_y != 0 {
                display_handle.center_y -= 1;
                if display_handle.center_x >= file_handle.content[display_handle.center_y].len() {
                    display_handle.center_x = file_handle.content[display_handle.center_y].len() - 1;
                }
            }
        } else if self.check(CMD_QUIT, key) {
            return false;
        }

        true
    }
}
