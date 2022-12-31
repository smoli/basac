use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::Write;
use crate::interpreter::InterpreterError::TypeMismatch;
use crate::parser::{Add, Assignment, Assignment_value, Block, Div, Expression, Factor, FloatLiteral, Group, IntegerLiteral, Mul, NumberLiteral, NumberLiteral_value, PrintStatement, PrintStatement_list, Program, Statement, Sub, Term};

use crate::value::Value;
use crate::error::InterpreterError;
use crate::scope::Scope;


trait Execute {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) {}

    fn execute(&self, scope: &mut Scope) {}
}


fn execute_print(s: &PrintStatement, scope: &mut Scope, stdout: &mut impl Write) {
    // match &s.list {
    //     PrintStatement_list::Expression(e) => match &e {
    //         Expression_term::NumberLiteral(n) => match &n.value {
    //             NumberLiteral_value::FloatLiteral(f) => {
    //                 let _ = stdout.write_all(format!("{}", f).as_bytes());
    //             }
    //             NumberLiteral_value::IntegerLiteral(i) => {
    //                 let _ = stdout.write_all(format!("{}", i).as_bytes());
    //             }
    //         }
    //         Expression_term::VariableName(v) => {
    //             let value = scope.get(v).unwrap();
    //
    //             match value {
    //                 Value::String(s) => {
    //                     let _ = stdout.write_all(s.as_bytes());
    //                 }
    //                 Value::Integer(i) => {
    //                     let _ = stdout.write_all(format!("{}", i).as_bytes());
    //                 }
    //                 Value::Float(f) => {
    //                     let _ = stdout.write_all(format!("{}", f).as_bytes());
    //                 }
    //                 Value::Boolean(b) => {
    //                     let _ = stdout.write_all(format!("{}", b).as_bytes());
    //                 }
    //             }
    //         }
    //     }
    //     PrintStatement_list::StringLiteral(s) => {
    //         let _ = stdout.write_all(s.body.as_bytes());
    //     }
    // };
}

fn execute_assignment(assignment: &Assignment, scope: &mut Scope) {

    // let v = match &assignment.value {
    //     Assignment_value::NumberLiteral(n) => {
    //         match &n.value {
    //             NumberLiteral_value::FloatLiteral(f) => Value::Float(f.parse::<f64>().unwrap()),
    //             NumberLiteral_value::IntegerLiteral(i) => Value::Integer(i.parse().unwrap())
    //         }
    //     }
    //     Assignment_value::StringLiteral(s) => Value::String(s.body.clone())
    // };
    //
    // scope.set(&assignment.variable, v);
}

impl Execute for PrintStatement {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) {

    }
}

impl Execute for Assignment {
    fn execute(&self, scope: &mut Scope) {

    }
}


impl Execute for Statement {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) {
        match self {
            Statement::EndStatement(_) => {}
            Statement::PrintStatement(s) => s.execute_stdout(scope, stdout),
            Statement::Assignment(a) => a.execute(scope)
        }
    }
}


impl Execute for Block {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) {
        for i in 0..self.statements.len() {
            let s = self.statements.get(i).unwrap();
            s.execute_stdout(scope, stdout)
        }
    }
}


impl Program {
    pub fn execute(&self, stdout: &mut impl Write) {
        let mut scope = Scope::new();
        self.body.execute_stdout(&mut scope, stdout);
    }
}

