use crate::functors::Functor;
use crate::operand::{Operand, OperandType};

/// Allows for the Calculator to divide stuff.
/// 
/// NOTE: If you supply Integer Operands, it'll do integer division.
pub struct Divide {}

impl Divide {
    pub fn new() -> Self {
        Self {}
    }
}

impl Functor for Divide {
    fn get_operand_types(&self) -> Vec<OperandType> {
        vec![OperandType::Int, OperandType::Float]
    }

    fn get_operator(&self) -> String {
        "/".to_string()
    }

    fn get_num_operands(&self) -> u8 {
        2
    }

    fn operate(&self, operands: Vec<Operand>) -> Operand {
        match &operands[..] {
            [dividend, divisor] => match dividend.dtype().resulting_type(&divisor.dtype()) {
                OperandType::Float => {
                    let Operand::Float(o) = dividend.resulting_value(&divisor.dtype()) else { panic!("This should never happen.") };
                    let Operand::Float(t) = divisor.resulting_value(&dividend.dtype()) else {panic!("This should never happen.")};
                    Operand::Float(o / t)
                }
                OperandType::Int => {
                    let Operand::Int(o) = dividend.resulting_value(&divisor.dtype()) else { panic!("This should never happen.") };
                    let Operand::Int(t) = divisor.resulting_value(&dividend.dtype()) else { panic!("This should never happen.") };
                    Operand::Int(o / t)
                }
                _ => panic!("Invalid resulting type."),
            },
            _ => panic!("Validate check failed."),
        }
    }
}