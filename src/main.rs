use javascript_rs::ir::IRNode;
use javascript_rs::parse::*;
use javascript_rs::runtime::Interpreter;

fn main() {
    let (input, file_name) = javascript_rs::util::get_input();

    let program = parse_program(&input, &file_name);

    println!("{}", program.dump(0));
    // construct interpreter
    let mut interpreter = Interpreter::default();
    // output: interpreter.run(program)
    let result = interpreter.run(program);

    println!("Output: {}", result);
}
