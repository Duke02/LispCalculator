use crate::global_state::GlobalState;
use crate::operand::{Operand, OperandType};
use crate::result::{CalcError, CalcResult};

pub trait StateChanger {
    fn get_keyword(&self) -> String;
    fn get_num_args(&self) -> u8;
    fn get_operand_types(&self) -> Vec<OperandType>;
    fn operation(&self, orig: &GlobalState, operands: &Vec<Operand>) -> GlobalState;
    
    fn can_change_state(&self, statement: &str) -> bool {
        statement[1..].split(" ").collect::<Vec<_>>()[0] == self.get_keyword()
    }
    
    fn validate_operands(&self, operands: &Vec<Operand>) -> CalcResult<()> {
        // Check number of operands.
        if operands.len() != self.get_num_args() as usize {
            return Err(CalcError::IncorrectNumOfOperands {
                required: self.get_num_args(),
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
    
    fn change_state(&self, orig: &GlobalState, operands: &Vec<Operand>) -> CalcResult<GlobalState> {
        if let Err(e) = self.validate_operands(operands) {
            Err(e)
        } else {
            Ok(self.operation(orig, operands))
        }
    }    
}