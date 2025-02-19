use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::File, io::Read};

lazy_static! {
    static ref COMMENT: Regex = Regex::new(r"\s*//.*").unwrap();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
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

    pub fn command_type(&self) -> CommandType {
        let command = Regex::new(r"\s*(\w+)").unwrap();
        let Some(command) = command.captures(&self.command) else {
            panic!("invalid command. line: {}", self.now_line);
        };
        match &command[1] {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => {
                CommandType::Arithmetic
            }
            "push" => CommandType::Push,
            "pop" => CommandType::Pop,
            "return" => CommandType::Return,
            _ => todo!(),
        }
    }

    pub fn arg1(&self) -> Result<String> {
        let command_type = self.command_type();
        match command_type {
            CommandType::Arithmetic => Ok(self.command.clone()),
            CommandType::Push => todo!(),
            CommandType::Pop => todo!(),
            CommandType::Label => todo!(),
            CommandType::Goto => todo!(),
            CommandType::If => todo!(),
            CommandType::Function => todo!(),
            CommandType::Return => Err(anyhow!("command type is return.")),
            CommandType::Call => todo!(),
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

    #[test]
    fn command_type_return_push() {
        let parser = Parser {
            lines: vec!["push local 2".to_string()],
            now_line: 1,
            command: "push local 2".to_string(),
        };

        let command_type = parser.command_type();

        assert_eq!(command_type, CommandType::Push);
    }

    #[test]
    fn command_type_return_arithmetic() {
        let parser = Parser {
            lines: vec!["neg".to_string()],
            now_line: 1,
            command: "neg".to_string(),
        };

        let command_type = parser.command_type();

        assert_eq!(command_type, CommandType::Arithmetic);
    }

    #[test]
    fn command_type_return_return() {
        let parser = Parser {
            lines: vec!["return".to_string()],
            now_line: 1,
            command: "return".to_string(),
        };

        let command_type = parser.command_type();

        assert_eq!(command_type, CommandType::Return);
    }

    #[test]
    fn arg1_return_return_error() {
        let parser = Parser {
            lines: vec!["return".to_string()],
            now_line: 1,
            command: "return".to_string(),
        };

        let result = parser.arg1();

        assert!(result.is_err());
    }

    #[test]
    fn arg1_add_return_add() -> Result<()> {
        let parser = Parser {
            lines: vec!["add".to_string()],
            now_line: 1,
            command: "add".to_string(),
        };

        let arg1 = parser.arg1()?;

        assert_eq!(arg1, "add");
        Ok(())
    }

    #[test]
    fn arg1_not_return_not() -> Result<()> {
        let parser = Parser {
            lines: vec!["not".to_string()],
            now_line: 1,
            command: "not".to_string(),
        };

        let arg1 = parser.arg1()?;

        assert_eq!(arg1, "not");
        Ok(())
    }

    #[test]
    fn arg1_push_return_local() -> Result<()> {
        let parser = Parser {
            lines: vec!["push local 2".to_string()],
            now_line: 1,
            command: "push local 2".to_string(),
        };

        let arg1 = parser.arg1()?;

        assert_eq!(arg1, "local");
        Ok(())
    }
}
