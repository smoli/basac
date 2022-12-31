use crate::error::InterpreterError;
use crate::error::InterpreterError::TypeMismatch;

#[derive(Debug,Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool)
}

impl Value {

    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::String(s) => Some(s.parse::<i64>().unwrap()),
            Value::Integer(i) => Some(*i),
            Value::Float(f) => Some(*f as i64),
            Value::Boolean(_) => None
        }
    }

    pub fn add(&self, rhs: &Value) -> Result<Value, InterpreterError> {
        match &self {
            Value::Integer(lhs) => match  rhs{
                Value::Integer(r) => Ok(Value::Integer(*lhs + *r)),
                Value::Float(f) => Ok(Value::Float(*lhs as f64 + *f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Float(lhs) => match rhs {
                Value::Integer(i) => Ok(Value::Float(*lhs + *i as f64)),
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
                Value::Float(f) => Ok(Value::Float(*lhs as f64 - f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Float(lhs) => match rhs {
                Value::Integer(i) => Ok(Value::Float(lhs - *i as f64)),
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
                Value::Float(f) => Ok(Value::Float(*lhs as f64* f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Float(lhs) => match rhs {
                Value::Integer(i) => Ok(Value::Float(*lhs as f64 * *i as f64)),
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
                Value::Float(f) => Ok(Value::Float(*lhs as f64 / f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Float(lhs) => match rhs {
                Value::Integer(i) => Ok(Value::Float(lhs / *i as f64)),
                Value::Float(f) => Ok(Value::Float(lhs / f)),
                Value::Boolean(_) => Err(TypeMismatch),
                Value::String(_) => Err(TypeMismatch)
            }
            Value::Boolean(_) => Err(InterpreterError::OperationUnsupported),
            Value::String(_) => Err(InterpreterError::OperationUnsupported)
        }
    }
}