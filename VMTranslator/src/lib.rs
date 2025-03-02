use code_writer::CodeWriter;
use parser::Parser;
use regex::Regex;

mod code_writer;
mod parser;

pub fn run(file_name: &str) {
    // parserの作成
    let mut parser = Parser::new(file_name);

    let file_name_base = Regex::new(r"(.+)\..+").unwrap();
    let Some(file_name_base) = file_name_base.captures(file_name) else {
        panic!("invalid file name.");
    };
    let file_name_base = file_name_base[1].to_string();

    // writerの作成
    let writer = CodeWriter::new(&file_name_base);

    // 反復処理
    while parser.has_more_lines() {
        parser.advance();

        let command = parser.command_type();
        match command {
            parser::CommandType::Arithmetic => todo!(),
            parser::CommandType::Push => todo!(),
            parser::CommandType::Pop => todo!(),
            parser::CommandType::Label => todo!(),
            parser::CommandType::Goto => todo!(),
            parser::CommandType::If => todo!(),
            parser::CommandType::Function => todo!(),
            parser::CommandType::Return => todo!(),
            parser::CommandType::Call => todo!(),
        }
    }
}
