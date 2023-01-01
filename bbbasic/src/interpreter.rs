use std::io::Write;
use crate::bool_expression::ComputeBool;
use crate::error::InterpreterError;
use crate::expression::Compute;
use crate::parser::{Assignment, Assignment_value, Block, ForStatement, IfStatement, PrintListItem_value, PrintStatement, Program, Statement};

use crate::scope::Scope;
use crate::value::Value;

pub enum ExecutionResult {
    Ok,
    ExitFor
}


trait Execute {
    #[allow(unused_variables)]
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {
        Ok(ExecutionResult::Ok)
    }

    #[allow(unused_variables)]
    fn execute(&self, scope: &mut Scope) -> Result<ExecutionResult, InterpreterError> {
        Ok(ExecutionResult::Ok)
    }
}

impl Execute for PrintStatement {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {
        for i in 0..self.list.len() {
            let item = self.list.get(i).unwrap();

            match &item.value {
                PrintListItem_value::Expression(e) => {
                    let v = e.compute(scope)?;

                    match v {
                            Value::String(s) => stdout.write_all(s.as_bytes()).unwrap(),
                            Value::Integer(i) => stdout.write_all(format!("{}", i).as_bytes()).unwrap(),
                            Value::Float(f) => stdout.write_all(format!("{}", f).as_bytes()).unwrap(),
                            Value::Boolean(b) => stdout.write_all(format!("{}", b).as_bytes()).unwrap(),
                        }
                }
                PrintListItem_value::StringLiteral(s) => stdout.write_all(s.body.as_bytes()).unwrap()
            };

            match item.sep {
                None => stdout.write_all("\n".as_bytes()).unwrap(),
                Some(_) => {}
            };
        }

        Ok(ExecutionResult::Ok)
    }
}

impl Execute for Assignment {
    fn execute(&self, scope: &mut Scope) -> Result<ExecutionResult, InterpreterError> {
        let v = match &self.value {
            Assignment_value::Expression(e) => e.compute(scope)?,
            Assignment_value::StringLiteral(s) => Value::String(s.body.clone())
        };

        scope.set(&self.variable.name, v);

        Ok(ExecutionResult::Ok)
    }

}

impl Execute for ForStatement {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {
        self.assignment.execute(scope)?;

        let step = match &self.step {
            None => Value::Integer(1),
            Some(f) => f.value.compute(scope)?
        };

        let target = self.target.compute(scope)?;

        loop {
            if self.iterate(&target, &step, scope, stdout)? == false {
                break
            }
        }

        Ok(ExecutionResult::Ok)
    }
}

impl ForStatement {
    fn iterate(&self, target: &Value, step: &Value, scope: &mut Scope, stdout: &mut impl Write) -> Result<bool, InterpreterError> {
        self.body.execute_stdout(scope, stdout)?;

        let curr = scope.get(&self.assignment.variable.name)?;

        let next = curr.add(step)?;

        if next.gt(target).unwrap() {
            return Ok(false)
        }

        scope.set(&self.assignment.variable.name, next.clone());

        Ok(true)
    }
}

impl Execute for IfStatement {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {
        let c = self.condition.compute_bool(scope)?.as_bool()?;

        if c == true {
            self.then_block.execute_stdout(scope, stdout)
        } else {
            match &self.else_block {
                None => Ok(ExecutionResult::Ok),
                Some(e) => e.execute_stdout(scope, stdout)
            }
        }
    }
}


impl Execute for Statement {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {
        return match self {
            Statement::EndStatement(_) => Ok(ExecutionResult::Ok),
            Statement::PrintStatement(s) => s.execute_stdout(scope, stdout),
            Statement::Assignment(a) => a.execute(scope),
            Statement::ForStatement(f) => f.execute_stdout(scope, stdout),
            Statement::IfStatement(i) => i.execute_stdout(scope, stdout),
            Statement::ExitForStatement(_) => Ok(ExecutionResult::ExitFor)
        }
    }
}


impl Execute for Block {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {
        for i in 0..self.statements.len() {
            let s = self.statements.get(i).unwrap();
            s.execute_stdout(scope, stdout)?;
        }

        Ok(ExecutionResult::Ok)
    }
}


impl Program {
    pub fn execute(&self, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {
        let mut scope = Scope::new();
        self.body.execute_stdout(&mut scope, stdout)
    }
}

