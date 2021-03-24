use javascript_rs::prelude::*;

mod common;

#[test]
fn parse() {
    let input = common::get_test_input("tests/input/parse_analysis.js");

    let program = parse_program(&input, "parse_analysis.js");

    println!("{}", program.dump(0));
}