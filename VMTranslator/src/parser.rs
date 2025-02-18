use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct Parser {
    lines: Vec<String>,
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
}
