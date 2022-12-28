use bbbasic::executor::execute;
use bbbasic::interpreter::interpret;

mod common;

#[test]
fn prints_integers() {
    let (mut out, exp) = common::make_buffer("12\n");
    let inp = "PRINT 12";

    let res = interpret(inp).unwrap();

    execute(&res, &mut out);

    assert_eq!(out.into_inner(), exp.into_inner())
}

#[test]
fn prints_strings() {
    let (mut out, exp) = common::make_buffer("Hello, World!\n");
    let inp = "PRINT \"Hello, World!\"";

    let res = interpret(inp).unwrap();

    execute(&res, &mut out);

    assert_eq!(out.into_inner(), exp.into_inner());
}