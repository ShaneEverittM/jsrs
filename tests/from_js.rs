use std::env;

use javascript_rs::prelude::*;

fn validate_output(file_name: &str, expected: Option<Value>) {
    let verbose = match env::var("VERBOSE") {
        Ok(v) => v == "1",
        Err(_) => false,
    };

    let input = std::fs::read_to_string(&format!("tests/input/{}.js", file_name)).unwrap();

    let program = parse_program(&input);

    if verbose {
        println!("{}", program.dump(0));
    }

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    if verbose {
        match result.as_ref() {
            None => {}
            Some(result) => { println!("Output: {}", result); }
        }
    }

    assert_eq!(result, expected);
}

#[test]
fn add_vars() {
    validate_output("add_vars", Some(Value::Number(5.5f64)));
}

#[test]
fn if_test() {
    validate_output("branches", Some(Value::Number(69f64)));
}

#[test]
fn if_no_block_test() {
    validate_output("statement_branches", Some(Value::Number(420f64)));
}

#[test]
fn string_test() {
    validate_output("string", Some(Value::String("Strings!".to_owned())));
}

#[test]
fn complicated() {
    validate_output("complicated", Some(Value::Number(6f64)));
}

#[test]
fn assignment() {
    validate_output("assignment", Some(Value::Number(7f64)));
}

#[test]
fn update() {
    validate_output("update", Some(Value::Number(6f64)));
}

#[test]
fn for_loop() {
    validate_output("for", Some(Value::Number(5f64)));
}

#[test]
fn break_test() {
    validate_output("break", Some(Value::Number(3f64)));
}

#[test]
fn return_test() {
    validate_output("return", Some(Value::Number(3f64)));
}

#[test]
fn complicated_v2() {
    validate_output("complicated2", Some(Value::Number(69f64)));
}

#[test]
fn empty_decl() {
    validate_output("empty_decl", Some(Value::Number(5f64)));
}
