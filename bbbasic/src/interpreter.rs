use std::io::Write;
use std::str::FromStr;
use crate::bool_expression::ComputeBool;
use crate::error::InterpreterError;
use crate::error::InterpreterError::NotImplemented;
use crate::expression::Compute;
use crate::interpreter::ExecutionResult::Exit;
use crate::interpreter::ExitReason::{For, While};
use crate::parser::{Assignment, Block, ForAssignment, ForStatement, ForStep, IfStatement, NumericVariable, NumericVariable_type_dem, PrintListItem_value, PrintStatement, Program, Statement, StringAssignment, WhileStatement};
use crate::parser::BoolOperand::Expression;

use crate::scope::{Byte, DataTypeQuery, Float, Integer, One, Scope, ScopeValue};
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
                        _ => {}
                    }
                }

                PrintListItem_value::StringLiteral(s) => stdout.write_all(s.body.as_bytes()).unwrap(),
                PrintListItem_value::StringVariable(s) => stdout.write_all(scope.get_string(&s.name)?.as_bytes()).unwrap()
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
        match &self.variable.type_dem {
            None => {
                let v = self.value.compute_float(scope)?;
                scope.set_float(&self.variable.name, v);
            }
            Some(d) => match d {
                NumericVariable_type_dem::ByteDenominator(_) => {
                    let v = self.value.compute_byte(scope)?;
                    scope.set_byte(&self.variable.name, v);
                }
                NumericVariable_type_dem::FloatDenominator(_) => {
                    let v = self.value.compute_float(scope)?;
                    scope.set_float(&self.variable.name, v);
                }
                NumericVariable_type_dem::IntegerDenominator(_) => {
                    let v = self.value.compute_integer(scope)?;
                    scope.set_int(&self.variable.name, v);
                }
            }
        }

        Ok(ExecutionResult::Ok)
    }
}

impl Execute for ForAssignment {
    fn execute(&self, scope: &mut Scope) -> Result<ExecutionResult, InterpreterError> {
        match &self.variable.type_dem {
            None => {
                let v = self.value.compute_float(scope)?;
                scope.set_float(&self.variable.name, v);
            }
            Some(d) => match d {
                NumericVariable_type_dem::ByteDenominator(_) => {
                    let v = self.value.compute_byte(scope)?;
                    scope.set_byte(&self.variable.name, v);
                }
                NumericVariable_type_dem::FloatDenominator(_) => {
                    let v = self.value.compute_float(scope)?;
                    scope.set_float(&self.variable.name, v);
                }
                NumericVariable_type_dem::IntegerDenominator(_) => {
                    let v = self.value.compute_integer(scope)?;
                    scope.set_int(&self.variable.name, v);
                }
            }
        }

        Ok(ExecutionResult::Ok)
    }
}

impl Execute for StringAssignment {
    fn execute(&self, scope: &mut Scope) -> Result<ExecutionResult, InterpreterError> {
        let v = self.value.body.to_string();

        scope.set_string(&self.variable.name, v);

        Ok(ExecutionResult::Ok)
    }
}

impl Execute for ForStatement {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {

        self.assignment.execute(scope)?;

        match &self.assignment.variable.type_dem {
            None => self.do_loop_float(scope, stdout),

            Some(d) => match d {
                // NumericVariable_type_dem::ByteDenominator(_) => self.do_loop::<Byte>(scope, stdout),
                NumericVariable_type_dem::FloatDenominator(_) => self.do_loop_float(scope, stdout),
                NumericVariable_type_dem::IntegerDenominator(_) => self.do_loop_integer(scope, stdout),
                _ => Err(NotImplemented("Loops for other than integers and floats".to_string()))
            }
        }
    }
}

impl ForStatement {
    fn do_loop_integer(&self, scope: &mut Scope, stdout: &mut impl Write)
                       -> Result<ExecutionResult, InterpreterError> {
        let target = self.target.compute_integer(scope)?;
        let step: Integer = match &self.step {
            Some(s) => s.value.compute_integer(scope)?,
            None => 1,
        };

        loop {
            match self.iterate_integer(target, step, scope, stdout)? {
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

    fn do_loop_float(&self, scope: &mut Scope, stdout: &mut impl Write)
                       -> Result<ExecutionResult, InterpreterError> {
        let target = self.target.compute_float(scope)?;
        let step: Float = match &self.step {
            Some(s) => s.value.compute_float(scope)?,
            None => 1.0,
        };

        loop {
            match self.iterate_float(target, step, scope, stdout)? {
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

    fn iterate_integer(&self, target: Integer, step: Integer, scope: &mut Scope, stdout: &mut impl Write)
                       -> Result<ExecutionResult, InterpreterError> {
        let result = self.body.execute_stdout(scope, stdout)?;


        match result {
            ExecutionResult::Ok => {
                let curr = scope.get_int(&self.assignment.variable.name)?;

                let next = curr + step;

                if next > target {
                    return Ok(ExecutionResult::ForCompleted);
                }

                scope.set_int(&self.assignment.variable.name, next.clone());

                Ok(ExecutionResult::Ok)
            }

            Exit(reason) => {
                Ok(Exit(reason))
            }

            ExecutionResult::ForCompleted => Err(InterpreterError::OperationUnsupported)
        }
    }

    fn iterate_float(&self, target: Float, step: Float, scope: &mut Scope, stdout: &mut impl Write)
                       -> Result<ExecutionResult, InterpreterError> {
        let result = self.body.execute_stdout(scope, stdout)?;


        match result {
            ExecutionResult::Ok => {
                let curr = scope.get_float(&self.assignment.variable.name)?;

                let next = curr + step;

                if next > target {
                    return Ok(ExecutionResult::ForCompleted);
                }

                scope.set_float(&self.assignment.variable.name, next.clone());

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
            Statement::StringAssignment(a) => a.execute(scope),
            Statement::ForStatement(f) => f.execute_stdout(scope, stdout),
            Statement::IfStatement(i) => i.execute_stdout(scope, stdout),
            Statement::ExitForStatement(_) => Ok(Exit(For)),
            Statement::WhileStatement(w) => w.execute_stdout(scope, stdout),
            Statement::ExitWhileStatement(_) => Ok(Exit(While))
        };
    }
}


impl Execute for Block {
    fn execute_stdout(&self, scope: &mut Scope, stdout: &mut impl Write) -> Result<ExecutionResult, InterpreterError> {
        for i in 0..self.statements.len() {
            let statement = self.statements.get(i).unwrap();
            let result = statement.execute_stdout(scope, stdout)?;

            match result {
                Exit(_) => return Ok(result.clone()),
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
