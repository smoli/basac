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


#[test]
fn while_endwhile_premature_exit() {
    let (mut out, exp) = common::make_buffer("0\n1\n2\n");
    let inp =
        "x = 0
WHILE x < 5
PRINT x
x = x + 1
IF x > 2 THEN
EXIT WHILE
ENDIF
ENDWHILE";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());

}

#[test]
fn while_endwhile_nested_exit() {
        let (mut out, exp) = common::make_buffer("00\n");
        let inp =
"x = 0
WHILE x < 3
    FOR y = 0 TO 2
        PRINT x;y
        EXIT WHILE
    NEXT y
    x = x + 1
ENDWHILE";

        let r = parser::Program::parse(inp).expect("Parse failed");

        r.execute(&mut out).expect("Execution failed");

        assert_eq!(out.stringify(), exp.stringify());


}


#[test]
fn while_endwhile_exit_in_for() {
    let (mut out, exp) = common::make_buffer("00\n10\n20\n");
    let inp =
"x = 0
WHILE x < 3
    FOR y = 0 TO 2
        PRINT x;y
        EXIT FOR
    NEXT y
    x = x + 1
ENDWHILE";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());


}
