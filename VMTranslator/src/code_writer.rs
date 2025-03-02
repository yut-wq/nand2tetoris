use std::{fs::File, io::Write};

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
        let mut bin_codes = String::new();
        match command {
            CommandType::Arithmetic
            | CommandType::Label
            | CommandType::Goto
            | CommandType::If
            | CommandType::Function
            | CommandType::Return
            | CommandType::Call => (),
            CommandType::Push => {
                let segment = match segment {
                    "argument" => todo!(),
                    "local" => {
                        // Dレジスタにxの値を置く
                        bin_codes.push_str("@LCL");
                        bin_codes.push_str("D=M");
                        bin_codes.push_str(&format!("@{}", index));
                        bin_codes.push_str("A=D+A");
                        bin_codes.push_str("D=M");
                    }
                    "static" => todo!(),
                    "constant" => {
                        // Dレジスタにxの値を置く
                        bin_codes.push_str(&format!("@{}", index));
                        bin_codes.push_str("D=A");
                    }
                    "this" => todo!(),
                    "that" => todo!(),
                    "pointer" => todo!(),
                    "temp" => todo!(),
                    _ => return,
                };

                let push_codes = push_data_register();
                bin_codes.push_str(&push_codes);

                self.file.write_all(bin_codes.as_bytes()).unwrap();
            }
            CommandType::Pop => todo!(),
        }
    }

    /// 算術コマンドに対する書き込み処理
    pub fn write_arithmetic(&mut self, command: &str) {
        // pop
        // pop
        // 加算
        // push
    }
}

/// push
/// push++
/// 上記の処理を実行するアセンブリを生成する。
fn push_data_register() -> String {
    let mut push_codes = String::new();

    // ram[sp] = x
    push_codes.push_str("@SP");
    push_codes.push_str("A=M");
    push_codes.push_str("M=D");

    // sp++
    push_codes.push_str("@SP");
    push_codes.push_str("M=M+1");

    push_codes
}
