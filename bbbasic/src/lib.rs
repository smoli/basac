extern crate peginator;
use std::io::Write;

pub mod parser;
pub mod interpreter;
mod value;
mod error;
mod expression;
mod scope;
mod bool_expression;

use std::io::stdout;
use peginator::PegParser;

pub fn execute(code: &str) {
    let parse_result = parser::Program::parse(code).expect("Parse Error!");

    parse_result.execute(&mut stdout()).expect("Execution Error");
}