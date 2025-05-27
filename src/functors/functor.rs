use crate::operand::{Operand, OperandType};
use crate::result::{CalcError, CalcResult};

pub trait Functor {
    /// Gets the supported operand types this Functor can operate on.
    ///
    /// # Returns
    /// A vector of supported Operand Types.
    fn get_operand_types(&self) -> Vec<OperandType>;
    /// Gets the operator as it appears in the statement.
    fn get_operator(&self) -> String;
    /// Gets the number of operands that the Functor supports.
    fn get_num_operands(&self) -> u8;
    /// Operates on the validated operands
    ///
    /// # Arguments
    /// * operands: A vector containing operands that have been properly validated to be the correct type and length.
    ///     - Note: Operands haven't been upcasted.
    ///
    /// # Returns
    /// The resulting Operand value from the operation.
    fn operate(&self, operands: Vec<Operand>) -> Operand;

    /// Performs validation on the operands before performing an operation on them.
    /// You generally don't need to the implement this unless you're doing something funny.
    ///
    /// Checks operand length and makes sure all operands are of the supported type.
    ///
    /// # Arguments
    /// * operands: The operands that need to be validated.
    ///
    /// # Returns
    /// A result of whether the operands are appropriate if they're valid. If not, provides an Error and the reason why they're not valid.
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
    
    /// One stop shop for performing the operation. This is what you should use by default.
    /// 
    /// Validates the operands then performs the functor's operation on them.
    /// 
    /// # Arguments
    /// * operands: The operands to validate then perform the operation on.
    /// 
    /// # Returns
    /// A result where if Ok(...) should be the resulting output from the operation. If Err(...), they must have failed in validation King.
    fn perform_operation(&self, operands: Vec<Operand>) -> CalcResult<Operand> {
        if let Err(err) = self.validate_operands(&operands) {
            Err(err)
        } else {
            Ok(self.operate(operands))
        }
    }
    
    /// Makes sure the Functor can operate on the provided statement.
    /// 
    /// # Arguments
    /// * statement: A full statement from the user, including operands and parenthesis.
    /// 
    /// # Returns
    /// True if the first word is the Functor's operator symbol/word. False otherwise. 
    fn can_operate(&self, statement: &str) -> bool {
        // Make sure first word in statement is the operator.
        // Assumes first character is ( and is unneeded.
        statement[1..].split(" ").collect::<Vec<_>>()[0] == self.get_operator()
    }
}
