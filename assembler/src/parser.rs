use regex::Regex;
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct Parser {
    pub lines: Vec<String>,
    now_line: usize,
    instruction: String,
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
            instruction: String::new(),
        }
    }

    fn has_more_line(&self) -> bool {
        let line_counts = self.lines.len();
        self.now_line < line_counts
    }

    /// 次の命令を読み込む
    fn advance(&mut self) {
        let comment = Regex::new(r"\s*//.*").unwrap();
        while self.has_more_line() {
            let now_line = &self.lines[self.now_line];
            let now_line = now_line.trim_start();

            let is_comment = comment.captures(now_line).is_some();
            if now_line.is_empty() || is_comment {
                self.now_line += 1;
                continue;
            }

            self.instruction = self.lines[self.now_line].clone();
            self.now_line += 1;
            break;
        }
    }

    /// 現在の命令タイプを返す
    fn instruction_type(&self) -> InstructionType {
        let a_instruction = Regex::new(r"\s*@\d+\s*").unwrap();
        let l_instruction = Regex::new(r"\s*\(\w+\)\s*").unwrap();

        if a_instruction.captures(&self.instruction).is_some() {
            InstructionType::AInstruction
        } else if l_instruction.captures(&self.instruction).is_some() {
            InstructionType::LInstruction
        } else {
            InstructionType::CInstruction
        }
    }

    fn symbol(&self) -> String {
        // 現在の命令で分岐
        let instruction_type = self.instruction_type();
        match instruction_type {
            InstructionType::AInstruction => {
                let symbol = Regex::new(r"\s*@(\d+)\s*").unwrap();
                let Some(symbol) = symbol.captures(&self.instruction) else {
                    return "".to_string();
                };
                symbol[1].to_string()
            }
            InstructionType::CInstruction => todo!(),
            InstructionType::LInstruction => todo!(),
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
            instruction: String::new(),
        };

        assert!(parser.has_more_line());
    }

    #[test]
    fn has_more_line_return_false() {
        let parser = Parser {
            lines: vec!["test".to_string()],
            now_line: 1,
            instruction: String::new(),
        };

        assert!(!parser.has_more_line());
    }

    #[test]
    fn advance_return_next_operation() {
        let mut parser = Parser {
            lines: vec!["    @99".to_string()],
            now_line: 0,
            instruction: String::new(),
        };

        parser.advance();

        assert_eq!(parser.now_line, 1);
        assert_eq!(parser.instruction, "    @99".to_string());
    }

    #[test]
    fn advance_ignore_space() {
        let mut parser = Parser {
            lines: vec![" ".to_string(), "    @99".to_string()],
            now_line: 0,
            instruction: String::new(),
        };

        parser.advance();

        assert_eq!(parser.now_line, 2);
        assert_eq!(parser.instruction, "    @99".to_string());
    }

    #[test]
    fn advance_ignore_comment() {
        let mut parser = Parser {
            lines: vec!["  // this is comment".to_string(), "    @99".to_string()],
            now_line: 0,
            instruction: String::new(),
        };

        parser.advance();

        assert_eq!(parser.now_line, 2);
        assert_eq!(parser.instruction, "    @99".to_string());
    }

    #[test]
    fn instruction_type_return_a_instruction() {
        let mut parser = Parser {
            lines: vec!["    @99".to_string()],
            now_line: 1,
            instruction: "    @99".to_string(),
        };

        let instruction_type = parser.instruction_type();

        assert_eq!(instruction_type, InstructionType::AInstruction);
    }

    #[test]
    fn instruction_type_return_l_instruction() {
        let mut parser = Parser {
            lines: vec!["    (WHITE)".to_string()],
            now_line: 1,
            instruction: "    (WHITE)".to_string(),
        };

        let instruction_type = parser.instruction_type();

        assert_eq!(instruction_type, InstructionType::LInstruction);
    }

    #[test]
    fn instruction_type_return_c_instruction() {
        let mut parser = Parser {
            lines: vec!["    D=D+1;JLE".to_string()],
            now_line: 1,
            instruction: "    D=D+1;JLE".to_string(),
        };

        let instruction_type = parser.instruction_type();

        assert_eq!(instruction_type, InstructionType::CInstruction);
    }

    #[test]
    fn symbol_return_a_instruction() {
        let mut parser = Parser {
            lines: vec!["    @99".to_string()],
            now_line: 1,
            instruction: "    @99".to_string(),
        };

        let instruction_type = parser.symbol();

        assert_eq!(instruction_type, "99");
    }
}
