use javascript_rs::prelude::*;
use std::env;

fn validate_output(file_name: &str, expected: Value) {
    let verbose = match env::var("VERBOSE") {
        Ok(v) => v == "1",
        Err(_) => false,
    };

    let input = std::fs::read_to_string(&format!("tests/input/{}.js", file_name)).unwrap();

    let program = parse_program(&input, "add_vars.js");

    if verbose {
        println!("{}", program.dump(0));
    }

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    if verbose {
        println!("\nResult: {}", result);
    }

    assert_eq!(result, expected);
}

#[test]
fn add_vars() {
    validate_output("add_vars", Value::Number(5.5f64));
}

#[test]
fn if_test() {
    validate_output("branches", Value::Number(420f64));
}

#[test]
fn if_no_block_test() {
    validate_output("statement_branches", Value::Number(420f64));
}

#[test]
fn string_test() {
    validate_output("string", Value::String("Strings!".to_owned()));
}

#[test]
fn complicated() {
    validate_output("complicated", Value::Number(6f64));
}

#[test]
fn assignment() {
    validate_output("assignment", Value::Number(7f64));
}

#[test]
fn update() {
    validate_output("update", Value::Number(6f64));
}

#[test]
fn for_loop() {
    validate_output("for", Value::Number(5f64));
}
