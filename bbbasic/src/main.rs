extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod interpreter;

use interpreter::BBStatement;
use interpreter::interpret;
use interpreter::BBExpression;
use crate::interpreter::{BBAssignment, BBBlock};


fn run_for_loop(assignment: &BBAssignment, body: &BBBlock) {
    println!("A for loop");
}


fn run(statements: &Vec<BBStatement>) {

    for s in statements {
        match s {
            BBStatement::PRINT(e) => {
                match e {
                    BBExpression::String(s) => println!("{}", s),
                    BBExpression::Integer(i) => println!("{}", i),
                    BBExpression::Float(f) => println!("{}", f)
                }
            },

            // BBStatement::FOR(a, b) => run_for_loop(a, b),

            BBStatement::Nop => {}

            _ => {}
        }
    }
}


fn main() {

    let inp =
"FOR i=1 TO 10
    PRINT 12
    NEXT i
END";

    let res = interpret(inp);

    for r in res {
        println!("{:?}", r);
    }

    // match res {
    //     Ok(statements) => run(&statements),
    //     Err(e) => println!("{:?}", e)
    // }
}


/*
  Tests:
  FOR `=1.4 TO 3.2
PRINT "S";`
NEXT `
END
 yields S1.4 S2.4

 */