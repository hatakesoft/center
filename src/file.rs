use {
    std:: {
        fs,
        io:: {
            self,
            BufRead,
        },
    },
};

pub struct File {
    path: String,
    content: Vec<String>,
}

impl File {
    pub fn new(_path: String) -> Self {
        Self {
            path: _path,
            content: Vec::new(),
        }
    }

    pub fn get_content(&self) -> &Vec<String> { &self.content }

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
