use crate::operand::OperandType;

pub enum CalcError {
    InvalidOperandType {
        invalid: OperandType,
        allowed: Vec<OperandType>,
    },
    IncorrectNumOfOperands {provided: u8, required: u8},
}

pub type CalcResult<T> = Result<T, CalcError>;