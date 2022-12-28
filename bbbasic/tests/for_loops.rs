use bbbasic::executor::execute;
use bbbasic::interpreter::interpret;


mod common;

#[test]
fn for_loops() {

    let inp = "FOR i = 1 TO 4
        PRINT i
        NEXT i";

    let exp = ["1", "2", "3", "4", ""].join("\n");

    let (mut out, exp) = common::make_buffer(exp.as_str());

    let res = interpret(inp).unwrap();

    execute(&res, &mut out);

    assert_eq!(out.into_inner(), exp.into_inner());
}

#[test]
fn for_loop_with_step() {
    let inp = "FOR i = 1 TO 8 STEP 2
        PRINT i
        NEXT i";

    let exp = ["1", "3", "5", "7", ""].join("\n");

    let (mut out, exp) = common::make_buffer(exp.as_str());

    let res = interpret(inp).unwrap();

    execute(&res, &mut out);

    assert_eq!(out.into_inner(), exp.into_inner());
}

#[test]
fn for_loop_with_step_from_var() {
    let inp = "s = 2
        FOR i = 1 TO 8 STEP s
        PRINT i
        NEXT i";

    let exp = ["1", "3", "5", "7", ""].join("\n");

    let (mut out, exp) = common::make_buffer(exp.as_str());

    let res = interpret(inp).unwrap();

    execute(&res, &mut out);

    assert_eq!(out.into_inner(), exp.into_inner());
}