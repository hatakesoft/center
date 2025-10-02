pub struct Cmd {
    cmd: String,
}

impl Cmd {
    pub fn new() -> Self {
        Self {
            cmd: String::new(),
        }
    }

    pub fn key(&mut self, key: char) -> bool {
        self.cmd.push(key);
        if key == 'q' {
            return false;
        } else {
            return true;
        }
    }
}
