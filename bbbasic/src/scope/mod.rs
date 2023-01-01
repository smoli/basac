use std::collections::HashMap;
use crate::error::InterpreterError;
use crate::value::Value;

#[derive(Debug)]
pub struct Scope {
    values: HashMap<String, Value>
}

impl Scope {

    pub fn new() ->Scope {
        Scope { values: HashMap::new() }
    }

    pub fn set(&mut self, name: &String, value: Value) {
        self.values.insert(name.clone(), value);
    }

    pub fn get(&self, name: &String) -> Result<&Value, InterpreterError> {
        match self.values.get(name) {
            None => Err(InterpreterError::UnknownVariable),
            Some(v) => Ok(v)
        }
    }
}
