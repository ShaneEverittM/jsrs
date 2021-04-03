use std::{env, string::ToString};

use jsrs::prelude::*;

fn validate_output(file_name: &str, expected: Result<Value, Exception>) {
    let verbose = match env::var("VERBOSE") {
        Ok(v) => v == "1",
        Err(_) => false,
    };

    let input = std::fs::read_to_string(&format!("tests/input/{}.js", file_name)).unwrap();

    let program = parse_program(&input);

    if verbose {
        println!("{}", program.dump(0));
    }

    let mut interpreter = Interpreter::new();

    let result = interpreter.run(program);

    if verbose {
        match result.as_ref() {
            Err(e) => {
                println!("Threw Exception: {}", e.to_string());
            }
            Ok(result) => {
                println!("Output: {}", result);
            }
        }
    }

    assert_eq!(result, expected);
}

#[test]
fn add_vars() {
    validate_output("add_vars", Ok(Value::Number(5.5f64)));
}

#[test]
fn if_test() {
    validate_output("branches", Ok(Value::Number(69f64)));
}

#[test]
fn if_no_block_test() {
    validate_output("statement_branches", Ok(Value::Number(420f64)));
}

#[test]
fn string_test() {
    validate_output("string", Ok(Value::String("Strings!".to_owned())));
}

#[test]
fn complicated() {
    validate_output("complicated", Ok(Value::Number(6f64)));
}

#[test]
fn assignment() {
    validate_output("assignment", Ok(Value::Number(7f64)));
}

#[test]
fn update() {
    validate_output("update", Ok(Value::Number(6f64)));
}

#[test]
fn for_loop() {
    validate_output("for", Ok(Value::Number(5f64)));
}

#[test]
fn break_test() {
    validate_output("break", Ok(Value::Number(3f64)));
}

#[test]
fn return_test() {
    validate_output("return", Ok(Value::Number(3f64)));
}

#[test]
fn complicated_v2() {
    validate_output("complicated2", Ok(Value::Number(50f64)));
}

#[test]
fn empty_decl() {
    validate_output("empty_decl", Ok(Value::Number(5f64)));
}

#[test]
fn params() {
    validate_output("parameters", Ok(Value::Number(3f64)));
}

#[test]
fn missing_params() {
    validate_output("missing_parameter", Ok(Value::Number(5f64)));
}

#[test]
fn recursion() {
    validate_output("recursion", Ok(Value::Number(7f64)));
}

#[test]
fn aliased_go() {
    validate_output("aliased_go", Ok(Value::Number(3f64)));
}

#[test]
fn built_in() {
    validate_output("built_in", Ok(Value::Undefined));
}

#[test]
fn exception() {
    validate_output("exception", Err(Exception::ReferenceError("x".to_owned())));
}

#[test]
fn member() {
    validate_output("member", Err(Exception::TypeError("Value is not an object".to_owned())));
}

// #[test]
// fn objects() {
//     validate_output("object", Err(Exception::ReferenceError("x".to_owned())));
// }
