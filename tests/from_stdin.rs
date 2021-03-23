use javascript_rs::ir::IRNode;
use javascript_rs::parse::parse_program;
use javascript_rs::runtime::{Interpreter, Value};
use std::io::Read;

#[test]
fn from_input() {
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();
    let mut buffer = String::new();
    lock.read_to_string(&mut buffer).unwrap();

    let program = parse_program(&buffer);

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::Boolean(true));

    println!("Output: {}", result);
}

#[test]
fn if_test() {
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();
    let mut buffer = String::new();
    lock.read_to_string(&mut buffer).unwrap();

    let program = parse_program(&buffer);

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::Number(420f64));

    println!("Output: {}", result);
}
