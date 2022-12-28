extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate core;

pub mod interpreter;
pub mod executor;

use interpreter::interpret;
use executor::execute;

fn main() {

    let inp =
"FOR i = 1 TO 100
FOR j = 1 TO 3
PRINT i
PRINT j
NEXT j
NEXT i
PRINT 12";

    let res = interpret(inp).unwrap();

    execute(&res, &mut std::io::stdout());
}



#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn prints_integers() {
        let inp = "PRINT 12";

        let res = interpret(inp).unwrap();

        execute(&res, &mut std::io::stdout());
    }

    #[test]
    fn prints_strings() {
        let inp = "PRINT \"Hello, World!\"";
        // let inp = "PRINT 12";

        let res = interpret(inp).unwrap();

        execute(&res, &mut std::io::stdout());
    }

    #[test]
    fn for_loops() {
        let inp = "FOR i = 1 TO 10
        PRINT i
        NEXT i";
        let res = interpret(inp).unwrap();

        execute(&res, &mut std::io::stdout());
    }
}
