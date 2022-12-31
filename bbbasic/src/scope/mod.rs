use std::collections::HashMap;
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

    pub fn get(&self, name: &String) -> Option<&Value> {
        self.values.get(name)
    }
}
