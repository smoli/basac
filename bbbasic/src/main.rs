extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod interpreter;

use interpreter::BBStatement;
use interpreter::interpret;
use interpreter::BBExpression;




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
            BBStatement::Nop => {}
        }
    }
}


fn main() {

    let res = interpret("PRINT 12");

    match res {
        Ok(statements) => run(&statements),
        Err(e) => println!("{:?}", e)
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