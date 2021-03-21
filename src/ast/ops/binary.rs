use std::fmt;

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOp::Add => f.write_str("+"),
        }
    }
}
