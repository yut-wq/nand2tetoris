use assembler::run;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    run(file_name);
}
