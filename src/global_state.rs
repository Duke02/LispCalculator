use std::collections::HashMap;
use crate::operand::Operand;

pub struct GlobalState {
    variables: HashMap<String, Operand>,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        GlobalState {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, val: Operand) {
        self.variables.insert(name, val);
    }
}