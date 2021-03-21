mod ast;
mod runtime;
mod util;

use ast::*;
use runtime::Interpreter;

fn main() {
    let mut program = Block::new("Program");

    let mut block = Block::new("FunctionBlock");

    block.append(ReturnStatement::new(BinaryExpression::new(
        BinaryOp::Add,
        Literal::new(Value::Number(1.5f64)),
        Literal::new(Value::Number(3.5f64)),
    )));

    program.append(FunctionDeclaration::new("add".to_owned(), block));

    program.append(ExpressionStatement::new(CallExpression::new(
        "add".to_owned(),
    )));

    let mut interpreter = Interpreter::default();

    println!("{}", program.dump(0));

    let result = interpreter.run(program);

    println!("Output: {}", result);
}
