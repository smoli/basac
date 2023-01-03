use bbbasic::parser;
use peginator::PegParser;
use crate::common::Stringify;

mod common;

#[test]
fn float_var() {
    let (mut out, exp) = common::make_buffer("2\n3\n");
    let inp =
"x = 2
xyz# = 3
PRINT x
PRINT xyz#";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());

}

#[test]
fn int_var() {
    let (mut out, exp) = common::make_buffer("2\n");
    let inp =
"x% = 2
PRINT x%";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());

}
#[test]
fn string_var() {
    let (mut out, exp) = common::make_buffer("Yo!\n");
    let inp =
"x$ = \"Yo!\"
PRINT x$";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());

}
