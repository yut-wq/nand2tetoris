use std::{fs::File, io::Read};

pub struct Parser {
    pub lines: Vec<String>,
}

impl Parser {
    /// ファイルを開く
    pub fn new(file_name: &str) -> Self {
        // file open
        let mut file = File::open(file_name).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).unwrap();
        let file_contents = file_contents.lines();

        // 行に分割
        let mut lines = vec![];
        for line in file_contents {
            lines.push(line.to_string());
        }

        Self { lines }
    }
    // fn has_more_line(&self) -> bool {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_open_file() {}
}
