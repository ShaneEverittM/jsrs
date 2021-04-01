use std::string::ToString;

use javascript_rs::prelude::*;

fn main() {
    let input = javascript_rs::util::get_input();

    let program = parse_program(&input);

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    match result {
        Err(e) => {
            println!("Threw Exception: {}", e.to_string());
        }
        Ok(result) => {
            println!("Output: {}", result);
        }
    }
}
