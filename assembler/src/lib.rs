use code::Code;
use parser::Parser;
use regex::Regex;
use std::{fs::File, io::Write};

mod code;
mod parser;
mod symbol_table;

pub fn run(file_name: &str) {
    // parserの作成
    let mut parser = Parser::new(file_name);

    // ファイルの作成
    let file_name_base = Regex::new(r"(.+)\..+").unwrap();
    let Some(file_name_base) = file_name_base.captures(file_name) else {
        panic!("invalid file name.");
    };
    let file_name_base = file_name_base[1].to_string();
    let mut file = File::create(format!("{}.hack", file_name_base)).unwrap();

    // parse処理
    let mut bin_codes = String::new();
    while parser.has_more_line() {
        parser.advance();

        let instruction_type = parser.instruction_type();
        match instruction_type {
            parser::InstructionType::AInstruction => {
                let symbol = parser.symbol();
                let symbol: u16 = symbol.parse().unwrap();
                let bin_code = format!("{:016b}\n", symbol);
                bin_codes.push_str(&bin_code);
            }
            parser::InstructionType::CInstruction => {
                let comp = parser.comp();
                let comp = Code::comp(&comp);

                let dest = parser.dest();
                let dest = Code::dest(&dest);

                let jump = parser.jump();
                let jump = Code::jump(&jump);

                let bin_code = format!("111{}{}{}\n", comp, dest, jump);
                bin_codes.push_str(&bin_code);
            }
            parser::InstructionType::LInstruction => todo!(),
        }
    }

    file.write_all(bin_codes.as_bytes()).unwrap();

    println!("finish");
}
