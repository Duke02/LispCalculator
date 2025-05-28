use crate::functors::Functor;
use crate::operand::{Operand, OperandType};

pub struct Subtract {}

impl Subtract {
    pub fn new() -> Self {
        Subtract {}
    }
}

impl Functor for Subtract {
    fn get_operand_types(&self) -> Vec<OperandType> {
        vec![OperandType::Float, OperandType::Int]
    }

    fn get_operator(&self) -> String {
        "-".to_string()
    }

    fn get_num_operands(&self) -> u8 {
        2
    }

    fn operate(&self, operands: Vec<Operand>) -> Operand {
        match &operands[..] {
            [base, subtractor] => match base.dtype().resulting_type(&subtractor.dtype()) {
                OperandType::Float => {
                    let Operand::Float(o) = base.resulting_value(&subtractor.dtype()) else { panic!("This should never happen.") };
                    let Operand::Float(t) = subtractor.resulting_value(&base.dtype()) else {panic!("This should never happen.")};
                    Operand::Float(o - t)
                }
                OperandType::Int => {
                    let Operand::Int(o) = base.resulting_value(&subtractor.dtype()) else { panic!("This should never happen.") };
                    let Operand::Int(t) = subtractor.resulting_value(&base.dtype()) else { panic!("This should never happen.") };
                    Operand::Int(o - t)
                }
                _ => panic!("Invalid resulting type."),
            },
            _ => panic!("Validate check failed."),
        }
    }
}