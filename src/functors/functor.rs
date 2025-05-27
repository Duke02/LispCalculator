use crate::operand::{Operand, OperandType};
use crate::result::{CalcError, CalcResult};

pub trait Functor {
    fn get_operand_types(&self) -> Vec<OperandType>;
    fn get_operator(&self) -> String;
    fn get_num_operands(&self) -> u8;
    fn operate(&self, operands: Vec<Operand>) -> Operand;
    fn validate_operands(&self, operands: &Vec<Operand>) -> CalcResult<()> {
        // Check number of operands.
        if operands.len() != self.get_num_operands() as usize {
            return Err(CalcError::IncorrectNumOfOperands {
                required: self.get_num_operands(),
                provided: operands.len() as u8,
            });
        }

        // Make sure they're all the correct type. 
        let operand_types = self.get_operand_types();

        let invalid_operands = operands
            .into_iter()
            .filter(|o| !operand_types.iter().any(|ot| OperandType::from((*o).clone()) == *ot))
            .collect::<Vec<_>>();
        
        // If we had no invalid operand types, we're good.
        if invalid_operands.is_empty() {
            Ok(())
        } else {
            // Otherwise we're not. 
            Err(CalcError::InvalidOperandType {
                invalid: OperandType::from(invalid_operands[0].clone()),
                allowed: operand_types.clone(),
            })
        }
    }

    fn perform_operation(&self, operands: Vec<Operand>) -> CalcResult<Operand> {
        if let Err(err) = self.validate_operands(&operands) {
            Err(err)
        } else {
            Ok(self.operate(operands))
        }
    }
    
    fn can_operate(&self, statement: &str) -> bool {
        // Make sure first word in statement is the operator.
        // Assumes first character is ( and is unneeded.
        statement[1..].split(" ").collect::<Vec<_>>()[0] == self.get_operator()
    }
}
