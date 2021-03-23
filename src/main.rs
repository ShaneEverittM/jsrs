use javascript_rs::ir::IRNode;
use javascript_rs::parse::*;
use javascript_rs::runtime::Interpreter;
use std::io::Read;

fn main() {
    let file_name = std::env::args().nth(1);
    let mut buffer = String::new();
    match file_name {
        None => {
            // read from stdin
            let stdin = std::io::stdin();
            let mut lock = stdin.lock();
            lock.read_to_string(&mut buffer).unwrap();
        }
        Some(file_name) => {
            // read from file
            buffer = std::fs::read_to_string(file_name).unwrap();
        }
    }

    let program = parse_program(&buffer);

    println!("{}", program.dump(0));
    // construct interpreter
    let mut interpreter = Interpreter::default();
    // output: interpreter.run(program)
    let result = interpreter.run(program);

    println!("Output: {}", result);
}
