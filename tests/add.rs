use javascript_rs::ast::{expression::*, ops::*, statement::*, ASTNode};
use javascript_rs::runtime::{Interpreter, Value};

#[test]
fn test() {
    let mut program = Scope::named("Program");

    let mut block = Scope::default();

    block.append(ReturnStatement::boxed(BinaryExpression::boxed(
        BinaryOp::Add,
        Literal::boxed(Value::Number(1.5f64)),
        Literal::boxed(Value::Number(3.5f64)),
    )));

    program.append(FunctionDeclaration::boxed("add".to_owned(), block));

    program.append(ExpressionStatement::boxed(CallExpression::boxed(
        "add".to_owned(),
    )));

    let mut interpreter = Interpreter::default();

    println!("{}", program.dump(0));

    let result = interpreter.run(program);

    println!("Output: {}", result);

    assert_eq!(result, Value::Number(5f64));
}
