use resast::prelude::*;
use ressa::Parser;

use crate::ir::{marker::Statement, statement::*};

pub fn parse_var_decl(mut var_decl: Vec<VarDecl>) -> Vec<Box<dyn Statement>> {
    let mut statements = Vec::new();
    for sub_dec in var_decl.drain(..) {
        statements.push(sub_dec.into());
    }
    statements
}

pub fn parse_block(statements: Vec<ProgramPart>, block: &mut Block) {
    for part in statements {
        match part {
            ProgramPart::Decl(d) => match d {
                Decl::Var(VarKind::Let, dec) => block.append(parse_var_decl(dec)),
                Decl::Func(_) => unimplemented!("Nested functions not supported"),
                _ => unimplemented!("{:?} not allowed in this context", d),
            },
            ProgramPart::Stmt(s) => match s {
                Stmt::Var(v) => block.append(parse_var_decl(v)),
                _ => block.push(s.into()),
            },
            _ => unimplemented!(),
        }
    }
}

pub fn parse_program(input: &str) -> Block {
    // parse
    let mut parser = Parser::new(input).unwrap();
    let ast = parser.parse().expect("Failed to parse");

    // programmatically construct IR from AST
    let mut ir = Block::new(BlockType::Global);

    if let Program::Script(ast_nodes) = ast {
        for node in ast_nodes {
            match node {
                ProgramPart::Decl(dec) => match dec {
                    Decl::Var(VarKind::Let, mut dec) => {
                        for sub_dec in dec.drain(..) {
                            ir.push(sub_dec.into());
                        }
                    }
                    Decl::Func(f) => {
                        ir.push(f.into());
                    }
                    _ => panic!("Unsupported Declaration"),
                },
                ProgramPart::Stmt(s) => ir.push(s.into()),
                ProgramPart::Dir(_) => panic!("Directives not supported"),
            }
        }
    }
    ir
}
