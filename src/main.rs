use crate::binary_expression::BinaryExpression;
use crate::binary_op::BinaryOp;
use crate::call_expression::CallExpression;
use crate::expression_statement::ExpressionStatement;
use crate::function_declaration::FunctionDeclaration;
use crate::interpreter::Interpreter;
use crate::literal::Literal;
use crate::return_statement::ReturnStatement;
use crate::value::Value;

mod ast_node;
mod binary_expression;
mod binary_op;
mod call_expression;
mod expression_statement;
mod function_declaration;
mod interpreter;
mod literal;
mod marker;
mod object;
mod return_statement;
mod scope_node;
mod value;
mod function;

fn main() {
    let mut program = scope_node::Block::new();

    let mut block = scope_node::Block::new();

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

    let result = interpreter.run(program);

    dbg!(result);
}
