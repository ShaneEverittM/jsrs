use javascript_rs::prelude::*;

fn get_test_input(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()

}

#[test]
fn from_input() {
    let input = get_test_input("tests/input/branches.js");

    let program = parse_program(&input, "branches.js");

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::Number(420f64));

    println!("Output: {}", result);
    println!();
}

#[test]
fn if_test() {
    let input = get_test_input("tests/input/add_vars.js");

    let program = parse_program(&input, "add_vars.js");

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    assert_eq!(result, Value::Number(5.5f64));

    println!("Output: {}", result);
    println!();
}
