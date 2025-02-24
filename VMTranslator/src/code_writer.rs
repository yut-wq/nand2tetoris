use std::fs::File;

use crate::parser::CommandType;

pub struct CodeWriter {
    file: File,
}

impl CodeWriter {
    pub fn new(file_name_base: &str) -> Self {
        // ファイルの作成
        let file = File::create(format!("{}.asm", file_name_base)).unwrap();
        Self { file }
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: i32) {
        // pop
        // pop
        // 加算
        // push
    }

    /// 算術コマンドに対する書き込み処理
    pub fn write_arithmetic(&mut self, command: &str) {
        // pop
        // pop
        // 加算
        // push
    }
}
