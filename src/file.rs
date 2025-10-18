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
    pub path: Option<String>,
    pub content: Vec<String>,
}

impl File {
    /// # CONTENT
    /// init File
    /// # ARGUMENT
    /// - _path: File::path value
    /// # RETURN VALUE
    /// inited File
    pub fn new(_path: Option<String>) -> Self {
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
        match self.path {
            Some(n) => {
                let fh = fs::File::open(&n)?;
                let fh_br =  io::BufReader::new(fh);
                for line in fh_br.lines() {
                    self.content.push(line?);
                }
            }
            None => {
                self.content.push(String::new());
            }
        }
        Ok(())
    }
}
