extern crate pest;

use std::arch::x86_64::_mm_pause;
use pest::iterators::{Pair, Pairs};
use pest::{Parser, RuleType};

pub mod bb_types;

use bb_types::BbInt;
use bb_types::BbFloat;


#[derive(Parser)]
#[grammar="bbbasic.pest"]
pub struct BBasicParser;


#[derive(Debug)]
pub enum InterpreterError {
    Generic(String),
    Syntax,
    UnknownVariable(String)

}

#[derive(Debug)]
pub enum BBExpression {
    String(String),
    Integer(BbInt),
    Float(BbFloat),
    Variable(String)
}

impl Clone for BBExpression {
    fn clone(&self) -> Self {
        match self {
            BBExpression::String(s) => BBExpression::String(s.clone()),
            BBExpression::Integer(i) => BBExpression::Integer(*i),
            BBExpression::Float(f) => BBExpression::Float(*f),
            BBExpression::Variable(v) => BBExpression::String(v.clone())
        }
    }
}

pub type BBBlock = Vec<BBStatement>;

#[derive(Debug)]
pub struct BBAssignment {
    pub name: String,
    pub value: BBExpression,
}


#[derive(Debug)]
pub enum BBNumeric {
    Float(BbFloat),
    Integer(BbInt)
}

#[derive(Debug)]
pub enum BBStatement {
    PRINT(BBExpression),
    FOR(BBAssignment, BBExpression),
    NEXT(String),
    ASSIGNMENT(BBAssignment),
    END,

    Nop,
}


fn interpret_expression(pair: Pair<Rule>) -> Result<BBExpression, InterpreterError> {

    if Rule::bb_var_name == pair.as_rule() {
        return Ok(BBExpression::Variable(pair.as_str().to_string()))
    }

    for p in pair.into_inner() {
        return match p.as_rule() {
            Rule::bb_float_literal => Ok(BBExpression::Float(p.as_str().parse::<BbFloat>().unwrap())),
            Rule::bb_int_literal => Ok(BBExpression::Integer(p.as_str().parse::<BbInt>().unwrap())),
            Rule::bb_string => Ok(BBExpression::String(p.as_str().to_string())),

            _ => {
                println!("ERROR on Expression {:?}", p);
                Err(InterpreterError::Generic("What do I know".to_string()))
            }
        }
    }

    return Err(InterpreterError::Generic("Empty Expression".to_string()))
}


fn interpret_assignment(pair: Pair<Rule>) -> Result<BBAssignment, InterpreterError> {
    let mut pairs = pair.into_inner();

    let var_name = pairs.next().unwrap();
    let value = interpret_expression(pairs.next().unwrap());

    return match value {
        Ok(e) => Ok(BBAssignment { name: var_name.as_str().to_string(), value: e }),
        Err(e) => Err(e)
    }
}

fn interpret_for_statement(pair: Pair<Rule>) -> Result<BBStatement, InterpreterError> {
    if Rule::bb_for_statement == pair.as_rule() {
        let mut pairs = pair.into_inner();

        let a = pairs.next().unwrap();

        let b = pairs.next().unwrap();

        return Ok(BBStatement::FOR(
            interpret_assignment(a).unwrap(),
            interpret_expression(b).unwrap()
        ))
    }

    Err(InterpreterError::Generic("Not a for loop".to_string()))
}

fn interpret_next_statement(pair: Pair<Rule>) -> Result<BBStatement, InterpreterError> {
    if Rule::bb_next_statement == pair.as_rule() {
        return Ok(BBStatement::NEXT(pair.into_inner().next().unwrap().as_str().to_string()));
    }

    Err(InterpreterError::Syntax)
}


fn interpret_print_statement(pair: Pair<Rule>) -> Result<BBStatement, InterpreterError> {
    if Rule::bb_print_statement == pair.as_rule() {
        return Ok(BBStatement::PRINT(interpret_expression(pair.into_inner().next().unwrap()).unwrap()))
    }

    return Err(InterpreterError::Syntax);
}

fn interpret_statement(pair: Pair<Rule>) -> Result<BBStatement, InterpreterError> {

    return match pair.as_rule() {
        Rule::bb_print_statement => interpret_print_statement(pair),

        Rule::bb_for_statement => interpret_for_statement(pair),

        Rule::bb_next_statement => interpret_next_statement(pair),

        Rule::bb_end_statement => Ok(BBStatement::END),

        Rule::bb_assignment => match interpret_assignment(pair) {
            Ok(a) => Ok(BBStatement::ASSIGNMENT(a)),
            Err(e) => Err(e)
        }

        Rule::EOI => Ok(BBStatement::Nop),

        _ => Err(InterpreterError::Generic(format!("{:?}", &pair)))
    }
}

fn interpret_statements(pairs: Pairs<Rule>) -> Result<Vec<BBStatement>, InterpreterError> {
    let mut r: Vec<BBStatement> = Vec::new();

    for pair in pairs {
        // println!("{:?}", pair);
        match interpret_statement(pair) {
            Ok(s) => r.push(s),
            Err(e) => {}
        }
    }

    Ok(r)
}

pub fn interpret_program(pair: Pair<Rule>) -> Result<Vec<BBStatement>, InterpreterError> {
    match pair.as_rule() {
        Rule::bb_program => interpret_statements(pair.into_inner()),
        _ => Err(InterpreterError::Generic(format!("{:?}", &pair)))
    }
}

pub fn interpret(input: &str) -> Result<Vec<BBStatement>, InterpreterError> {

    let r = BBasicParser::parse(Rule::bb_program, input);

    // println!("Interpreting");
    // println!("{}", input);

    match r {
        Ok(r) => {
            let p = r.into_iter().next().unwrap();
            return interpret_program(p);

        },

        Err(e) => Err(InterpreterError::Generic(e.to_string()))
    }
}