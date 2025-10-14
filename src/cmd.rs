//! # CONTENT
//! cmd-related programs

/// # CONTENT
/// cmd info
/// # FIELD
/// - cmd: cmd history
pub struct Cmd {
    history: String,
    buffer: String,
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
            cmd: String::new(),
        }
    }

    /// # CONTENT
    /// execute cmd
    /// # ARGUMENT
    /// - key: enterd key event
    /// # RETURN VALUE
    /// - true: continue program
    /// - false: quit program
    pub fn key(&mut self, key: char) -> bool {
        if key == 'q' {
            // quit program
            buffer.push(key);
            history.push(key);
            return false;
        } else {
            return true;
        }
    }
}
