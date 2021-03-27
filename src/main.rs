use javascript_rs::prelude::*;

fn main() {
    let input = javascript_rs::util::get_input();

    let program = parse_program(&input);

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    match result {
        None => {}
        Some(result) => { println!("Output: {}", result); }
    }
}
