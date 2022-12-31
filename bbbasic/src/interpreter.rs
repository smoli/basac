use std::collections::HashMap;
use std::io::Write;
use crate::parser::{Assignment, Assignment_value, Block, Expression_term, NumberLiteral, NumberLiteral_value, PrintStatement, PrintStatement_list, Program, Statement};


#[derive(Debug,Clone)]
enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool)
}

#[derive(Debug)]
struct Scope {
    values: HashMap<String, Value>
}

impl Scope {

    fn new() ->Scope {
        Scope { values: HashMap::new() }
    }

    fn set(&mut self, name: &String, value: Value) {
        self.values.insert(name.clone(), value);
    }

    fn get(&self, name: &String) -> Option<&Value> {
        self.values.get(name)
    }

}


fn execute_print(s: &PrintStatement, scope: &mut Scope, stdout: &mut impl Write) {
    match &s.list {
        PrintStatement_list::Expression(e) => match &e.term {
            Expression_term::NumberLiteral(n) => match &n.value {
                NumberLiteral_value::FloatLiteral(f) => {
                    let _ = stdout.write_all(format!("{}", f).as_bytes());
                }
                NumberLiteral_value::IntegerLiteral(i) => {
                    let _ = stdout.write_all(format!("{}", i).as_bytes());
                }
            }
            Expression_term::VariableName(v) => {
                let value = scope.get(v).unwrap();

                match value {
                    Value::String(s) => {
                        let _ = stdout.write_all(s.as_bytes());
                    }
                    Value::Integer(i) => {
                        let _ = stdout.write_all(format!("{}", i).as_bytes());
                    }
                    Value::Float(f) => {
                        let _ = stdout.write_all(format!("{}", f).as_bytes());
                    }
                    Value::Boolean(b) => {
                        let _ = stdout.write_all(format!("{}", b).as_bytes());
                    }
                }
            }
        }
        PrintStatement_list::StringLiteral(s) => {
            let _ = stdout.write_all(s.body.as_bytes());
        }
    };
}

fn execute_assignment(assignment: &Assignment, scope: &mut Scope) {

    let v = match &assignment.value {
        Assignment_value::NumberLiteral(n) => {
            match &n.value {
                NumberLiteral_value::FloatLiteral(f) => Value::Float(f.parse::<f64>().unwrap()),
                NumberLiteral_value::IntegerLiteral(i) => Value::Integer(i.parse().unwrap())
            }
        }
        Assignment_value::StringLiteral(s) => Value::String(s.body.clone())
    };

    scope.set(&assignment.variable, v);
}

fn execute_statement(statement: &Statement, scope: &mut Scope, stdout: &mut impl Write) {
    match statement {
        Statement::EndStatement(_) => {}
        Statement::PrintStatement(s) => execute_print(s, scope, stdout),
        Statement::Assignment(a) => execute_assignment(a, scope)
    }
}

fn execute_block(block: &Block, scope: &mut Scope, stdout: &mut impl Write ) {
    for i in 0..block.statements.len() {
        let s = block.statements.get(i).unwrap();
            execute_statement(s, scope, stdout);
    }
}

pub fn execute(program: &Program, stdout: &mut impl Write) {
    let mut scope = Scope::new();
    execute_block(&program.body, &mut scope, stdout);
}