use bbbasic::parser;
use peginator::PegParser;
use crate::common::Stringify;

mod common;

#[test]
fn while_endwhile() {
    let (mut out, exp) = common::make_buffer("0\n1\n2\n3\n4\n");
    let inp =
"x = 0
WHILE x < 5
PRINT x
x = x + 1
ENDWHILE";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());

}
