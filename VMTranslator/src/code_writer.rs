use std::fs::File;

pub struct CodeWriter {
    file: File,
}

impl CodeWriter {
    pub fn new(file_name_base: &str) -> Self {
        // ファイルの作成
        let file = File::create(format!("{}.asm", file_name_base)).unwrap();
        Self { file }
    }
}
