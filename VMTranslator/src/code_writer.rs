use std::fmt::Write;

use crate::parser::CommandType;

pub struct CodeWriter {
    file: String,
}

impl CodeWriter {
    pub fn new(file_name_base: &str) -> Self {
        // ファイルの作成
        let file = String::new();
        Self { file }
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: u32) {
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
                let _segment = match segment {
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

                self.file.write_str(&bin_codes).unwrap();
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

/// Dレジスタの値をスタックにpushする。
/// 上記の処理を実行するアセンブリを生成する。
fn push_data_register() -> String {
    let mut push_codes = String::new();

    // ram[sp] = x
    push_codes.push_str("@SP\n");
    push_codes.push_str("A=M\n");
    push_codes.push_str("M=D\n");

    // sp++
    push_codes.push_str("@SP\n");
    push_codes.push_str("M=M+1\n");

    push_codes
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn push_data_register_test() -> Result<(), Box<dyn std::error::Error>> {
        let code = push_data_register();

        let expect = r"@SP
A=M
M=D
@SP
M=M+1
";
        assert_eq!(code, expect);

        Ok(())
    }

    #[test]
    fn push_argument_1() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
        };
        writer.write_push_pop(CommandType::Push, "argument", 1);

        assert_eq!(writer.file, "push ");

        Ok(())
    }
}
