use bbbasic::parser;
use peginator::PegParser;
use crate::common::Stringify;

mod common;

#[test]
fn test1() {
    let (mut out, exp) = common::make_buffer("");
    let inp =
"W = 80
H = 20
MW = 2.47
MH = 2.24
MAXIT = 1000

FOR Py = 1 TO H
    FOR Px = 1 TO W
        x0 = (Px / W) * MW - 2.0
        y0 = (Py / H) * MH - 1.12

        x = 0.0
        y = 0.0
        iteration = 0

        WHILE x * x  + y * y <= 4 AND iteration < MAXIT
            xtemp = x * x - y * y + x0
            y = 2 * x * y + y0
            x = xtemp

            iteration = iteration + 1
        ENDWHILE

    NEXT x
    PRINT \"\"
NEXT y";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out).expect("Execution failed");

    assert_eq!(1, 1);

}


