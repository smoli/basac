use std::collections::HashMap;
use std::io::Write;
use crate::interpreter::{BBAssignment, BBExpression, BBStatement, InterpreterError};
use crate::interpreter::bb_types::{BbFloat, BbInt};

type PCOffset = i32;

enum ExpressionResult {
    String(String),
    Integer(BbInt),
    Float(BbFloat),
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

impl ExpressionResult {
    fn as_float(&self) -> Option<BbFloat> {
        match self {
            ExpressionResult::String(_) => None,
            ExpressionResult::Integer(i) => Some(*i as BbFloat),
            ExpressionResult::Float(f) => Some(*f)
        }
    }
}

enum Numeric {
    Integer(BbInt),
    Float(BbFloat),
}

impl Numeric {
    fn as_float(&self) -> BbFloat {
        match self {
            Numeric::Integer(i) => *i as BbFloat,
            Numeric::Float(f) => *f
        }
    }

    #[allow(dead_code)]
    fn as_int(&self) -> BbInt {
        match self {
            Numeric::Integer(i) => *i,
            Numeric::Float(f) => f.round() as BbInt
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

    /*fn update_inner_for_loop(&mut self, pc: usize) -> Result<PCOffset, InterpreterError> {
        let my_loop = self.loops.last();

        return match my_loop {
            None => Err(InterpreterError::NotInForLoop),
            Some(l) => match l {
                Loop::For(name, target, step, goto_pc) => {
                    let val = self.values.get(&name.clone());

                    if let Some(r) = val {
                        let curr = match r {
                            ExpressionResult::String(_) => return Err(InterpreterError::TypeMismatch),
                            ExpressionResult::Integer(i) => Numeric::Integer(*i),
                            ExpressionResult::Float(f) => Numeric::Float(*f)
                        };

                        match curr {
                            Numeric::Integer(i) => {
                                let curr = i + (step.as_float().round() as BbInt);
                                self.values.insert(name.clone(), ExpressionResult::Integer(curr));
                                if curr > target.as_int() {
                                    self.loops.pop();
                                    return Ok(1);
                                }
                            }
                            Numeric::Float(i) => {
                                let curr = i + step.as_float();
                                self.values.insert(name.clone(), ExpressionResult::Float(curr));
                                if curr > target.as_float() {
                                    self.loops.pop();
                                    return Ok(1);
                                }
                            }
                        }

                        Ok((*goto_pc as PCOffset) - pc as PCOffset + 1)
                    } else {
                        Err(InterpreterError::UnknownVariable(name.clone()))
                    }
                }
            }
        };
    }*/
}


fn execute_expression(expression: &BBExpression, scope: &Scope) -> Result<ExpressionResult, InterpreterError> {
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


fn expression_to_numeric(expression: &BBExpression, scope: &mut Scope) -> Result<Numeric, InterpreterError> {
    let target_result = execute_expression(expression, scope)?;

    let target_value = match target_result {
        ExpressionResult::String(_) => return Err(InterpreterError::TypeMismatch),
        ExpressionResult::Integer(i) => Numeric::Integer(i),
        ExpressionResult::Float(f) => Numeric::Float(f)
    };

    Ok(target_value)
}

fn execute_print(statement: &BBStatement, scope: &Scope, stdout: &mut impl Write) -> Result<PCOffset, InterpreterError> {
    if let BBStatement::PRINT(e) = statement {
        match execute_expression(e, scope)? {
            ExpressionResult::String(s) => {
                let _ = stdout.write_all(s.as_bytes());
                let _ = stdout.write_all("\n".as_bytes());
            }
            ExpressionResult::Integer(i) => {
                let _ = stdout.write_all(format!("{}\n", i).as_bytes());
            }
            ExpressionResult::Float(f) => {
                let _ = stdout.write_all(format!("{}\n", f).as_bytes());
            }
        };
    }

    Ok(1)
}

fn execute_assignment(assignment: &BBAssignment, scope: &mut Scope) -> Result<PCOffset, InterpreterError> {
    let v = execute_expression(&assignment.value, scope)?;
    scope.set(assignment.name.clone(), v);
    Ok(1)
}

fn execute_for(pc: usize, for_statement: &BBStatement, scope: &mut Scope, stdout: &mut impl Write) -> Result<PCOffset, InterpreterError> {
    if let BBStatement::FOR(a, target, step, block) = for_statement {
        let v = execute_expression(&a.value, scope)?;

        let target_value = expression_to_numeric(target, scope)?.as_float();
        let step_value = expression_to_numeric(step, scope)?.as_float();

        let mut iterator_value: BbFloat;

        match v.as_float() {
            None => return Err(InterpreterError::TypeMismatch),
            Some(f) => iterator_value = f
        }

        let mut pc_distance: PCOffset;

        loop {
            scope.set(a.name.clone(), ExpressionResult::Float(iterator_value));

            pc_distance = execute_block(pc, block, scope, stdout)?;

            iterator_value += step_value;

            if iterator_value > target_value {
                break;
            }
        }

        // +2 for the FOR statement and the NEXT statement
        return Ok(pc_distance + 2);
    }

    return Err(InterpreterError::Generic("NOT a For Loop".to_string()));
}

fn execute_block(pc: usize, statements: &Vec<BBStatement>, scope: &mut Scope, stdout: &mut impl Write) -> Result<PCOffset, InterpreterError> {
    let mut l_pc = 0;

    while l_pc < statements.len() {
        let r = statements.get(l_pc).unwrap();
        let jmp = match r {
            BBStatement::PRINT(_) => execute_print(&r, scope, stdout),
            BBStatement::ASSIGNMENT(a) => execute_assignment(&a, scope),
            BBStatement::FOR(_, _, _, _) => execute_for(pc, &r, scope, stdout),

            _ => Ok(1)
        };

        match jmp {
            Ok(dst) => {
                let next = l_pc as PCOffset + dst;
                if next < 0 {
                    panic!("This should not happen! PC < 0")
                }
                l_pc = next as usize;
            }
            Err(_) => panic!("PANIC!") // TODO
        }
    }

    Ok(l_pc as PCOffset)
}

pub fn execute(statements: &Vec<BBStatement>, stdout: &mut impl Write) {
    let mut scope = Scope::new();

    let pc: usize = 0;

    let _ = execute_block(pc, statements, &mut scope, stdout);
}
