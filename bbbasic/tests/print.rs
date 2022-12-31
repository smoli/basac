use bbbasic::parser;
use peginator::PegParser;

mod common;

#[test]
fn print_a_string() {

    let (mut out, exp) = common::make_buffer("Hello, World!\n");
    let r = parser::Program::parse("PRINT \"Hello, World!\"").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.into_inner(), exp.into_inner())
}

#[test]
fn print_an_integer() {

    let (mut out, exp) = common::make_buffer("12\n");
    let r = parser::Program::parse("PRINT 12").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.into_inner(), exp.into_inner())
}

#[test]
fn print_a_float() {

    let (mut out, exp) = common::make_buffer("12.123\n");
    let r = parser::Program::parse("PRINT 12.123").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.into_inner(), exp.into_inner())
}


#[test]
fn print_an_expression() {
    let (mut out, exp) = common::make_buffer("60\n");
    let r = parser::Program::parse("PRINT 12 + (23 * 2 + 4 / 2)").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.into_inner(), exp.into_inner())
}

#[test]
fn print_multiline() {
    let (mut out, exp) = common::make_buffer("1\n2\n");
    let r = parser::Program::parse("PRINT 1\nPRINT 2\n").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.into_inner(), exp.into_inner())
}


#[test]
fn print_skip_new_line() {
    let (mut out, exp) = common::make_buffer("12\n");
    let r = parser::Program::parse("PRINT 1;\nPRINT 2\n").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.into_inner(), exp.into_inner())
}