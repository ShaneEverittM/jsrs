use javascript_rs::ir::{expression::*, ops::*, statement::*, IRNode};
use javascript_rs::runtime::{Interpreter, Value};

#[test]
fn test() {
    let mut program = Scope::named("Program");

    program.append(VariableDeclaration::boxed(
        "x",
        Literal::boxed(Value::Number(2.0f64)),
    ));

    let mut block = Scope::default();

    block.append(VariableDeclaration::boxed(
        "x",
        Literal::boxed(Value::Number(1.5f64)),
    ));

    block.append(ReturnStatement::boxed(BinaryExpression::boxed(
        BinaryOperator::Plus,
        Variable::boxed("x"),
        Literal::boxed(Value::Number(3.5f64)),
    )));

    program.append(FunctionDeclaration::boxed("add", block));

    program.append(ExpressionStatement::boxed(CallExpression::boxed(
        "add",
    )));

    let mut interpreter = Interpreter::default();

    println!("{}", program.dump(0));

    let result = interpreter.run(program);

    println!("Output: {}", result);

    assert_eq!(result, Value::Number(5f64));
}
