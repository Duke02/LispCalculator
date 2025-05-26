use std::fmt::{Display, Formatter};
use crate::operand::OperandType;

#[derive(Debug)]
pub enum CalcError {
    InvalidOperandType {
        invalid: OperandType,
        allowed: Vec<OperandType>,
    },
    IncorrectNumOfOperands {provided: u8, required: u8},
    ParseError(String),
}

impl Display for CalcError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

pub type CalcResult<T> = Result<T, CalcError>;