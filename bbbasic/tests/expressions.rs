use bbbasic::parser;
use peginator::PegParser;

mod common;

#[test]
fn parse_an_expression() {

    let _ = parser::Program::parse("x = 12 * (4 + a)").expect("Parse failed");
    assert!(true);
}

