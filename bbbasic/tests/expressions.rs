use bbbasic::parser;
use peginator::PegParser;

mod common;

#[test]
fn parse_an_expression() {

    let r = parser::Program::parse("x = 12 * (4 + a)").expect("Parse failed");
    println!("{:?}", r);
    assert!(true);
}

