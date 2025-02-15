use code::Code;
use parser::Parser;
use regex::Regex;
use std::{collections::HashMap, fs::File, io::Write};
use symbol_table::SymbolTable;

mod code;
mod parser;
mod symbol_table;

pub fn run(file_name: &str) {
    // ファイルの作成
    let file_name_base = Regex::new(r"(.+)\..+").unwrap();
    let Some(file_name_base) = file_name_base.captures(file_name) else {
        panic!("invalid file name.");
    };
    let file_name_base = file_name_base[1].to_string();
    let mut file = File::create(format!("{}.hack", file_name_base)).unwrap();

    // 変数で使用可能なアドレス
    let keys = 16..0b0111_1111_1111_1111_u16;
    let values = [true; 0xffff];
    let key_value_pair = keys.zip(values);
    let mut available_address: HashMap<u16, bool> = key_value_pair.collect();

    // 1パス目
    let mut parser = Parser::new(file_name);
    let mut symbol_table = SymbolTable::new();
    let mut line_count = 0;
    while parser.has_more_line() {
        parser.advance();

        let instruction_type = parser.instruction_type();
        match instruction_type {
            parser::InstructionType::AInstruction => {
                line_count += 1;
            }
            parser::InstructionType::CInstruction => {
                line_count += 1;
            }
            parser::InstructionType::LInstruction => {
                let symbol = parser.symbol();
                let address = line_count;
                symbol_table.add_entry(symbol, address);
                // 使用アドレスを使用不可にする
                available_address.remove(&address);
            }
        }
    }
    println!("{:#?}", symbol_table);

    // 2パス目
    let mut parser = Parser::new(file_name);
    let mut bin_codes = String::new();
    let mut min_address = 16;
    while parser.has_more_line() {
        parser.advance();

        let instruction_type = parser.instruction_type();
        match instruction_type {
            parser::InstructionType::AInstruction => {
                let symbol = parser.symbol();
                match symbol.parse::<u16>() {
                    Ok(symbol) => {
                        let bin_code = format!("{:016b}\n", symbol);
                        bin_codes.push_str(&bin_code);
                    }
                    Err(_) => {
                        if symbol_table.contains(&symbol) {
                            let symbol = symbol_table.get_address(&symbol);
                            let bin_code = format!("{:016b}\n", symbol);
                            bin_codes.push_str(&bin_code);
                        } else {
                            // シンボルテーブルに追加する

                            // 利用可能な最小アドレスを取得する
                            while !available_address.contains_key(&min_address) {
                                min_address += 1;
                            }
                            symbol_table.add_entry(symbol, min_address);
                            let symbol = min_address;
                            available_address.remove(&min_address);
                            let bin_code = format!("{:016b}\n", symbol);
                            bin_codes.push_str(&bin_code);
                        }
                    }
                }
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
            parser::InstructionType::LInstruction => {}
        }
    }

    file.write_all(bin_codes.as_bytes()).unwrap();

    println!("finish");
}
