#![allow(clippy::collapsible_match)]
use resast::prelude::{Program as ParsedProgram, *};
use ressa::Parser;

use crate::ir::statement::*;

pub fn parse_program(input: &str) -> Scope {
    // parse
    let mut parser = Parser::new(input).unwrap();
    let script = parser.parse().expect("Failed to parse");

    // programmatically construct IR from AST
    let mut program = Scope::named("Script");
    if let ParsedProgram::Script(ast_nodes) = script {
        for node in ast_nodes {
            match node {
                ProgramPart::Decl(dec) => match dec {
                    Decl::Var(_, mut dec) => {
                        for sub_dec in dec.drain(..) {
                            program.append(sub_dec.into());
                        }
                    }
                    Decl::Func(f) => {
                        program.append(f.into());
                    }
                    _ => unimplemented!(),
                },
                ProgramPart::Stmt(s) => match s {
                    Stmt::Expr(e) => match e {
                        Expr::Call(c) => {
                            program.append(c.into());
                        }
                        Expr::Ident(i) => {
                            program.append(i.into());
                        }
                        _ => unimplemented!(),
                    },
                    _ => {
                        unimplemented!()
                    }
                },
                _ => unimplemented!(),
            }
        }
    }
    program
}
