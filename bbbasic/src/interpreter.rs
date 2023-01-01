use std::io::Write;
use crate::bool_expression::ComputeBool;
use crate::error::InterpreterError;
use crate::expression::Compute;
use crate::interpreter::ExecutionResult::Exit;
use crate::interpreter::ExitReason::{For, While};
use crate::parser::{Assignment, Assignment_value, Block, ForStatement, IfStatement, PrintListItem_value, PrintStatement, Program, Statement, WhileStatement};

use crate::scope::Scope;
use crate::value::Value;

#[derive(Clone, Copy)]
pub enum ExitReason {
    For,
    While,
}

#[derive(Clone, Copy)]
pub enum ExecutionResult {
    Ok,
    ForCompleted,
    Exit(ExitReason),
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
            match self.iterate(&target, &step, scope, stdout)? {
                ExecutionResult::Ok => {}

                ExecutionResult::ForCompleted => {
                    return Ok(ExecutionResult::Ok);
                }

                Exit(ExitReason::For) => {
                    return Ok(ExecutionResult::Ok);
                }
                Exit(ExitReason::While) => {
                    return Ok(Exit(While));
                }
            }
        }
    }
}

impl ForStatement {
    fn iterate(&self, target: &Value, step: &Value, scope: &mut Scope, stdout: &mut impl Write)
               -> Result<ExecutionResult, InterpreterError> {
        let result = self.body.execute_stdout(scope, stdout)?;

        match result {
            ExecutionResult::Ok => {
                let curr = scope.get(&self.assignment.variable.name)?;

                let next = curr.add(step)?;

                if next.gt(target).unwrap() {
                    return Ok(ExecutionResult::ForCompleted);
                }

                scope.set(&self.assignment.variable.name, next.clone());

                Ok(ExecutionResult::Ok)
            }

            Exit(reason) => {
                Ok(Exit(reason))
            }

            ExecutionResult::ForCompleted => Err(InterpreterError::OperationUnsupported)
        }
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

impl Execute for WhileStatement {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {
        loop {
            let c = self.condition.compute_bool(scope)?.as_bool()?;

            if c {
                let r = self.body.execute_stdout(scope, stdout)?;

                match r {
                    ExecutionResult::Ok => {}
                    Exit(While) => return Ok(ExecutionResult::Ok),
                    Exit(For) => return Ok(Exit(For)),
                    ExecutionResult::ForCompleted => return Err(InterpreterError::OperationUnsupported)
                }
            } else {
                break;
            }
        }

        Ok(ExecutionResult::Ok)
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
            Statement::ExitForStatement(_) => Ok(ExecutionResult::Exit(ExitReason::For)),
            Statement::WhileStatement(w) => w.execute_stdout(scope, stdout),
            Statement::ExitWhileStatement(_) => Ok(ExecutionResult::Exit(ExitReason::While))
        };
    }
}


impl Execute for Block {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {
        for i in 0..self.statements.len() {
            let statement = self.statements.get(i).unwrap();
            let result = statement.execute_stdout(scope, stdout)?;

            match result {
                ExecutionResult::Exit(_) => return Ok(result.clone()),
                _ => {}
            };
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

