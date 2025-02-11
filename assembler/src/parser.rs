pub struct Parser {
    file_name: String,
}

impl Parser {
    pub fn new(file_name: String) -> Self {
        // ファイルを開く
        Self { file_name }
    }
}
