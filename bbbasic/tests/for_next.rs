use bbbasic::parser;
use peginator::PegParser;
use crate::common::Stringify;

mod common;

#[test]
fn for_loop() {
    let (mut out, exp) = common::make_buffer("1\n2\n3\n4\n");
    let inp =
"FOR i = 1 TO 4
    PRINT i
NEXT i
";

    let r = parser::Program::parse(inp).expect("Parse failed");
    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());

}

#[test]
fn for_loop_with_step() {
    let (mut out, exp) = common::make_buffer("1\n3\n5\n7\n9\n");
    let inp =
"FOR i = 1 TO 9 STEP 2
    PRINT i
NEXT i
";

    let r = parser::Program::parse(inp).expect("Parse failed");
    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());

}
#[test]
fn for_loop_with_body_manipulating_var() {
    let (mut out, exp) = common::make_buffer("1\n3\n5\n7\n9\n");
    let inp =
"FOR i = 1 TO 9
    PRINT i
    i = i + 1
NEXT i
";

    let r = parser::Program::parse(inp).expect("Parse failed");
    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());
}

#[test]
fn for_loop_premature_exit() {
    let (mut out, exp) = common::make_buffer("1\n2\n3\n4\n5\n");
    let inp =
"FOR i = 1 TO 9
    PRINT i
    IF i > 4 THEN
        EXIT FOR
    ENDIF
NEXT i
";


    let r = parser::Program::parse(inp).expect("Parse failed");
    println!("{:#?}", r);
    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());
}
