use std::io::Write;
use crate::parser::{PrintStatement, PrintStatement_literal, Program, Statement};


fn execute_print(s: &PrintStatement, stdout: &mut impl Write) {
    match &s.literal {
        PrintStatement_literal::IntegerLiteral(i) => {
            let _ = stdout.write_all(format!("{}", i).as_bytes());
        }
        PrintStatement_literal::StringLiteral(s) => {
            let _ = stdout.write_all(s.body.as_bytes());
        }
    };
}

pub fn execute(program: Program, stdout: &mut impl Write) {

    for s in program.body.statements  {
        match s {
            Statement::EndStatement(_) => {}
            Statement::PrintStatement(s) => execute_print(&s, stdout)
        }
    }
}