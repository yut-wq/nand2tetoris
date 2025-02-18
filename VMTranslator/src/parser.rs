use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::File, io::Read};

lazy_static! {
    static ref COMMENT: Regex = Regex::new(r"\s*//.*").unwrap();
}

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

    pub fn advance(&mut self) {
        while self.has_more_lines() {
            let now_line = &self.lines[self.now_line];
            let now_line = now_line.trim_start();

            let is_comment = COMMENT.captures(now_line).is_some();
            if now_line.is_empty() || is_comment {
                self.now_line += 1;
                continue;
            }

            self.command = self.lines[self.now_line].clone();
            self.now_line += 1;
            break;
        }
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

    #[test]
    fn advance_ignore_space() {
        let mut parser = Parser {
            lines: vec![" ".to_string(), "push local 2".to_string()],
            now_line: 0,
            command: String::new(),
        };

        parser.advance();

        assert_eq!(parser.now_line, 2);
        assert_eq!(parser.command, "push local 2".to_string());
    }

    #[test]
    fn advance_ignore_comment() {
        let mut parser = Parser {
            lines: vec![
                "  // this is comment".to_string(),
                "push local 2".to_string(),
            ],
            now_line: 0,
            command: String::new(),
        };

        parser.advance();

        assert_eq!(parser.now_line, 2);
        assert_eq!(parser.command, "push local 2".to_string());
    }
}
