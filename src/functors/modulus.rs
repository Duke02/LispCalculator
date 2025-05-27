use crate::functors::Functor;
use crate::operand::{Operand, OperandType};

pub struct Modulus {}

impl Modulus {
    pub fn new() -> Modulus {
        Modulus {}
    }
}

impl Functor for Modulus {
    fn get_operand_types(&self) -> Vec<OperandType> {
        vec![OperandType::Int]
    }

    fn get_operator(&self) -> String {
        "%".to_string()
    }

    fn get_num_operands(&self) -> u8 {
        2
    }

    fn operate(&self, operands: Vec<Operand>) -> Operand {
        match &operands[..] {
            [dividend, divisor] => match dividend.dtype().resulting_type(&divisor.dtype()) {
                OperandType::Int => {
                    let Operand::Int(o) = dividend.resulting_value(&divisor.dtype()) else { panic!("This should never happen.") };
                    let Operand::Int(t) = divisor.resulting_value(&dividend.dtype()) else { panic!("This should never happen.") };
                    Operand::Int(o % t)
                }
                _ => panic!("Invalid resulting type."),
            },
            _ => panic!("Validate check failed."),
        }
    }
}