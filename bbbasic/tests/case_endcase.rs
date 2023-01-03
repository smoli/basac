use bbbasic::parser;
use peginator::PegParser;
use crate::common::Stringify;

mod common;

#[test]
fn case_when() {
    let (mut out, exp) = common::make_buffer("b\na\n");
    let inp =
"x = 2
CASE x OF
    WHEN 1.0,3.0 : PRINT \"a\"
    WHEN 2.0:
    PRINT \"b\"
ENDCASE
";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out).expect("Execution failed");

    assert_eq!(out.stringify(), exp.stringify());

}

