use assembler::{parser::Parser, run};

fn main() {
    // run()
    let parser = Parser::new("test.txt");
    println!("{:?}", parser.lines);
}
