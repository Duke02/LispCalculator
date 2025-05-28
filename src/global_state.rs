use std::collections::HashMap;
use crate::operand::Operand;

/// The global state of the calculator.
/// 
/// Helps us store variables and what symbol the user is having us represent its value. 
#[derive(Debug, Clone)]
pub struct GlobalState {
    variables: HashMap<String, Operand>,
    tolerance: f64,
}

impl GlobalState {
    pub fn new() -> GlobalState {
        GlobalState {
            variables: HashMap::new(),
            tolerance: 1e-12,
        }
    }

    pub fn set(&mut self, name: String, val: Operand) {
        self.variables.insert(name, val);
    }
    
    pub fn update_tolerance(&mut self, tolerance: f64) {
        self.tolerance = tolerance;
    }
}