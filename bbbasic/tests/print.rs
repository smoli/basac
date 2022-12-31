use bbbasic::parser;
use peginator::PegParser;
use bbbasic::interpreter::execute;

mod common;

#[test]
fn print_a_string() {

    let (mut out, exp) = common::make_buffer("Hello, World!");
    let r = parser::Program::parse("PRINT \"Hello, World!\"").expect("Parse failed");

    println!("{:?}", r);
    execute(&r, &mut out);

    assert_eq!(out.into_inner(), exp.into_inner())
}

#[test]
fn print_an_integer() {

    let (mut out, exp) = common::make_buffer("12");
    let r = parser::Program::parse("PRINT 12").expect("Parse failed");

    println!("{:?}", r);
    execute(&r, &mut out);

    assert_eq!(out.into_inner(), exp.into_inner())
}

#[test]
fn print_a_float() {

    let (mut out, exp) = common::make_buffer("12.012");
    let r = parser::Program::parse("PRINT 12.012").expect("Parse failed");

    println!("{:?}", r);
    execute(&r, &mut out);

    assert_eq!(out.into_inner(), exp.into_inner())
}