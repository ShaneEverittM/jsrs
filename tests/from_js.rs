use javascript_rs::prelude::*;

mod common;

#[test]
fn add_vars() {
    let input = common::get_test_input("tests/input/add_vars.js");

    let program = parse_program(&input, "add_vars.js");

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::Number(5.5f64));

    println!("Output: {}", result);
    println!();
}

#[test]
fn if_test() {
    let input = common::get_test_input("tests/input/branches.js");

    let program = parse_program(&input, "branches.js");

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::Number(420f64));

    println!("Output: {}", result);
    println!();
}


#[test]
fn if_no_block_test() {
    let input = common::get_test_input("tests/input/statement_branches.js");

    let program = parse_program(&input, "statement_branches.js");

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::Number(420f64));

    println!("Output: {}", result);
    println!();
}

#[test]
fn string_test() {
    let input = common::get_test_input("tests/input/string.js");

    let program = parse_program(&input, "string.js");

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::String(String::from("Strings!")));

    println!("Output: {}", result);
    println!();
}

#[test]
fn complicated() {
    let input = common::get_test_input("tests/input/complicated.js");

    let program = parse_program(&input, "complicated.js");

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::Number(6f64));

    println!("Output: {}", result);
    println!();
}

#[test]
fn assignment() {
    let input = common::get_test_input("tests/input/assignment.js");

    let program = parse_program(&input, "assignment.js");

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::Number(7f64));

    println!("Output: {}", result);
    println!();
}

#[test]
fn update() {
    let input = common::get_test_input("tests/input/update.js");

    let program = parse_program(&input, "update.js");

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::Number(6f64));

    println!("Output: {}", result);
    println!();
}
