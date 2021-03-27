use std::fmt::Debug;

use crate::ir::IrNode;

pub trait Statement: IrNode + Debug + StatementClone {}

pub trait Expression: IrNode + Debug + ExpressionClone {}

pub trait BlockStatement: IrNode + Debug + Statement {}

pub trait Declaration: Statement {}

impl<T> Statement for T where T: Declaration {}

pub trait StatementClone {
    fn clone_box(&self) -> Box<dyn Statement>;
}

pub trait ExpressionClone {
    fn clone_box(&self) -> Box<dyn Expression>;
}

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

impl Clone for Box<dyn Statement> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Clone for Box<dyn Expression> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
