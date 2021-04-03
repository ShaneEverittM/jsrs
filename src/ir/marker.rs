//! For the following, we cannot derive Clone because then a trait object like Box<dyn Statement>
//! would have a method of return type Self, the exact type of which is lost when converted
//! from a concrete type to trait object. This is called not being "object safe" In order to
//! get around this, we constructed some helper clone types that clone into trait objects, but
//! require that anything they are implemented on implement Clone. For more info see the
//! [book](https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects)
use std::fmt::Debug;

use crate::ir::IrNode;

pub use jsrs_derive::{Expression, Statement, Declaration};

pub trait Statement: IrNode + Debug + StatementClone {}

pub trait Expression: IrNode + Debug + ExpressionClone {}

// Anything that is a Declaration is automatically a statement
pub trait Declaration: Statement {}
impl<T> Statement for T where T: Declaration {}

// The object safety workaround mentioned in the module docs.
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
