use crate::operand::{Operand, OperandType};
use crate::result::{CalcError, CalcResult};

pub trait Functor {
    fn get_operand_types(&self) -> Vec<OperandType>;
    fn get_operator(&self) -> String;
    fn get_num_operands(&self) -> u8;
    fn operate(&self, operands: Vec<Operand>) -> Operand;
    fn validate_operands(&self, operands: &Vec<Operand>) -> Option<CalcError> {
        let operand_types = self.get_operand_types();
        if operands.len() != self.get_num_operands() as usize {
            return Some(CalcError::IncorrectNumOfOperands {
                required: self.get_num_operands(),
                provided: operands.len() as u8,
            });
        }

        let invalid_operands = operands
            .into_iter()
            .filter(|o| operand_types.iter().any(|ot| OperandType::from((*o).clone()) == *ot))
            .collect::<Vec<_>>();
        if !invalid_operands.is_empty() {
            None
        } else {
            Some(CalcError::InvalidOperandType {
                invalid: OperandType::from(invalid_operands[0].clone()),
                allowed: operand_types.clone(),
            })
        }
    }

    fn perform_operation(&self, operands: Vec<Operand>) -> CalcResult<Operand> {
        if let Some(err) = self.validate_operands(&operands) {
            Err(err)
        } else {
            Ok(self.operate(operands))
        }
    }
}
