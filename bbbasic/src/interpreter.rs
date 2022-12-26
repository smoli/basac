extern crate pest;

use pest::iterators::{Pair, Pairs};
use pest::Parser;

pub mod bb_types;

use bb_types::BbInt;
use bb_types::BbFloat;



#[derive(Parser)]
#[grammar="bbbasic.pest"]
pub struct BBasicParser;


#[derive(Debug)]
pub enum InterpreterError {
    Generic(String),

}

pub enum BBExpression {
    String(String),
    Integer(BbInt),
    Float(BbFloat)
}

pub type BBBlock = Vec<BBStatement>;

pub struct BBAssignment {
    name: String,
    value: BBExpression
}

#[derive(Debug)]
pub enum BBStatement {
    PRINT(BBExpression),
    FORLoop(BBAssignment, BBBlock),

    Nop,
}


fn interpret_expression(pairs: Pairs<Rule>) -> Result<BBExpression, InterpreterError> {
    for p in pairs {
        return match p.as_rule() {
            Rule::bb_float_literal => Ok(BBExpression::Float(p.as_str().parse::<BbFloat>().unwrap())),
            Rule::bb_int_literal => Ok(BBExpression::Integer(p.as_str().parse::<BbInt>().unwrap())),
            Rule::bb_string => Ok(BBExpression::String(p.as_str().to_string())),

            _ => {
                println!("{:?}", p);
                Err(InterpreterError::Generic("What do I know".to_string()))
            }
        }
    }

    return Err(InterpreterError::Generic("Empty Expression".to_string()))
}

fn interpret_for_loop(pair: Pair<Rule>) -> Result<BBStatement, InterpreterError> {
    Err(InterpreterError::Generic("Not a for loop".to_string()))
}

fn interpret_statement(pair: Pair<Rule>) -> Result<BBStatement, InterpreterError> {

    return match pair.as_rule() {
        Rule::bb_print_statement => Ok(BBStatement::PRINT(interpret_expression(pair.into_inner()).unwrap())),

        Rule::bb_for_statement => interpret_for_loop(pair),

        Rule::EOI => Ok(BBStatement::Nop),

        _ => Err(InterpreterError::Generic(format!("{:?}", &pair)))
    }
}

fn interpret_statements(pairs: Pairs<Rule>) -> Result<Vec<BBStatement>, InterpreterError> {
    let mut r: Vec<BBStatement> = Vec::new();

    for pair in pairs {
        match interpret_statement(pair) {
            Ok(s) => r.push(s),
            Err(e) => return Err(e)
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

    match r {
        Ok(r) => {
            let p = r.into_iter().next().unwrap();
            return interpret_program(p);

        },

        Err(e) => Err(InterpreterError::Generic(e.to_string()))
    }
}