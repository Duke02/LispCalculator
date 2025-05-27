use std::collections::HashMap;
use crate::operand::Operand;

/// The global state of the calculator.
/// 
/// Helps us store variables and what symbol the user is having us represent its value. 
#[derive(Debug, Clone)]
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