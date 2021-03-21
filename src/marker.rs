use crate::ast_node::ASTNode;
use std::fmt::Debug;

pub trait StatementClone {
    fn clone_box(&self) -> Box<dyn Statement>;
}

pub trait ExpressionClone {
    fn clone_box(&self) -> Box<dyn Expression>;
}
pub trait Statement: ASTNode + Debug + StatementClone {}
pub trait Expression: ASTNode + Debug + ExpressionClone {}
pub trait Declaration: Statement + Debug {}

impl<T: 'static + Statement + Clone> StatementClone for T {
    fn clone_box(&self) -> Box<dyn Statement> {
        Box::new(self.clone())
    }
}

impl<T: 'static + Expression + Clone> ExpressionClone for T {
    fn clone_box(&self) -> Box<dyn Expression> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
