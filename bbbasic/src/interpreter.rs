use std::collections::HashMap;
use std::io::Write;
use crate::parser::{Block, NumberLiteral, NumberLiteral_value, PrintStatement, PrintStatement_literal, Program, Statement};


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
    match &s.literal {
        PrintStatement_literal::NumberLiteral(n) => {
            match &n.value {
                NumberLiteral_value::FloatLiteral(f) => {
                    let _ = stdout.write_all(format!("{}", f).as_bytes());
                }
                NumberLiteral_value::IntegerLiteral(i) => {
                    let _ = stdout.write_all(format!("{}", i).as_bytes());
                }
            }
        },

        PrintStatement_literal::StringLiteral(s) => {
            let _ = stdout.write_all(s.body.as_bytes());
        }
    };
}

fn execute_statement(statement: &Statement, scope: &mut Scope, stdout: &mut impl Write) {
    match statement {
        Statement::EndStatement(_) => {}
        Statement::PrintStatement(s) => execute_print(s, scope, stdout)
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