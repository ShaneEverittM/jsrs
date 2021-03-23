use javascript_rs::ast::ASTNode;
use javascript_rs::parser::parse_program;
use javascript_rs::runtime::Interpreter;
use std::io::Read;

fn main() {
    // read from stdin
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();
    let mut buffer = String::new();
    lock.read_to_string(&mut buffer).unwrap();

    let program = parse_program(&buffer);

    println!("{}", program.dump(0));
    // construct interpreter
    let mut interpreter = Interpreter::default();
    // output: interpreter.run(program)
    let result = interpreter.run(program);

    println!("Output: {}", result);
}
