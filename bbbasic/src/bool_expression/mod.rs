use crate::error::InterpreterError;
use crate::expression::Compute;
use crate::scope::Scope;
use crate::value::Value;
use crate::parser::{BoolCondition, BoolConjunction, BoolDisjunction, BoolExpression, BoolOperand, BoolOperator, BoolTerm, Factor};

pub trait ComputeBool {
    #[allow(unused_variables)]
    fn compute_bool(&self, scope: &mut Scope) -> Result<Value, InterpreterError>;
}


impl ComputeBool for Factor {
    fn compute_bool(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.compute(scope)
    }
}

impl ComputeBool for BoolOperand {
    fn compute_bool(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match self {
            BoolOperand::Expression(e) => e.compute(scope),
            BoolOperand::StringLiteral(s) => Ok(Value::String(s.body.to_string()))
        }
    }
}

impl ComputeBool for BoolCondition {
    fn compute_bool(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match self.op {
            BoolOperator::BoolOpEqual(_) => Ok(Value::Boolean(self.left.compute_bool(scope)?.eq(&self.right.compute_bool(scope)?)?)),
            BoolOperator::BoolOpGreater(_) => Ok(Value::Boolean(self.left.compute_bool(scope)?.gt(&self.right.compute_bool(scope)?)?)),
            BoolOperator::BoolOpGreaterEqual(_) => Err(InterpreterError::NotImplemented),
            BoolOperator::BoolOpLower(_) => Ok(Value::Boolean(self.left.compute_bool(scope)?.lt(&self.right.compute_bool(scope)?)?)),
            BoolOperator::BoolOpLowerEqual(_) => Err(InterpreterError::NotImplemented),
            BoolOperator::BoolOpNotEqual(_) => Err(InterpreterError::NotImplemented)
        }
    }
}

impl ComputeBool for BoolTerm {
    fn compute_bool(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match &self.expression {
            None => {}
            Some(e) => return e.compute_bool(scope)
        };

        match &self.condition {
            None => {}
            Some(c) => return c.compute_bool(scope)
        };

        Err(InterpreterError::NotImplemented)
    }
}

impl ComputeBool for BoolConjunction {
    fn compute_bool(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        for i in 0..self.term.len() {
            match self.term.get(i).unwrap().compute_bool(scope)? {
                Value::Boolean(b) => if b == false { return Ok(Value::Boolean(false)); }
                _ => return Err(InterpreterError::TypeMismatch),
            }
        }

        Ok(Value::Boolean(true))
    }
}

impl ComputeBool for BoolDisjunction {
    fn compute_bool(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        for i in 0..self.conjunction.len() {
            match self.conjunction.get(i).unwrap().compute_bool(scope)? {
                Value::String(_) => return Err(InterpreterError::TypeMismatch),
                Value::Integer(_) => return Err(InterpreterError::TypeMismatch),
                Value::Float(_) => return Err(InterpreterError::TypeMismatch),
                Value::Boolean(b) => if b == true { return Ok(Value::Boolean(true)); }
            }
        }

        Ok(Value::Boolean(false))
    }
}

impl ComputeBool for BoolExpression {
    fn compute_bool(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        self.disjunction.compute_bool(scope)
    }
}

#[cfg(test)]
mod tests {
    use peginator::PegParser;
    use super::*;

    #[test]
    fn bool_expression_true() {
        let r = BoolExpression::parse("12 < 23").expect("Parse failed");
        let mut s = Scope::new();

        let v = r.compute_bool(&mut s).expect("Boolean computation failed");

        assert!(v.as_bool().expect("Not a bool"));
    }

    #[test]
    fn bool_expression_false() {
        let r = BoolExpression::parse("12 > 23").expect("Parse failed");
        let mut s = Scope::new();

        let v = r.compute_bool(&mut s).expect("Boolean computation failed");

        assert!(!v.as_bool().expect("Not a bool"));
    }

    #[test]
    fn bool_expression_conjunction() {
        let r = BoolExpression::parse("12 < 23 AND 12 > 1").expect("Parse failed");
        let mut s = Scope::new();

        let v = r.compute_bool(&mut s).expect("Boolean computation failed");

        assert!(v.as_bool().expect("Not a bool"));
    }

    #[test]
    fn bool_expression_conjunction_false() {
        let r = BoolExpression::parse("12 < 23 AND 12 < 1").expect("Parse failed");
        let mut s = Scope::new();

        let v = r.compute_bool(&mut s).expect("Boolean computation failed");

        assert!(!v.as_bool().expect("Not a bool"));
    }

    #[test]
    fn bool_expression_disjunction() {
        let r = BoolExpression::parse("12 > 23 OR 12 > 1").expect("Parse failed");
        let mut s = Scope::new();

        let v = r.compute_bool(&mut s).expect("Boolean computation failed");

        assert!(v.as_bool().expect("Not a bool"));
    }

    #[test]
    fn bool_expression_disjunction_false() {
        let r = BoolExpression::parse("12 > 23 OR 12 < 1").expect("Parse failed");
        let mut s = Scope::new();

        let v = r.compute_bool(&mut s).expect("Boolean computation failed");

        assert!(!v.as_bool().expect("Not a bool"));
    }


    #[test]
    fn bool_expression_lt() {
        let mut s = Scope::new();

        let r1 = BoolExpression::parse("12 < 13").expect("Parse failed");
        let v1 = r1.compute_bool(&mut s).expect("Boolean computation failed");
        assert!(v1.as_bool().expect("Not a bool"));

        let r2 = BoolExpression::parse("12 < 11").expect("Parse failed");
        let v2 = r2.compute_bool(&mut s).expect("Boolean computation failed");
        assert!(!v2.as_bool().expect("Not a bool"));
    }


    #[test]
    fn bool_expression_gt() {
        let mut s = Scope::new();

        let r1 = BoolExpression::parse("12 > 10 + 1").expect("Parse failed");
        let v1 = r1.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v1);
        assert!(v1.as_bool().expect("Not a bool"));

        let r2 = BoolExpression::parse("12 > 10 + 4").expect("Parse failed");
        let v2 = r2.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v2);
        assert!(!v2.as_bool().expect("Not a bool"));
    }

    #[test]
    fn bool_expression_eq() {
        let mut s = Scope::new();

        let r1 = BoolExpression::parse("12 = 12.0").expect("Parse failed");
        let v1 = r1.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v1);
        assert!(v1.as_bool().expect("Not a bool"));

        let r2 = BoolExpression::parse("12 = 14").expect("Parse failed");
        let v2 = r2.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v2);
        assert!(!v2.as_bool().expect("Not a bool"));

        let r3 = BoolExpression::parse("\"ABC\" = \"ABC\"").expect("Parse failed");
        let v3 = r3.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v3);
        assert!(v3.as_bool().expect("Not a bool"));
    }

    #[test]
    fn comparing_variables() {
        let mut s = Scope::new();

        s.set(&"a".to_string(), Value::Integer(1));
        s.set(&"b".to_string(), Value::Integer(1));

        let r1 = BoolExpression::parse("a = b").expect("Parse failed");
        let v1 = r1.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v1);
        assert!(v1.as_bool().expect("Not a bool"));
    }

    #[test]
    fn comparing_expressions() {
        let mut s = Scope::new();

        s.set(&"a".to_string(), Value::Integer(1));
        s.set(&"b".to_string(), Value::Integer(1));

        let r = BoolExpression::parse("a * 2 = b * 2").expect("Parse failed");
        let v = r.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v);
        assert!(v.as_bool().expect("Not a bool"));

        let r = BoolExpression::parse("3 > b * 2").expect("Parse failed");
        let v = r.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v);
        assert!(v.as_bool().expect("Not a bool"));
    }

    #[test]
    fn comparing_strings() {
        let mut s = Scope::new();

        s.set(&"a".to_string(), Value::String("ABC".to_string()));


        let r = BoolExpression::parse("a = \"ABC\"").expect("Parse failed");
        let v = r.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v);
        assert!(v.as_bool().expect("Not a bool"));

        let r = BoolExpression::parse("\"ABC\" = a").expect("Parse failed");
        let v = r.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v);
        assert!(v.as_bool().expect("Not a bool"));

        let r = BoolExpression::parse("\"ABC\" = \"ABC\"").expect("Parse failed");
        let v = r.compute_bool(&mut s).expect("Boolean computation failed");
        println!("{:?}", v);
        assert!(v.as_bool().expect("Not a bool"));
    }
}