use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct Parser {
    lines: Vec<String>,
    now_line: usize,
    command: String,
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

        Self {
            lines,
            now_line: 0,
            command: String::new(),
        }
    }

    pub fn has_more_lines(&self) -> bool {
        let line_counts = self.lines.len();
        self.now_line < line_counts
    }

    pub fn advance(&self) {}
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn has_more_line_return_true() {
        let parser = Parser {
            lines: vec!["test".to_string()],
            now_line: 0,
            command: String::new(),
        };

        assert!(parser.has_more_lines());
    }

    #[test]
    fn has_more_line_return_false() {
        let parser = Parser {
            lines: vec!["test".to_string()],
            now_line: 1,
            command: String::new(),
        };

        assert!(!parser.has_more_lines());
    }

    #[test]
    fn advance_return_next_command() {
        let mut parser = Parser {
            lines: vec!["push local 2".to_string()],
            now_line: 0,
            command: String::new(),
        };

        parser.advance();

        assert_eq!(parser.now_line, 1);
        assert_eq!(parser.command, "push local 2".to_string());
    }
}
