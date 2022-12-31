use crate::error::InterpreterError;
use crate::parser::{Add, Div, Expression, Factor, Group, IntegerLiteral, Mul, NumberLiteral, NumberLiteral_value, Sub, Term};
use crate::scope::Scope;
use crate::value::Value;

pub trait Compute {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError>;
}

impl Compute for NumberLiteral {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match &self.value {
            NumberLiteral_value::FloatLiteral(f) => f.ausrechnen(scope),
            NumberLiteral_value::IntegerLiteral(i) => i.ausrechnen(scope)
        }
    }
}


impl Compute for IntegerLiteral {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        Ok(Value::Integer(self.parse().unwrap()))
    }
}

impl Compute for Add {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.left.ausrechnen(scope)?.add(&self.right.ausrechnen(scope)?)
    }
}


impl Compute for Sub {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.left.ausrechnen(scope)?.sub(&self.right.ausrechnen(scope)?)
    }
}

impl Compute for Mul {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.left.ausrechnen(scope)?.mul(&self.right.ausrechnen(scope)?)
    }
}

impl Compute for Div {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.left.ausrechnen(scope)?.div(&self.right.ausrechnen(scope)?)
    }
}

impl Compute for Factor {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match self {
            Factor::Group(g) => g.ausrechnen(scope),
            Factor::NumberLiteral(n) => n.ausrechnen(scope),
            Factor::VariableName(v) => v.ausrechnen(scope)
        }
    }
}

impl Compute for Term {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match self {
            Term::Div(d) => d.ausrechnen(scope),
            Term::Factor(f) => f.ausrechnen(scope),
            Term::Mul(m) => m.ausrechnen(scope)
        }
    }
}

impl Compute for Group {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.body.ausrechnen(scope)
    }
}

impl Compute for Expression {
    fn ausrechnen(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match self {
            Expression::Add(a) => a.ausrechnen(scope),
            Expression::Sub(s) => s.ausrechnen(scope),
            Expression::Term(t) => t.ausrechnen(scope)
        }
    }
}

#[cfg(test)]
mod tests {
    use peginator::PegParser;
    use super::*;

    #[test]
    fn expression_can_be_parsed_and_computed() {
        let r = Expression::parse("12 + (23 + 2 / 1)").expect("Parse error");
        let mut s = Scope::new();
        let v = r.ausrechnen(&mut s).expect("Computation error");

        assert_eq!(12 + (23 + 2 / 1), v.as_int().unwrap());
    }


}