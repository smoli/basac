use std::io::Write;
use crate::expression::Compute;
use crate::parser::{Assignment, Assignment_value, Block, PrintListItem_value, PrintStatement, Program, Statement};

use crate::scope::Scope;
use crate::value::Value;


trait Execute {
    #[allow(unused_variables)]
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) {}

    #[allow(unused_variables)]
    fn execute(&self, scope: &mut Scope) {}
}

impl Execute for PrintStatement {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) {
        for i in 0..self.list.len() {
            let item = self.list.get(i).unwrap();

            match &item.value {
                PrintListItem_value::Expression(e) => {
                    let v = e.compute(scope);

                    match v {
                        Ok(v) => match v {
                            Value::String(s) => stdout.write_all(s.as_bytes()).unwrap(),
                            Value::Integer(i) => stdout.write_all(format!("{}", i).as_bytes()).unwrap(),
                            Value::Float(f) => stdout.write_all(format!("{}", f).as_bytes()).unwrap(),
                            Value::Boolean(b) => stdout.write_all(format!("{}", b).as_bytes()).unwrap(),
                        }
                        Err(_) => {}
                    };
                }
                PrintListItem_value::StringLiteral(s) => stdout.write_all(s.body.as_bytes()).unwrap()
            }

            match item.sep {
                None => stdout.write_all("\n".as_bytes()).unwrap(),
                Some(_) => {}
            }
        }
    }
}

impl Execute for Assignment {
    fn execute(&self, scope: &mut Scope) {
        let v = match &self.value {
            Assignment_value::Expression(e) => e.compute(scope).unwrap(),
            Assignment_value::StringLiteral(s) => Value::String(s.body.clone())
        };

        scope.set(&self.variable.name, v);
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

