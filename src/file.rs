//! # CONTENT
//! program to control file

use {
    std:: {
        fs,
        io:: {
            self,
            BufRead,
        },
    },
};

/// # CONTENT
/// file handle
/// # FIELD
/// - path: file path
/// - content: file content
pub struct File {
    pub path: String,
    pub content: Vec<String>,
}

impl File {
    /// # CONTENT
    /// init File
    /// # ARGUMENT
    /// - _path: File::path value
    /// # RETURN VALUE
    /// inited File
    pub fn new(_path: String) -> Self {
        Self {
            path: _path,
            content: Vec::new(), // set in File::read()
        }
    }

    /// # CONTENT
    /// get File::content
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// File::content
    pub fn get_content(&self) -> &Vec<String> { &self.content }

    /// # CONTENT
    /// get file content
    /// # ARGUMENT
    /// none
    /// # RETURN VALUE
    /// Ok(()): ok
    pub fn read(&mut self) -> io::Result<()> {
        let fh = match fs::File::open(&self.path) {
            Ok(file) => file,
            Err(e) => {
                match e.kind() {
                    io::ErrorKind::NotFound => {
                        return Ok(());
                    }
                    _ => {
                        return Err(e);
                    }
                }
            }
        };
        let fh_br =  io::BufReader::new(fh);
        for line in fh_br.lines() {
            self.content.push(line?);
        }
        Ok(())
    }
}
