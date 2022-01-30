use jsrs::prelude::*;

fn main() {
    let input = get_input();

    let program = parse_program(&input);

    println!("{}", program.print());

    let result = Interpreter::default().run(program);

    match result {
        Err(e) => println!("Threw Exception: {}", e),
        Ok(result) => println!("Output: {}", result),
    }
}
