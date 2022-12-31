use bbbasic::parser;
use peginator::PegParser;

mod common;

#[test]
fn parse_an_expression() {

    let (mut out, exp) = common::make_buffer("Hello, World!");
    let r = parser::Program::parse_with_trace("x = 12 * (4 + a)").expect("Parse failed");

    println!("{:?}", r);
    assert_eq!(out.into_inner(), exp.into_inner())
}

