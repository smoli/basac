extern crate pest;
#[macro_use]
extern crate pest_derive;

pub mod interpreter;

use interpreter::interpret;

pub mod executor;

use executor::execute;

fn main() {

    let inp =
"b = 12
a = 13
PRINT b
PRINT a";

    let res = interpret(inp).unwrap();

    execute(&res);
}


/*
  Tests:
  FOR `=1.4 TO 3.2
PRINT "S";`
NEXT `
END
 yields S1.4 S2.4

 */