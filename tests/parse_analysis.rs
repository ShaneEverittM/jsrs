use javascript_rs::prelude::*;

#[test]
fn parse() {
    let input = std::fs::read_to_string("tests/input/parse_analysis.js").unwrap();

    let program = parse_program(&input);

    println!("{}", program.dump(0));
}
