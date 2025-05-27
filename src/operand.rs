use std::fmt::Display;
use std::str::FromStr;
use crate::result::CalcError;

/// The data types of operands.
#[derive(PartialEq, Clone, Debug)]
pub enum OperandType {
    Float,
    Int,
    Bool,
}

impl OperandType {
    /// Determines the correct data type between self and other to up-cast to.
    ///
    /// Up-casting basically means resolving a difference between two data types such that
    /// they are the same. Currently, Float is King, so if you provide a bool/int with a float,
    /// the up-casted data type will be a float. In order of priority, Float is King, Int is second
    /// priority, and Bool is least priority.
    ///
    /// # Arguments
    /// * other: The other data type that we need to compare self to.
    ///
    /// # Returns
    /// The data type to upcast self and other to.
    pub fn resulting_type(&self, other: &OperandType) -> OperandType {
        match self {
            // Float is King
            OperandType::Float => OperandType::Float,
            // Int is 2nd priority
            OperandType::Int => match other {
                OperandType::Float => OperandType::Float,
                _ => OperandType::Int,
            },
            // Bool has the least priority 
            OperandType::Bool => match other {
                OperandType::Bool => OperandType::Bool,
                OperandType::Int => OperandType::Int,
                _ => OperandType::Float,
            }
        }
    }
}

/// The actual operands of a statement that we need to evaluate.
#[derive(PartialEq, Clone, Debug)]
pub enum Operand {
    Float(f64),
    Int(i64),
    Bool(bool),
}

impl Operand {
    pub fn new_from_float(value: f64) -> Operand {
        Operand::Float(value)
    }
    
    pub fn new_from_int(value: i64) -> Operand {
        Operand::Int(value)
    }
    
    pub fn new_from_bool(bool: bool) -> Operand {
        Operand::Bool(bool)
    }

    /// Gets the data type of the operand.
    pub fn dtype(&self) -> OperandType {
        match self {
            Operand::Float(_) => OperandType::Float,
            Operand::Int(_) => OperandType::Int,
            Operand::Bool(_) => OperandType::Bool,
        }
    }

    /// Up-casts self to the correct OperandType based on self.dtype() and other.
    ///
    /// Up-casting basically means resolving a difference between two data types such that
    /// they are the same. Currently, Float is King, so if you provide a bool/int with a float,
    /// the up-casted data type will be a float. In order of priority, Float is King, Int is second
    /// priority, and Bool has the least priority.
    ///
    /// # Arguments
    /// * other: The data type of the other operand.
    ///
    /// # Returns
    /// Self but up-casted to the correct data type based on other.
    pub fn resulting_value(&self, other: &OperandType) -> Operand {
        match self.dtype().resulting_type(other) {
            OperandType::Float => match self {
                Operand::Float(f) => Operand::new_from_float(*f),
                Operand::Int(i) => Operand::new_from_float(*i as f64),
                Operand::Bool(b) => Operand::new_from_float(if *b {1.0} else {0.0}),
            },
            OperandType::Int => match self {
                Operand::Int(i) => Operand::new_from_int(*i),
                Operand::Bool(b) => Operand::new_from_int(if *b {1} else {0}),
                Operand::Float(f) => Operand::new_from_int(*f as i64),
            },
            OperandType::Bool => match self {
                Operand::Bool(b) => Operand::new_from_bool(*b),
                Operand::Int(i) => Operand::new_from_bool(*i > 0),
                Operand::Float(f) => Operand::new_from_bool(*f > 0.0),
            }
        }
    }
}

impl Display for Operand {
    fn fmt(&self, f1: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Operand::Float(f) => f.to_string(),
            Operand::Int(i) => i.to_string(),
            Operand::Bool(b) => b.to_string(),
        };
        write!(f1, "{}", str)
    }
}

impl From<Operand> for OperandType {
    fn from(o: Operand) -> Self {
        match o {
            Operand::Float(_) => OperandType::Float,
            Operand::Int(_) => OperandType::Int,
            Operand::Bool(_) => OperandType::Bool,
        }
    }
}

impl TryFrom<String> for Operand {
    type Error = CalcError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Operand::try_from(value.as_str())
    }
}

impl TryFrom<&str> for Operand {
    type Error = CalcError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let v = value.trim();
        if let Ok(i) = i64::from_str(v) {
            Ok(Operand::Int(i))
        } else if let Ok(f) = f64::from_str(v) {
            Ok(Operand::Float(f))
        } else if let Ok(b) = bool::from_str(v) {
            Ok(Operand::Bool(b))
        } else {
            Err(CalcError::ParseError("Invalid operand".to_string()))
        }
    }
}
