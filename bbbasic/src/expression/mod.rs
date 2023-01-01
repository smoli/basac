use crate::error::InterpreterError;
use crate::parser::{Add, Div, Expression, Factor, Group, Mul, NumberLiteral, NumberLiteral_value, Sub, Term, Variable};
use crate::scope::Scope;
use crate::value::Value;

pub trait Compute {
    #[allow(unused_variables)]
    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError>;
}

impl Compute for NumberLiteral {
    fn compute(&self, _: &mut Scope) -> Result<Value, InterpreterError> {
        match &self.value {
            NumberLiteral_value::FloatLiteral(f) => Ok(Value::Float(f.parse().unwrap())),
            NumberLiteral_value::IntegerLiteral(i) => Ok(Value::Integer(i.parse().unwrap()))
        }
    }
}

impl Compute for Variable {
    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match scope.get(&self.name) {
            Ok(v) => Ok(v.clone()),
            Err(e) => Err(e)
        }
    }
}

impl Compute for Add {
    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.left.compute(scope)?.add(&self.right.compute(scope)?)
    }
}


impl Compute for Sub {
    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.left.compute(scope)?.sub(&self.right.compute(scope)?)
    }
}

impl Compute for Mul {
    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.left.compute(scope)?.mul(&self.right.compute(scope)?)
    }
}

impl Compute for Div {
    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.left.compute(scope)?.div(&self.right.compute(scope)?)
    }
}

impl Compute for Factor {
    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match self {
            Factor::Group(g) => g.compute(scope),
            Factor::NumberLiteral(n) => n.compute(scope),
            Factor::Variable(v) => v.compute(scope)
        }
    }
}

impl Compute for Term {
    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match self {
            Term::Div(d) => d.compute(scope),
            Term::Factor(f) => f.compute(scope),
            Term::Mul(m) => m.compute(scope)
        }
    }
}

impl Compute for Group {
    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.body.compute(scope)
    }
}

impl Compute for Expression {
    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match self {
            Expression::Add(a) => a.compute(scope),
            Expression::Sub(s) => s.compute(scope),
            Expression::Term(t) => t.compute(scope)
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
        let v = r.compute(&mut s).expect("Computation error");

        assert_eq!(12 + (23 + 2 / 1), v.as_int().unwrap());
    }

    #[test]
    fn expression_with_vars() {
        let r = Expression::parse("12 + (23 + a / 1)").expect("Parse error");
        let mut s = Scope::new();

        s.set(&"a".to_string(), Value::Integer(4));

        let v = r.compute(&mut s).expect("Computation error");

        assert_eq!(12 + (23 + 4 / 1), v.as_int().unwrap());
    }

}