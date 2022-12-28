use std::collections::HashMap;
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

    fn as_int(&self) -> BbInt {
        match self {
            Numeric::Integer(i) => *i,
            Numeric::Float(f) => f.round() as BbInt
        }
    }
}


enum Loop {
    //  Variable name, Target value, Step, Line of For Statement
    For(String, Numeric, Numeric, usize)
}

struct Scope {
    parent: Option<Box<Scope>>,
    values: HashMap<String, ExpressionResult>,
    loops: Vec<Loop>,
}

impl Scope {
    fn new() -> Scope {
        Scope { parent: None, values: HashMap::new(), loops: Vec::new() }
    }

    fn get(&self, name: &String) -> Option<&ExpressionResult> {
        self.values.get(name.as_str())
    }

    fn set(&mut self, name: String, value: ExpressionResult) {
        self.values.insert(name, value);
    }

    fn update_inner_for_loop(&mut self, pc: usize) -> Result<PCOffset, InterpreterError>{
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
        }
    }
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

fn execute_print(statement: &BBStatement, scope: &Scope) -> Result<PCOffset, InterpreterError> {
    if let BBStatement::PRINT(e) = statement {
        match execute_expression(e, scope) {
            Ok(r) => match r {
                ExpressionResult::String(s) => println!("{}", s),
                ExpressionResult::Integer(i) => println!("{}", i),
                ExpressionResult::Float(f) => println!("{}", f)
            }
            Err(_) => {}
        }
    }

    Ok(1)
}

fn execute_assignment(assignment: &BBAssignment, scope: &mut Scope) -> Result<PCOffset, InterpreterError> {
    let v = execute_expression(&assignment.value, scope)?;
    scope.set(assignment.name.clone(), v);

    Ok(1)
}

fn execute_for(pc: usize, for_statement: &BBStatement, scope: &mut Scope) -> Result<PCOffset, InterpreterError> {
    if let BBStatement::FOR(a, target) = for_statement {
        let _ = execute_assignment(a, scope)?;

        let target_result = execute_expression(target, scope)?;

        let target_value = match target_result {
            ExpressionResult::String(_) => return Err(InterpreterError::TypeMismatch),
            ExpressionResult::Integer(i) => Numeric::Integer(i),
            ExpressionResult::Float(f) => Numeric::Float(f)
        };

        scope.loops.push(Loop::For(a.name.clone(), target_value, Numeric::Integer(1), pc));

        return Ok(1);
    }

    return Err(InterpreterError::Generic("NOT a For Loop".to_string()));
}

fn execute_next(pc: usize, _next_statement: &BBStatement, scope: &mut Scope) -> Result<PCOffset, InterpreterError> {
    scope.update_inner_for_loop(pc)
}


pub fn execute(statements: &Vec<BBStatement>) {
    let mut scope = Scope::new();

    let mut pc: usize = 0;

    while pc < statements.len() {
        let r = statements.get(pc).unwrap();
        let jmp = match r {
            BBStatement::PRINT(_) => execute_print(&r, &scope),
            BBStatement::ASSIGNMENT(a) => execute_assignment(&a, &mut scope),
            BBStatement::FOR(_, _) => execute_for(pc, &r, &mut scope),
            BBStatement::NEXT(_) => execute_next(pc, &r, &mut scope),

            _ => Ok(1)
        };

        match jmp {
            Ok(dst) => {
                let next = pc as PCOffset + dst;
                if next < 0 {
                    panic!("Panic!")
                }
                pc = next as usize;
            },
            Err(_) => panic!("PANIC!") // TODO
        }
    }
}