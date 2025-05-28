use crate::functors::Functor;
use crate::operand::{Operand, OperandType};

/// Allows for multiplication for the calculator.
pub struct Multiply {}

impl Multiply {
    pub fn new() -> Self {
        Self {}
    }
}

impl Functor for Multiply {
    fn get_operand_types(&self) -> Vec<OperandType> {
        vec![OperandType::Int, OperandType::Float]
    }

    fn get_operator(&self) -> String {
        "*".to_string()
    }

    fn get_num_operands(&self) -> u8 {
        2
    }

    fn operate(&self, operands: Vec<Operand>) -> Operand {
        match &operands[..] {
            [one, two] => match one.dtype().resulting_type(&two.dtype()) {
                OperandType::Float => {
                    let Operand::Float(o) = one.resulting_value(&two.dtype()) else { panic!("This should never happen.") };
                    let Operand::Float(t) = two.resulting_value(&one.dtype()) else {panic!("This should never happen.")};
                    Operand::Float(o * t)
                }
                OperandType::Int => {
                    let Operand::Int(o) = one.resulting_value(&two.dtype()) else { panic!("This should never happen.") };
                    let Operand::Int(t) = two.resulting_value(&one.dtype()) else { panic!("This should never happen.") };
                    Operand::Int(o * t)
                }
                _ => panic!("Invalid resulting type."),
            },
            _ => panic!("Validate check failed."),
        }
    }
}