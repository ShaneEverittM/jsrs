use javascript_rs::ast::{expression::*, ops::*, statement::*, ASTNode};
use javascript_rs::runtime::{Interpreter, Value};

#[test]
fn test() {
    let mut program = BlockStatement::new("Program");

    let mut block = BlockStatement::new("FunctionBlock");

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

    assert_eq!(result, Value::Number(5f64));
}