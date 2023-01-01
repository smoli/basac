use bbbasic::parser;
use peginator::PegParser;
use crate::common::Stringify;

mod common;

#[test]
fn if_then() {
    let (mut out, exp) = common::make_buffer("1\n");
    let inp =
"x = 2
IF x = 2 THEN
PRINT 1
ELSE
PRINT 2
ENDIF";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify());

}

#[test]
fn if_then_else() {
    let (mut out, exp) = common::make_buffer("2\n");
    let inp =
        "x = 1
IF x = 2 THEN
PRINT 1
ELSE



PRINT 2


ENDIF";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify());
}

#[test]
fn if_then_no_else() {
    let (mut out, exp) = common::make_buffer("");
    let inp =
        "x = 1
IF x = 2 THEN
PRINT 1
ENDIF";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify());
}