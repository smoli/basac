use crate::error::InterpreterError;
use crate::error::InterpreterError::NotImplemented;
use crate::parser::{Expression, Factor, Group, NumberLiteral, NumberLiteral_value, NumericVariable, NumericVariable_type_dem, Term};
use crate::scope::{Byte, DataType, Float, Integer, Scope};
use crate::value::Value;

pub trait Compute {
    #[allow(unused_variables)]
    fn compute_float(&self, scope: &mut Scope) -> Result<Float, InterpreterError> {
        Err(NotImplemented("Base implementation compute".to_string()))
    }

    #[allow(unused_variables)]
    fn compute_integer(&self, scope: &mut Scope) -> Result<Integer, InterpreterError> {
        Err(NotImplemented("Base implementation compute".to_string()))
    }

    #[allow(unused_variables)]
    fn compute_byte(&self, scope: &mut Scope) -> Result<Byte, InterpreterError> {
        Err(NotImplemented("Base implementation compute".to_string()))
    }

    fn get_type(&self) -> DataType;

    fn compute(&self, scope: &mut Scope) -> Result<Value, InterpreterError> {
        match self.get_type() {
            DataType::Byte => Ok(Value::Byte(self.compute_byte(scope)?)),
            DataType::Integer => Ok(Value::Integer(self.compute_integer(scope)?)),
            DataType::Float => Ok(Value::Float(self.compute_float(scope)?)),
            _ => Err(InterpreterError::TypeMismatch)
        }
    }
}

/*
impl Compute for IntegerLiteral {
    fn compute<T>(&self, _: &mut Scope) -> Result<T, InterpreterError> {
        Ok(self.body.parse::<Integer>().unwrap() as FromStr)
    }
}

impl Compute for FloatLiteral {
    fn compute<T>(&self, _: &mut Scope) -> Result<T, InterpreterError> {
        Ok(self.body.parse().unwrap())
    }
}*/

impl Compute for NumberLiteral {
    fn compute_float(&self, _: &mut Scope) -> Result<Float, InterpreterError> {
        match &self.value {
            NumberLiteral_value::FloatLiteral(f) => match f.body.parse() {
                Err(_) => Err(InterpreterError::TypeMismatch),
                Ok(v) => Ok(v)
            },

            NumberLiteral_value::IntegerLiteral(f) => match f.body.parse::<Integer>() {
                Err(_) => Err(InterpreterError::TypeMismatch),
                Ok(v) => Ok(v as Float)
            },
        }
    }

    fn compute_integer(&self, _: &mut Scope) -> Result<Integer, InterpreterError> {
        match &self.value {
            NumberLiteral_value::IntegerLiteral(f) => match f.body.parse() {
                Err(_) => Err(InterpreterError::TypeMismatch),
                Ok(v) => Ok(v)
            },

            NumberLiteral_value::FloatLiteral(f) => match f.body.parse::<Float>() {
                Err(_) => Err(InterpreterError::TypeMismatch),
                Ok(v) => Ok(v as Integer)
            },
        }
    }

    fn get_type(&self) -> DataType {
        match self.value {
            NumberLiteral_value::FloatLiteral(_) => DataType::Float,
            NumberLiteral_value::IntegerLiteral(_) => DataType::Integer
        }
    }
}

impl Compute for NumericVariable {
    fn compute_float(&self, scope: &mut Scope) -> Result<Float, InterpreterError> {
        match &self.type_dem {
            None => Ok(scope.get_float(&self.name)?),

            Some(d) => match d {
                NumericVariable_type_dem::ByteDenominator(_) => Ok(scope.get_byte(&self.name)? as Float),
                NumericVariable_type_dem::FloatDenominator(_) => Ok(scope.get_float(&self.name)?),
                NumericVariable_type_dem::IntegerDenominator(_) => Ok(scope.get_int(&self.name)? as Float)
            }
        }
    }

    fn compute_integer(&self, scope: &mut Scope) -> Result<Integer, InterpreterError> {
        match &self.type_dem {
            None => Ok(scope.get_int(&self.name)?),

            Some(d) => match d {
                NumericVariable_type_dem::ByteDenominator(_) => Ok(scope.get_byte(&self.name)? as Integer),
                NumericVariable_type_dem::FloatDenominator(_) => Ok(scope.get_float(&self.name)? as Integer),
                NumericVariable_type_dem::IntegerDenominator(_) => Ok(scope.get_int(&self.name)?)
            }
        }
    }

    fn compute_byte(&self, scope: &mut Scope) -> Result<Byte, InterpreterError> {
        match &self.type_dem {
            None => Ok(scope.get_byte(&self.name)?),

            Some(d) => match d {
                NumericVariable_type_dem::ByteDenominator(_) => Ok(scope.get_byte(&self.name)?),
                NumericVariable_type_dem::FloatDenominator(_) => Ok(scope.get_float(&self.name)? as Byte),
                NumericVariable_type_dem::IntegerDenominator(_) => Ok(scope.get_int(&self.name)? as Byte)
            }
        }
    }

    fn get_type(&self) -> DataType {
        match &self.type_dem {
            None => DataType::Float,

            Some(d) => match d {
                NumericVariable_type_dem::ByteDenominator(_) => DataType::Byte,
                NumericVariable_type_dem::FloatDenominator(_) => DataType::Float,
                NumericVariable_type_dem::IntegerDenominator(_) => DataType::Integer
            }
        }
    }
}

impl Compute for Factor {
    fn compute_float(&self, scope: &mut Scope) -> Result<Float, InterpreterError> {
        match &self {
            Factor::Group(g) => g.compute_float(scope),
            Factor::NumberLiteral(n) => n.compute_float(scope),
            Factor::NumericVariable(v) => match &v.type_dem {
                None => Ok(scope.get_float(&v.name.to_string())?),
                Some(d) => match d {
                    NumericVariable_type_dem::ByteDenominator(_) => Ok(scope.get_byte(&v.name.to_string())? as Float),
                    NumericVariable_type_dem::FloatDenominator(_) => Ok(scope.get_float(&v.name.to_string())?),
                    NumericVariable_type_dem::IntegerDenominator(_) => Ok(scope.get_int(&v.name.to_string())? as Float),
                }
            }
        }
    }

    fn compute_integer(&self, scope: &mut Scope) -> Result<Integer, InterpreterError> {
        match &self {
            Factor::Group(g) => g.compute_integer(scope),
            Factor::NumberLiteral(n) => n.compute_integer(scope),
            Factor::NumericVariable(v) => match &v.type_dem {
                None => Ok(scope.get_float(&v.name.to_string())? as Integer),
                Some(d) => match d {
                    NumericVariable_type_dem::ByteDenominator(_) => Ok(scope.get_byte(&v.name.to_string())? as Integer),
                    NumericVariable_type_dem::FloatDenominator(_) => Ok(scope.get_float(&v.name.to_string())? as Integer),
                    NumericVariable_type_dem::IntegerDenominator(_) => Ok(scope.get_int(&v.name.to_string())?),
                }
            }
        }
    }

    fn compute_byte(&self, scope: &mut Scope) -> Result<Byte, InterpreterError> {
        match self {
            Factor::Group(g) => g.compute_byte(scope),
            Factor::NumberLiteral(n) => n.compute_byte(scope),
            Factor::NumericVariable(v) => match &v.type_dem {
                None => Ok(scope.get_float(&v.name.to_string())? as Byte),
                Some(d) => match d {
                    NumericVariable_type_dem::ByteDenominator(_) => Ok(scope.get_byte(&v.name.to_string())?),
                    NumericVariable_type_dem::FloatDenominator(_) => Ok(scope.get_float(&v.name.to_string())? as Byte),
                    NumericVariable_type_dem::IntegerDenominator(_) => Ok(scope.get_int(&v.name.to_string())? as Byte),
                }
            }
        }
    }

    fn get_type(&self) -> DataType {
        match &self {
            Factor::Group(g) => g.get_type(),
            Factor::NumberLiteral(l) => l.get_type(),
            Factor::NumericVariable(v) => match &v.type_dem {
                None => DataType::Float,
                Some(d) => match d {
                    NumericVariable_type_dem::ByteDenominator(_) => DataType::Byte,
                    NumericVariable_type_dem::FloatDenominator(_) => DataType::Float,
                    NumericVariable_type_dem::IntegerDenominator(_) => DataType::Integer
                }
            }
        }
    }
}

impl Compute for Term {
    fn compute_float(&self, scope: &mut Scope) -> Result<Float, InterpreterError> {
        match self {
            Term::Div(d) => Ok(d.left.compute_float(scope)? / d.right.compute_float(scope)?),
            Term::Factor(f) => f.compute_float(scope),
            Term::Mul(m) => Ok(m.left.compute_float(scope)? * m.right.compute_float(scope)?)
        }
    }

    fn compute_integer(&self, scope: &mut Scope) -> Result<Integer, InterpreterError> {
        match self {
            Term::Div(d) => Ok(d.left.compute_integer(scope)? / d.right.compute_integer(scope)?),
            Term::Factor(f) => f.compute_integer(scope),
            Term::Mul(m) => Ok(m.left.compute_integer(scope)? * m.right.compute_integer(scope)?)
        }
    }

    fn compute_byte(&self, scope: &mut Scope) -> Result<Byte, InterpreterError> {
        match self {
            Term::Div(d) => Ok(d.left.compute_byte(scope)? / d.right.compute_byte(scope)?),
            Term::Factor(f) => f.compute_byte(scope),
            Term::Mul(m) => Ok(m.left.compute_byte(scope)? * m.right.compute_byte(scope)?)
        }
    }

    fn get_type(&self) -> DataType {
        let l: u16;
        let r: u16;

        match self {
            Term::Div(d) => {
                l = d.left.get_type() as u16;
                r = d.right.get_type() as u16;
            }

            Term::Mul(d) => {
                l = d.left.get_type() as u16;
                r = d.right.get_type() as u16;
            }

            Term::Factor(d) => {
                return d.get_type();
            }
        };


        match u16::max(l, r) {
            1 => DataType::Byte,
            2 => DataType::Integer,
            3 => DataType::Float,
            _ => panic!("Wrong Type!")
        }
    }
}

impl Compute for Group {
    fn compute_float(&self, scope: &mut Scope) -> Result<Float, InterpreterError> {
        self.body.compute_float(scope)
    }
    fn compute_integer(&self, scope: &mut Scope) -> Result<Integer, InterpreterError> {
        self.body.compute_integer(scope)
    }
    fn compute_byte(&self, scope: &mut Scope) -> Result<Byte, InterpreterError> {
        self.body.compute_byte(scope)
    }

    fn get_type(&self) -> DataType {
        self.body.get_type()
    }
}

impl Compute for Expression {
    fn compute_float(&self, scope: &mut Scope) -> Result<Float, InterpreterError> {
        match self {
            Expression::Add(a) => Ok(a.left.compute_float(scope)? + a.right.compute_float(scope)?),
            Expression::Sub(s) => Ok(s.left.compute_float(scope)? - s.right.compute_float(scope)?),
            Expression::Term(t) => t.compute_float(scope)
        }
    }

    fn compute_integer(&self, scope: &mut Scope) -> Result<Integer, InterpreterError> {
        match self {
            Expression::Add(a) => Ok(a.left.compute_integer(scope)? + a.right.compute_integer(scope)?),
            Expression::Sub(s) => Ok(s.left.compute_integer(scope)? - s.right.compute_integer(scope)?),
            Expression::Term(t) => t.compute_integer(scope)
        }
    }

    fn compute_byte(&self, scope: &mut Scope) -> Result<Byte, InterpreterError> {
        match self {
            Expression::Add(a) => Ok(a.left.compute_byte(scope)? + a.right.compute_byte(scope)?),
            Expression::Sub(s) => Ok(s.left.compute_byte(scope)? - s.right.compute_byte(scope)?),
            Expression::Term(t) => t.compute_byte(scope)
        }
    }

    fn get_type(&self) -> DataType {
        let l: u16;
        let r: u16;

        match self {
            Expression::Add(d) => {
                l = d.left.get_type() as u16;
                r = d.right.get_type() as u16;
            }

            Expression::Sub(d) => {
                l = d.left.get_type() as u16;
                r = d.right.get_type() as u16;
            }

            Expression::Term(d) => {
                return d.get_type();
            }
        };


        match u16::max(l, r) {
            1 => DataType::Byte,
            2 => DataType::Integer,
            3 => DataType::Float,
            _ => panic!("Wrong Type!")
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
        let v = r.compute_integer(&mut s).expect("Computation error");

        assert_eq!(12 + (23 + 2 / 1), v);
    }

    #[test]
    fn expression_with_vars() {
        let r = Expression::parse("12 + (23 + a / 1)").expect("Parse error");
        let mut s = Scope::new();

        s.set_float(&"a".to_string(), 4.0);

        let v = r.compute_integer(&mut s).expect("Computation error");

        assert_eq!(12 + (23 + 4 / 1), v);
    }
}