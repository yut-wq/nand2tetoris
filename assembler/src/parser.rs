use std::{fs::File, io::Read};

pub struct Parser {
    pub lines: Vec<String>,
    now_line: usize,
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

        Self { lines, now_line: 0 }
    }

    fn has_more_line(&self) -> bool {
        let line_counts = self.lines.len();
        self.now_line < line_counts
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_more_line_return_true() {
        let parser = Parser {
            lines: vec!["test".to_string()],
            now_line: 0,
        };

        assert!(parser.has_more_line());
    }

    #[test]
    fn has_more_line_return_false() {
        let parser = Parser {
            lines: vec!["test".to_string()],
            now_line: 1,
        };

        assert!(!parser.has_more_line());
    }
}
