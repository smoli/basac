use bbbasic::parser;
use peginator::PegParser;
use crate::common::Stringify;

mod common;

#[test]
fn print_a_string() {

    let (mut out, exp) = common::make_buffer("Hello, World!\n");
    let r = parser::Program::parse("PRINT \"Hello, World!\"").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_an_integer() {

    let (mut out, exp) = common::make_buffer("12\n");
    let r = parser::Program::parse("PRINT 12").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_a_float() {

    let (mut out, exp) = common::make_buffer("12.123\n");
    let r = parser::Program::parse("PRINT 12.123").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}


#[test]
fn print_an_expression() {
    let (mut out, exp) = common::make_buffer("60\n");
    let r = parser::Program::parse("PRINT 12 + (23 * 2 + 4 / 2)").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_multiline() {
    let (mut out, exp) = common::make_buffer("1\n2\n");
    let r = parser::Program::parse("PRINT 1\nPRINT 2\n").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}


#[test]
fn print_skip_new_line() {
    let (mut out, exp) = common::make_buffer("12\n");
    let r = parser::Program::parse("PRINT 1;\nPRINT 2\n").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_an_int_variable() {
    let (mut out, exp) = common::make_buffer("32\n");
    let r = parser::Program::parse("x = 32\nPRINT x\n").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_a_float_variable() {
    let (mut out, exp) = common::make_buffer("32.1\n");
    let r = parser::Program::parse("x = 32.1\nPRINT x\n").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_multiple_values_skipped() {
    let (mut out, exp) = common::make_buffer("123\n");
    let r = parser::Program::parse("PRINT 1;2;3\n").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_multiple_values_unskipped() {
    let (mut out, exp) = common::make_buffer("1\n2\n3\n");
    let r = parser::Program::parse("PRINT 1 2 3\n").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_multiple_strings() {
    let (mut out, exp) = common::make_buffer("1\n2\n3\n");
    let r = parser::Program::parse("PRINT \"1\" \"2\" \"3\"\n").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_multiple_strings_skipped() {
    let (mut out, exp) = common::make_buffer("123\n");
    let r = parser::Program::parse("PRINT \"1\";\"2\";\"3\"\n").expect("Parse failed");
    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_multiple_types() {
    let (mut out, exp) = common::make_buffer("x1a1.02b4c9\n");
    let r = parser::Program::parse("x = 9\nPRINT \"x\";1;\"a\";1.02;\"b\";2*2;\"c\";x\n").expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify())
}

#[test]
fn print_a_string_variable() {
    let (mut out, exp) = common::make_buffer("Hello, World!\n");
    let inp =
"x = \"World\"
PRINT \"Hello, \";x;\"!\"";

    let r = parser::Program::parse(inp).expect("Parse failed");

    r.execute(&mut out);

    assert_eq!(out.stringify(), exp.stringify());

}