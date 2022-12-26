use std::collections::HashMap;
use crate::interpreter::{BBAssignment, BBExpression, BBStatement, InterpreterError};
use crate::interpreter::bb_types::{BbFloat, BbInt};

enum ExpressionResult {
    String(String),
    Integer(BbInt),
    Float(BbFloat)
}

impl Clone for ExpressionResult {
    fn clone(&self) -> Self {
        match self {
            ExpressionResult::String(s) => ExpressionResult::String(s.clone()),
            ExpressionResult::Integer(i) => ExpressionResult::Integer(*i),
            ExpressionResult::Float(f) => ExpressionResult::Float(*f)
        }
    }
}

struct Scope {
    values: HashMap<String, ExpressionResult>
}

impl Scope {

    fn new() -> Scope {
        Scope { values: HashMap::new() }
    }

    fn get(&self, name: &String) -> Option<&ExpressionResult> {
        self.values.get(name.as_str())
    }

    fn set(&mut self, name: String, value: ExpressionResult) {
        self.values.insert(name, value);
    }
}


fn execute_expression(expression: &BBExpression, scope: &Scope) -> Result<ExpressionResult, InterpreterError>{
    match expression {
        BBExpression::String(s) => Ok(ExpressionResult::String(s.clone())),
        BBExpression::Integer(i) => Ok(ExpressionResult::Integer(*i)),
        BBExpression::Float(f) => Ok(ExpressionResult::Float(*f)),
        BBExpression::Variable(v) => match scope.get(v) {
            None => Err(InterpreterError::UnknownVariable(v.clone())),
            Some(v) => Ok(v.clone())
        }
    }
}

fn execute_print(statement: &BBStatement, scope: &Scope) -> Result<bool, InterpreterError> {
    if let BBStatement::PRINT(e) = statement {

        match execute_expression(e, scope) {
            Ok(r) => match r {
                ExpressionResult::String(s) => println!("{}",s),
                ExpressionResult::Integer(i) => println!("{}", i),
                ExpressionResult::Float(f) => println!("{}", f)
            }
            Err(_) => {}
        }
    }

    Ok(true)
}

fn execute_assignment(assignment: &BBAssignment, scope: &mut Scope) -> Result<bool, InterpreterError> {
    let v = execute_expression(&assignment.value, scope);

    match v {
        Ok(r) => scope.set(assignment.name.clone(), r),
        Err(e) => return Err(e)
    };

    Ok(true)
}

pub fn execute(statements: &Vec<BBStatement>) {

    let mut scope = Scope::new();

    for r in statements {
        match r {
            BBStatement::PRINT(_) => {
                let _  = execute_print(r, &scope);
            },
            BBStatement::FOR(_, _) => {},
            BBStatement::NEXT(_) => {},
            BBStatement::END => {},
            BBStatement::Nop => {}
            BBStatement::ASSIGNMENT(a) => {
                let _ = execute_assignment(a, &mut scope);
            }
        };
    }
}