use std::collections::HashMap;
use std::str::FromStr;
use crate::error::InterpreterError;
use crate::parser::{NumericVariable, NumericVariable_type_dem};
use crate::value::Value;

pub type Float = f64;
pub type Integer = i64;
pub type Byte = u8;


pub trait One {
    fn one() -> Self;
}

impl One for Float {
    fn one() -> Self {
        1.0
    }
}

impl One for Integer {
    fn one() -> Self {
        1
    }
}

impl One for Byte {
    fn one() -> Self {
        1
    }
}

pub trait DataTypeQuery {
    fn get_type() -> DataType;
}


impl DataTypeQuery for Float {
    fn get_type() -> DataType {
        DataType::Float
    }
}

impl DataTypeQuery for Byte {
    fn get_type() -> DataType {
        DataType::Byte
    }
}

impl DataTypeQuery for Integer {
    fn get_type() -> DataType {
        DataType::Integer
    }
}

impl DataTypeQuery for String {
    fn get_type() -> DataType {
        DataType::String
    }
}


pub trait ScopeValue : std::ops::Add + std::ops::Sub + std::ops::Div + std::ops::Mul + One + FromStr + Copy + DataTypeQuery {}
impl <T: std::ops::Add<Output = T> + std::ops::Sub + std::ops::Div + std::ops::Div<Output=T> + std::ops::Mul + One + FromStr + Copy + DataTypeQuery>  ScopeValue for T {}


pub enum DataType {
    Byte = 1,
    Integer = 2,
    Float = 3,
    String = 99
}


#[derive(Debug)]
pub struct Scope {
    floats: HashMap<String, Float>,
    ints: HashMap<String, Integer>,
    bytes: HashMap<String, Byte>,
    strings: HashMap<String, String>
}

impl Scope {

    pub fn new() ->Scope {
        Scope {
            floats: HashMap::new(),
            ints: HashMap::new(),
            bytes: HashMap::new(),
            strings: HashMap::new()
        }
    }
/*
    pub fn set<T: DataTypeQuery>(&mut self, name: &String, value: T) {
        match T::get_type() {
            DataType::Float => self.set_float(name, value as Float),
            _ => {}
        }
    }

    pub fn get<T: DataTypeQuery>(&mut self, name: &String) -> Result<T, InterpreterError> {
        match T::get_type() {
            DataType::Float => self.get_float(name),
            _ => {}
        }
    }
*/

    pub fn get(&self, variable: &NumericVariable) -> Result<Value, InterpreterError> {
        match &variable.type_dem {
            None => Ok(Value::Float(self.get_float(&variable.name)?)),
            Some(t) => match t {
                NumericVariable_type_dem::ByteDenominator(_) => Ok(Value::Byte(self.get_byte(&variable.name)?)),
                NumericVariable_type_dem::FloatDenominator(_) => Ok(Value::Float(self.get_float(&variable.name)?)),
                NumericVariable_type_dem::IntegerDenominator(_) => Ok(Value::Integer(self.get_int(&variable.name)?)),
            }
        }
    }

    pub fn set_float(&mut self, name: &String, value: Float) {
        self.floats.insert(name.clone(), value);
    }

    pub fn set_int(&mut self, name: &String, value: Integer) {
        self.ints.insert(name.clone(), value);
    }

    pub fn set_byte(&mut self, name: &String, value: Byte) {
        self.bytes.insert(name.clone(), value);
    }

    pub fn set_string(&mut self, name: &String, value: String) {
        self.strings.insert(name.clone(), value);
    }

    pub fn get_float(&self, name: &String) -> Result<Float, InterpreterError> {
        match self.floats.get(name) {
            None => Err(InterpreterError::UnknownVariable(name.clone())),
            Some(v) => Ok(*v)
        }
    }

    pub fn get_int(&self, name: &String) -> Result<Integer, InterpreterError> {
        match self.ints.get(name) {
            None => Err(InterpreterError::UnknownVariable(name.clone())),
            Some(v) => Ok(*v)
        }
    }

    pub fn get_byte(&self, name: &String) -> Result<Byte, InterpreterError> {
        match self.bytes.get(name) {
            None => Err(InterpreterError::UnknownVariable(name.clone())),
            Some(v) => Ok(*v)
        }
    }

    pub fn get_string(&self, name: &String) -> Result<&String, InterpreterError> {
        match self.strings.get(name) {
            None => Err(InterpreterError::UnknownVariable(name.clone())),
            Some(v) => Ok(v)
        }
    }

    #[allow(dead_code)]
    pub fn is(&self, name: &String) -> Result<DataType, InterpreterError> {
        if self.floats.contains_key(name) {
            return Ok(DataType::Float)
        }

        if self.ints.contains_key(name) {
            return Ok(DataType::Integer)
        }

        if self.bytes.contains_key(name) {
            return Ok(DataType::Byte)
        }

        if self.strings.contains_key(name) {
            return Ok(DataType::String)
        }

        Err(InterpreterError::UnknownVariable(name.clone()))
    }
}
