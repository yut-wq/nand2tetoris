use std::fmt::Write;

use crate::parser::CommandType;

pub struct CodeWriter {
    file_name: String,
    file: String,
}

impl CodeWriter {
    pub fn new(file_name_base: &str) -> Self {
        // ファイルの作成
        let file = String::new();
        Self {
            file_name: file_name_base.to_string(),
            file,
        }
    }

    pub fn write_push_pop(&mut self, command: CommandType, segment: &str, index: u32) {
        match command {
            CommandType::Arithmetic
            | CommandType::Label
            | CommandType::Goto
            | CommandType::If
            | CommandType::Function
            | CommandType::Return
            | CommandType::Call => (),
            CommandType::Push => {
                let bin_codes = self.generate_push_codes(segment, index);
                self.file.write_str(&bin_codes).unwrap();
            }
            CommandType::Pop => {
                let bin_codes = self.generate_pop_codes(segment, index);
                self.file.write_str(&bin_codes).unwrap();
            }
        }
    }

    fn generate_push_codes(&self, segment: &str, index: u32) -> String {
        let mut bin_codes = String::new();
        match segment {
            "argument" => {
                bin_codes.push_str("@ARG\n");
                bin_codes.push_str("D=M\n");
                bin_codes.push_str(&format!("@{}\n", index));
                bin_codes.push_str("D=D+A\n");
                bin_codes.push_str("A=D\n");
                bin_codes.push_str("D=M\n");
            }
            "local" => {
                bin_codes.push_str("@LCL\n");
                bin_codes.push_str("D=M\n");
                bin_codes.push_str(&format!("@{}\n", index));
                bin_codes.push_str("D=D+A\n");
                bin_codes.push_str("A=D\n");
                bin_codes.push_str("D=M\n");
            }
            "static" => {
                bin_codes.push_str(&format!("@{}.{}\n", self.file_name, index));
                bin_codes.push_str("D=M\n");
            }
            "constant" => {
                bin_codes.push_str(&format!("@{}\n", index));
                bin_codes.push_str("D=A\n");
            }
            "this" => {
                bin_codes.push_str("@THIS\n");
                bin_codes.push_str("D=M\n");
                bin_codes.push_str(&format!("@{}\n", index));
                bin_codes.push_str("D=D+A\n");
                bin_codes.push_str("A=D\n");
                bin_codes.push_str("D=M\n");
            }
            "that" => {
                bin_codes.push_str("@THAT\n");
                bin_codes.push_str("D=M\n");
                bin_codes.push_str(&format!("@{}\n", index));
                bin_codes.push_str("D=D+A\n");
                bin_codes.push_str("A=D\n");
                bin_codes.push_str("D=M\n");
            }
            "pointer" => {
                match index {
                    0 => bin_codes.push_str("@THIS\n"),
                    1 => bin_codes.push_str("@THAT\n"),
                    _ => return String::new(),
                }
                bin_codes.push_str("D=M\n");
            }
            "temp" => {
                bin_codes.push_str(&format!("@{}\n", index + 5));
                bin_codes.push_str("D=M\n");
            }
            _ => return String::new(),
        };

        let push_codes = push_data_register();
        bin_codes.push_str(&push_codes);
        bin_codes
    }

    fn generate_pop_codes(&self, segment: &str, index: u32) -> String {
        let mut bin_codes = String::new();
        match segment {
            "argument" => {
                bin_codes.push_str(&decrement_sp());
                bin_codes.push_str("@ARG\n");
                bin_codes.push_str("D=M\n");
                bin_codes.push_str(&format!("@{}\n", index));
                bin_codes.push_str("D=D+A\n");
                bin_codes.push_str("@R13\n");
                bin_codes.push_str("M=D\n");
                bin_codes.push_str(&assign_sp_to_r13());
            }
            "local" => {
                bin_codes.push_str(&decrement_sp());
                bin_codes.push_str("@LCL\n");
                bin_codes.push_str("D=M\n");
                bin_codes.push_str(&format!("@{}\n", index));
                bin_codes.push_str("D=D+A\n");
                bin_codes.push_str("@R13\n");
                bin_codes.push_str("M=D\n");
                bin_codes.push_str(&assign_sp_to_r13());
            }
            "static" => {}
            "constant" => {}
            "this" => {
                bin_codes.push_str(&decrement_sp());
                bin_codes.push_str("@THIS\n");
                bin_codes.push_str("D=M\n");
                bin_codes.push_str(&format!("@{}\n", index));
                bin_codes.push_str("D=D+A\n");
                bin_codes.push_str("@R13\n");
                bin_codes.push_str("M=D\n");
                bin_codes.push_str(&assign_sp_to_r13());
            }
            "that" => {
                bin_codes.push_str(&decrement_sp());
                bin_codes.push_str("@THAT\n");
                bin_codes.push_str("D=M\n");
                bin_codes.push_str(&format!("@{}\n", index));
                bin_codes.push_str("D=D+A\n");
                bin_codes.push_str("@R13\n");
                bin_codes.push_str("M=D\n");
                bin_codes.push_str(&assign_sp_to_r13());
            }
            "pointer" => {
                bin_codes.push_str(&decrement_sp());
                match index {
                    0 => bin_codes.push_str("@THIS\n"),
                    1 => bin_codes.push_str("@THAT\n"),
                    _ => return String::new(),
                }
                bin_codes.push_str("D=M\n");
                bin_codes.push_str("@R13\n");
                bin_codes.push_str("M=D\n");
                bin_codes.push_str(&assign_sp_to_r13());
            }
            "temp" => {}
            _ => return String::new(),
        };

        bin_codes
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

/// スタックからpopした値をDレジスタに格納する。
/// 上記の処理を実行するアセンブリを生成する。
fn pop_data_register() -> String {
    let mut push_codes = String::new();

    // sp--
    push_codes.push_str("@SP\n");
    push_codes.push_str("M=M-1\n");

    // x = ram[sp]
    push_codes.push_str("@SP\n");
    push_codes.push_str("A=M\n");
    push_codes.push_str("D=M\n");

    push_codes
}

/// スタックポインタをデクリメントするアセンブリを生成する。
fn decrement_sp() -> String {
    let mut codes = String::new();
    codes.push_str("@SP\n");
    codes.push_str("M=M-1\n");
    codes
}

/// R13に格納されているアドレスにスタックポインタの値を格納するアセンブリを生成する。
/// R13は、popした値を格納するためのアドレスを格納するために使用される、一時保存用のレジスタ。
fn assign_sp_to_r13() -> String {
    let mut codes = String::new();
    codes.push_str("@SP\n");
    codes.push_str("A=M\n");
    codes.push_str("D=M\n");

    codes.push_str("@R13\n");
    codes.push_str("A=M\n");
    codes.push_str("M=D\n");

    codes
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
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Push, "argument", 1);
        let expect = r"@ARG
D=M
@1
D=D+A
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn push_local_2() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Push, "local", 2);
        let expect = r"@LCL
D=M
@2
D=D+A
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn push_this_3() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Push, "this", 3);
        let expect = r"@THIS
D=M
@3
D=D+A
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn push_that_4() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Push, "that", 4);
        let expect = r"@THAT
D=M
@4
D=D+A
A=D
D=M
@SP
A=M
M=D
@SP
M=M+1
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn push_pointer_1() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Push, "pointer", 1);
        let expect = r"@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn push_temp_7() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Push, "temp", 7);
        let expect = r"@12
D=M
@SP
A=M
M=D
@SP
M=M+1
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn push_constant_17() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Push, "constant", 17);
        let expect = r"@17
D=A
@SP
A=M
M=D
@SP
M=M+1
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn push_static_16() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::from("Foo"),
        };
        writer.write_push_pop(CommandType::Push, "static", 16);
        let expect = r"@Foo.16
D=M
@SP
A=M
M=D
@SP
M=M+1
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn pop_data_register_test() -> Result<(), Box<dyn std::error::Error>> {
        let code = pop_data_register();

        let expect = r"@SP
M=M-1
@SP
A=M
D=M
";
        assert_eq!(code, expect);

        Ok(())
    }

    #[test]
    fn pop_local_1() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Pop, "local", 1);
        let expect = r"@SP
M=M-1
@LCL
D=M
@1
D=D+A
@R13
M=D
@SP
A=M
D=M
@R13
A=M
M=D
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn pop_argument_2() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Pop, "argument", 2);
        let expect = r"@SP
M=M-1
@ARG
D=M
@2
D=D+A
@R13
M=D
@SP
A=M
D=M
@R13
A=M
M=D
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn pop_this_3() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Pop, "this", 3);
        let expect = r"@SP
M=M-1
@THIS
D=M
@3
D=D+A
@R13
M=D
@SP
A=M
D=M
@R13
A=M
M=D
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn pop_that_4() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Pop, "that", 4);
        let expect = r"@SP
M=M-1
@THAT
D=M
@4
D=D+A
@R13
M=D
@SP
A=M
D=M
@R13
A=M
M=D
";

        assert_eq!(writer.file, expect);

        Ok(())
    }

    #[test]
    fn pop_pointer_0() -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = CodeWriter {
            file: String::new(),
            file_name: String::new(),
        };
        writer.write_push_pop(CommandType::Pop, "pointer", 0);
        let expect = r"@SP
M=M-1
@THIS
D=M
@R13
M=D
@SP
A=M
D=M
@R13
A=M
M=D
";

        assert_eq!(writer.file, expect);

        Ok(())
    }
}
