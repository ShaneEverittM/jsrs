use javascript_rs::prelude::*;

fn main() {
    let (input, file_name) = javascript_rs::util::get_input();

    let program = parse_program(&input, &file_name);

    println!("{}", program.dump(0));

    let mut interpreter = Interpreter::default();

    let result = interpreter.run(program);

    println!("Output: {}", result);
}
