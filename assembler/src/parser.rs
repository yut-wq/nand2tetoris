use lazy_static::lazy_static;
use regex::Regex;
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

lazy_static! {
    static ref COMMENT: Regex = Regex::new(r"\s*//.*").unwrap();
    static ref A_INSTRUCTION: Regex = Regex::new(r"\s*@\S+\s*").unwrap();
    static ref L_INSTRUCTION: Regex = Regex::new(r"\s*\(\S+\)\s*").unwrap();
    static ref A_INSTRUCTION_SYMBOL: Regex = Regex::new(r"\s*@(\S+)\s*").unwrap();
    static ref L_INSTRUCTION_SYMBOL: Regex = Regex::new(r"\s*\((\S+)\)\s*").unwrap();
    static ref DEST: Regex = Regex::new(r"\s*(\w+)\s*\=.*").unwrap();
    static ref COMP: Regex = Regex::new(r"\s*(\w+\s*=\s*|)([^\s;]+)(;.+|\s*)").unwrap();
    static ref JUMP: Regex = Regex::new(r".+;(\w+)").unwrap();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct Parser {
    lines: Vec<String>,
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

    pub fn has_more_line(&self) -> bool {
        let line_counts = self.lines.len();
        self.now_line < line_counts
    }

    /// 次の命令を読み込む
    pub fn advance(&mut self) {
        while self.has_more_line() {
            let now_line = &self.lines[self.now_line];
            let now_line = now_line.trim_start();

            let is_comment = COMMENT.captures(now_line).is_some();
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
    pub fn instruction_type(&self) -> InstructionType {
        if A_INSTRUCTION.captures(&self.instruction).is_some() {
            InstructionType::AInstruction
        } else if L_INSTRUCTION.captures(&self.instruction).is_some() {
            InstructionType::LInstruction
        } else {
            InstructionType::CInstruction
        }
    }

    pub fn symbol(&self) -> String {
        // 現在の命令で分岐
        let instruction_type = self.instruction_type();
        match instruction_type {
            InstructionType::AInstruction => {
                let Some(symbol) = A_INSTRUCTION_SYMBOL.captures(&self.instruction) else {
                    return "".to_string();
                };
                symbol[1].to_string()
            }
            InstructionType::CInstruction => todo!(),
            InstructionType::LInstruction => {
                let Some(symbol) = L_INSTRUCTION_SYMBOL.captures(&self.instruction) else {
                    return "".to_string();
                };
                symbol[1].to_string()
            }
        }
    }

    pub fn dest(&self) -> String {
        let instruction_type = self.instruction_type();
        match instruction_type {
            InstructionType::AInstruction => todo!(),
            InstructionType::LInstruction => todo!(),
            InstructionType::CInstruction => {
                let Some(dest) = DEST.captures(&self.instruction) else {
                    return "".to_string();
                };
                dest[1].to_string()
            }
        }
    }

    pub fn comp(&self) -> String {
        let instruction_type = self.instruction_type();
        match instruction_type {
            InstructionType::AInstruction => todo!(),
            InstructionType::LInstruction => todo!(),
            InstructionType::CInstruction => {
                let Some(comp) = COMP.captures(&self.instruction) else {
                    panic!("no comp. invalid.")
                };
                comp[2].to_string()
            }
        }
    }

    pub fn jump(&self) -> String {
        let instruction_type = self.instruction_type();
        match instruction_type {
            InstructionType::AInstruction => todo!(),
            InstructionType::LInstruction => todo!(),
            InstructionType::CInstruction => {
                let Some(jump) = JUMP.captures(&self.instruction) else {
                    return "".to_string();
                };
                jump[1].to_string()
            }
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
    fn advance_include_dot() {
        let mut parser = Parser {
            lines: vec!["(sys.init)".to_string(), "    @99".to_string()],
            now_line: 0,
            instruction: String::new(),
        };

        parser.advance();

        assert_eq!(parser.now_line, 1);
        assert_eq!(parser.instruction, "(sys.init)".to_string());
    }

    #[test]
    fn instruction_type_return_a_instruction() {
        let parser = Parser {
            lines: vec!["  @R0".to_string()],
            now_line: 1,
            instruction: "  @R0".to_string(),
        };

        let instruction_type = parser.instruction_type();

        assert_eq!(instruction_type, InstructionType::AInstruction);
    }

    #[test]
    fn instruction_type_return_l_instruction() {
        let parser = Parser {
            lines: vec!["    (WHITE)".to_string()],
            now_line: 1,
            instruction: "    (WHITE)".to_string(),
        };

        let instruction_type = parser.instruction_type();

        assert_eq!(instruction_type, InstructionType::LInstruction);
    }

    #[test]
    fn instruction_type_include_dot_return_l_instruction() {
        let parser = Parser {
            lines: vec!["    (ball.new)".to_string()],
            now_line: 1,
            instruction: "    (ball.new)".to_string(),
        };

        let instruction_type = parser.instruction_type();

        assert_eq!(instruction_type, InstructionType::LInstruction);
    }

    #[test]
    fn instruction_type_return_c_instruction() {
        let parser = Parser {
            lines: vec!["    D=D+1;JLE".to_string()],
            now_line: 1,
            instruction: "    D=D+1;JLE".to_string(),
        };

        let instruction_type = parser.instruction_type();

        assert_eq!(instruction_type, InstructionType::CInstruction);
    }

    #[test]
    fn symbol_return_a_instruction() {
        let parser = Parser {
            lines: vec!["    @99".to_string()],
            now_line: 1,
            instruction: "    @99".to_string(),
        };

        let symbol = parser.symbol();

        assert_eq!(symbol, "99");
    }

    #[test]
    fn symbol_include_dot_return_a_instruction() {
        let parser = Parser {
            lines: vec!["@sys.halt".to_string()],
            now_line: 1,
            instruction: "@sys.halt".to_string(),
        };

        let symbol = parser.symbol();

        assert_eq!(symbol, "sys.halt");
    }

    #[test]
    fn symbol_return_l_instruction() {
        let parser = Parser {
            lines: vec!["    (WHITE)".to_string()],
            now_line: 1,
            instruction: "    (WHITE)".to_string(),
        };

        let symbol = parser.symbol();

        assert_eq!(symbol, "WHITE");
    }

    #[test]
    fn symbol_include_dot_return_l_instruction() {
        let parser = Parser {
            lines: vec![r"(sys.init)".to_string()],
            now_line: 1,
            instruction: r"(sys.init)".to_string(),
        };

        let symbol = parser.symbol();

        assert_eq!(symbol, r"sys.init");
    }

    #[test]
    fn dest_return_dest() {
        let parser = Parser {
            lines: vec!["    D=D+1;JLE".to_string()],
            now_line: 1,
            instruction: "    D=D+1;JLE".to_string(),
        };

        let dest = parser.dest();

        assert_eq!(dest, "D");
    }

    #[test]
    fn dest_return_empty() {
        let parser = Parser {
            lines: vec!["    D;JMP".to_string()],
            now_line: 1,
            instruction: "    D;JMP".to_string(),
        };

        let dest = parser.dest();

        assert_eq!(dest, "");
    }

    #[test]
    fn comp_return_comp() {
        let parser = Parser {
            lines: vec!["    D=D+1;JLE".to_string()],
            now_line: 1,
            instruction: "    D=D+1;JLE".to_string(),
        };

        let comp = parser.comp();

        assert_eq!(comp, "D+1");
    }

    #[test]
    fn comp_no_dest() {
        let parser = Parser {
            lines: vec!["    D+1;JLE".to_string()],
            now_line: 1,
            instruction: "    D+1;JLE".to_string(),
        };

        let comp = parser.comp();

        assert_eq!(comp, "D+1");
    }

    #[test]
    fn comp_no_jmp() {
        let parser = Parser {
            lines: vec!["    D=A+1".to_string()],
            now_line: 1,
            instruction: "    D=A+1".to_string(),
        };

        let comp = parser.comp();

        assert_eq!(comp, "A+1");
    }

    #[test]
    fn jump_return_jmp() {
        let parser = Parser {
            lines: vec!["    D=D+1;JLE".to_string()],
            now_line: 1,
            instruction: "    D=D+1;JLE".to_string(),
        };

        let jump = parser.jump();

        assert_eq!(jump, "JLE");
    }

    #[test]
    fn jump_no_dest() {
        let parser = Parser {
            lines: vec!["    D+1;JLE".to_string()],
            now_line: 1,
            instruction: "    D+1;JLE".to_string(),
        };

        let jump = parser.jump();

        assert_eq!(jump, "JLE");
    }

    #[test]
    fn jump_no_jump() {
        let parser = Parser {
            lines: vec!["    D=D+1".to_string()],
            now_line: 1,
            instruction: "    D=D+1".to_string(),
        };

        let jump = parser.jump();

        assert_eq!(jump, "");
    }
}
