use crate::error::InterpreterError;
use crate::error::InterpreterError::TypeMismatch;
use crate::scope::{Byte, Float, Integer};

#[derive(Debug,Clone)]
pub enum Value {
    String(String),
    Integer(Integer),
    Float(Float),
    Byte(Byte),
    Boolean(bool)
}

impl Value {

    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<Integer> {
        match self {
            Value::String(s) => Some(s.parse::<Integer>().unwrap()),
            Value::Integer(i) => Some(*i),
            Value::Float(f) => Some(*f as Integer),
            Value::Boolean(_) => None,
            Value::Byte(s) => Some(*s as Integer)
        }
    }

    #[allow(dead_code)]
    pub fn as_bool(&self) -> Result<bool, InterpreterError> {
        match self {
            Value::Boolean(b) => Ok(*b),
            _ => Err(InterpreterError::TypeMismatch)
        }
    }

    pub fn eq(&self, rhs:&Value) -> Result<bool, InterpreterError> {
        match &self {
            Value::Integer(i) => match rhs {
                Value::Integer(rhs) => Ok(*i == *rhs),
                Value::Float(rhs) => Ok(*i as Float == *rhs),
                Value::Byte(rhs) => Ok(*i == *rhs as Integer),
                _ => Err(TypeMismatch)
            }
            Value::Float(f) => match rhs {
                Value::Integer(rhs) => Ok(*f == *rhs as Float),
                Value::Float(rhs) => Ok(*f == *rhs),
                Value::Byte(rhs) => Ok(*f == *rhs as Float),
                Value::String(_) => Err(TypeMismatch),
                Value::Boolean(_) => Err(TypeMismatch),
            }
            Value::String(lhs) => match rhs {
                Value::String(rhs) => Ok(lhs == rhs),
                _ => Err(TypeMismatch)
            },
            Value::Byte(lhs) => match rhs {
                Value::Integer(rhs) => Ok(*lhs == *rhs as Byte),
                Value::Float(rhs) => Ok(*lhs == *rhs as Byte),
                Value::Byte(rhs) => Ok(*lhs == *rhs),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch),
            },
            Value::Boolean(_) => Err(TypeMismatch),
        }
    }

    pub fn gt(&self, rhs:&Value) -> Result<bool, InterpreterError> {
        match &self {
            Value::Integer(i) => match rhs {
                Value::String(_) => Err(TypeMismatch),
                Value::Integer(rhs) => Ok(*i > *rhs),
                Value::Float(rhs) => Ok(*i as Float > *rhs),
                Value::Byte(rhs) => Ok(*i > *rhs as Integer),

                Value::Boolean(_) => Err(TypeMismatch)
            }
            Value::Float(f) => match rhs {
                Value::String(_) => Err(TypeMismatch),
                Value::Integer(rhs) => Ok(*f > *rhs as Float),
                Value::Float(rhs) => Ok(*f > *rhs),
                Value::Byte(rhs) => Ok(*f > *rhs as Float),
                Value::Boolean(_) => Err(TypeMismatch)
            }
            Value::Byte(lhs) => match rhs {
                Value::Integer(rhs) => Ok(*lhs > *rhs as Byte),
                Value::Float(rhs) => Ok(*lhs > *rhs as Byte),
                Value::Byte(rhs) => Ok(*lhs > *rhs),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch),
            },

            Value::String(_) => Err(TypeMismatch),
            Value::Boolean(_) => Err(TypeMismatch)
        }
    }

    pub fn ge(&self, rhs:&Value) -> Result<bool, InterpreterError> {
        match &self {
            Value::Integer(i) => match rhs {
                Value::String(_) => Err(TypeMismatch),
                Value::Integer(rhs) => Ok(*i >= *rhs),
                Value::Byte(rhs) => Ok(*i >= *rhs as Integer),
                Value::Float(rhs) => Ok(*i as Float >= *rhs),
                Value::Boolean(_) => Err(TypeMismatch)
            }
            Value::Float(f) => match rhs {
                Value::String(_) => Err(TypeMismatch),
                Value::Integer(rhs) => Ok(*f >= *rhs as Float),
                Value::Byte(rhs) => Ok(*f >= *rhs as Float),
                Value::Float(rhs) => Ok(*f >= *rhs),
                Value::Boolean(_) => Err(TypeMismatch)
            }
            Value::Byte(lhs) => match rhs {
                Value::Integer(rhs) => Ok(*lhs >= *rhs as Byte),
                Value::Float(rhs) => Ok(*lhs >= *rhs as Byte),
                Value::Byte(rhs) => Ok(*lhs >= *rhs),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch),
            },

            Value::String(_) => Err(TypeMismatch),
            Value::Boolean(_) => Err(TypeMismatch)
        }
    }

    pub fn lt(&self, rhs:&Value) -> Result<bool, InterpreterError> {
        match &self {
            Value::Integer(i) => match rhs {
                Value::String(_) => Err(TypeMismatch),
                Value::Integer(rhs) => Ok(*i < *rhs),
                Value::Byte(rhs) => Ok(*i < *rhs as Integer),
                Value::Float(rhs) => Ok(*i < *rhs as Integer),
                Value::Boolean(_) => Err(TypeMismatch)
            }
            Value::Float(f) => match rhs {
                Value::String(_) => Err(TypeMismatch),
                Value::Integer(rhs) => Ok(*f < *rhs as Float),
                Value::Byte(rhs) => Ok(*f < *rhs as Float),
                Value::Float(rhs) => Ok(*f < *rhs),
                Value::Boolean(_) => Err(TypeMismatch)
            }
            Value::Byte(lhs) => match rhs {
                Value::Integer(rhs) => Ok(*lhs < *rhs as Byte),
                Value::Float(rhs) => Ok(*lhs < *rhs as Byte),
                Value::Byte(rhs) => Ok(*lhs < *rhs),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch),
            },

            Value::String(_) => Err(TypeMismatch),
            Value::Boolean(_) => Err(TypeMismatch)
        }
    }

    pub fn le(&self, rhs:&Value) -> Result<bool, InterpreterError> {
        match &self {
            Value::Integer(i) => match rhs {
                Value::String(_) => Err(TypeMismatch),
                Value::Integer(rhs) => Ok(*i <= *rhs),
                Value::Byte(rhs) => Ok(*i <= *rhs as Integer),
                Value::Float(rhs) => Ok(*i <= *rhs as Integer),
                Value::Boolean(_) => Err(TypeMismatch)
            }
            Value::Float(f) => match rhs {
                Value::String(_) => Err(TypeMismatch),
                Value::Integer(rhs) => Ok(*f <= *rhs as Float),
                Value::Byte(rhs) => Ok(*f <= *rhs as Float),
                Value::Float(rhs) => Ok(*f <= *rhs),
                Value::Boolean(_) => Err(TypeMismatch)
            }
            Value::Byte(lhs) => match rhs {
                Value::Integer(rhs) => Ok(*lhs <= *rhs as Byte),
                Value::Float(rhs) => Ok(*lhs <= *rhs as Byte),
                Value::Byte(rhs) => Ok(*lhs <= *rhs),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch),
            },

            Value::String(_) => Err(TypeMismatch),
            Value::Boolean(_) => Err(TypeMismatch)
        }
    }

  /*  pub fn add(&self, rhs: &Value) -> Result<Value, InterpreterError> {
        match &self {
            Value::Integer(lhs) => match  rhs{
                Value::Integer(r) => Ok(Value::Integer(*lhs + *r)),
                Value::Float(f) => Ok(Value::Float(*lhs as Float + *f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Float(lhs) => match rhs {
                Value::Integer(i) => Ok(Value::Float(*lhs + *i as Float)),
                Value::Float(f) => Ok(Value::Float(*lhs + *f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Boolean(_) => Err(InterpreterError::OperationUnsupported),
            Value::String(_) => Err(InterpreterError::OperationUnsupported)
        }
    }

    pub fn sub(&self, rhs: &Value) -> Result<Value, InterpreterError> {
        match &self {
            Value::Integer(lhs) => match  rhs{
                Value::Integer(r) => Ok(Value::Integer(*lhs - *r)),
                Value::Float(f) => Ok(Value::Float(*lhs as Float - f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Float(lhs) => match rhs {
                Value::Integer(i) => Ok(Value::Float(lhs - *i as Float)),
                Value::Float(f) => Ok(Value::Float(lhs - f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Boolean(_) => Err(InterpreterError::OperationUnsupported),
            Value::String(_) => Err(InterpreterError::OperationUnsupported)
        }
    }

    pub fn mul(&self, rhs: &Value) -> Result<Value, InterpreterError> {
        match &self {
            Value::Integer(lhs) => match  rhs{
                Value::Integer(r) => Ok(Value::Integer(lhs * r)),
                Value::Float(f) => Ok(Value::Float(*lhs as Float* f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Float(lhs) => match rhs {
                Value::Integer(i) => Ok(Value::Float(*lhs as Float * *i as Float)),
                Value::Float(f) => Ok(Value::Float(lhs * f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Boolean(_) => Err(InterpreterError::OperationUnsupported),
            Value::String(_) => Err(InterpreterError::OperationUnsupported)
        }
    }

    pub fn div(&self, rhs: &Value) -> Result<Value, InterpreterError> {
        match &self {
            Value::Integer(lhs) => match  rhs{
                Value::Integer(r) => Ok(Value::Integer(lhs / r)),
                Value::Float(f) => Ok(Value::Float(*lhs as Float / f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Float(lhs) => match rhs {
                Value::Integer(i) => Ok(Value::Float(lhs / *i as Float)),
                Value::Float(f) => Ok(Value::Float(lhs / f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Boolean(_) => Err(InterpreterError::OperationUnsupported),
            Value::String(_) => Err(InterpreterError::OperationUnsupported)
        }
    }*/
}