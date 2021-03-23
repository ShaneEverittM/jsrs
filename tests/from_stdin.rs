use javascript_rs::ast::ASTNode;
use javascript_rs::parser::parse_program;
use javascript_rs::runtime::Interpreter;
use std::io::Read;

#[test]
fn main() {
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();
    let mut buffer = String::new();
    lock.read_to_string(&mut buffer).unwrap();

    let program = parse_program(&buffer);

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    println!("Output: {}", result);
}
