extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::{Pair, Pairs};
use pest::Parser;
use crate::BBStatement::Nop;

#[derive(Parser)]
#[grammar="bbbasic.pest"]
pub struct BBasicParser;


#[derive(Debug)]
enum InterpreterError {
    Generic(String),

}

enum BBExpression {
    String(String),
    Integer(i64),
    Float(f64)
}

enum BBStatement {
    PRINT(BBExpression),
    Nop
}

fn interpret_expression(pairs: Pairs<Rule>) -> Result<BBExpression, InterpreterError> {
    for p in pairs {
        return match p.as_rule() {
            Rule::bb_float_literal => Ok(BBExpression::Float(p.as_str().parse::<f64>().unwrap())),
            Rule::bb_int_literal => Ok(BBExpression::Integer(p.as_str().parse::<i64>().unwrap())),
            Rule::bb_string => Ok(BBExpression::String(p.as_str().to_string())),

            _ => {
                println!("{:?}", p);
                Err(InterpreterError::Generic("What do I know".to_string()))
            }
        }
    }

    return Err(InterpreterError::Generic("Empty Expression".to_string()))
}

fn interpret_statement(pair: Pair<Rule>) -> Result<BBStatement, InterpreterError> {

    return match pair.as_rule() {
        Rule::bb_print_statement => Ok(BBStatement::PRINT(interpret_expression(pair.into_inner()).unwrap())),
        Rule::EOI => Ok(Nop),

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

fn interpret_program(pair: Pair<Rule>) -> Result<Vec<BBStatement>, InterpreterError> {
    match pair.as_rule() {
        Rule::bb_program => interpret_statements(pair.into_inner()),
        _ => Err(InterpreterError::Generic(format!("{:?}", &pair)))
    }
}

fn interpret(parse: Pairs<Rule>) {

    for t in parse {
        match interpret_program(t) {
            Ok(s) => run(&s),
            Err(e) => {}
        }
    }

}

fn run(statements: &Vec<BBStatement>) {

    for s in statements {
        match s {
            BBStatement::PRINT(e) => {
                match e {
                    BBExpression::String(s) => println!("{s}"),
                    BBExpression::Integer(i) => println!("{i}"),
                    BBExpression::Float(f) => println!("{f}")
                }
            }
            Nop => {}
        }
    }
}


fn main() {
    let parse = BBasicParser::parse(Rule::bb_program, "PRINT \"Hello, World!\"");

    match parse {
        Ok(res) => interpret(res),
        Err(e) => println!("{e}")
    }
}


/*
  Tests:
  FOR `=1.4 TO 3.2
PRINT "S";`
NEXT `
END
 yields S1.4 S2.4

 */