use std::{fs::File, io::Read};

enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

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
        if self.has_more_line() {
            self.instruction = self.lines[self.now_line].clone();
            self.now_line += 1;
        }
    }

    /// 現在の命令タイプを返す
    fn instruction_type(&self) -> InstructionType {
        todo!();
    }

    fn symbol(&self) -> String {
        todo!()
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
}
