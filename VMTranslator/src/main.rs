use std::env;
use vm_translator::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    run(file_name);
}
