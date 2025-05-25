
#[derive(PartialEq, Clone, Debug)]
pub enum OperandType {
    Float,
    Int,
    Bool,
}

impl OperandType {
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
                _ => OperandType::Float,
            }
        }
    }
}


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
    
    pub fn dtype(&self) -> OperandType {
        match self {
            Operand::Float(_) => OperandType::Float,
            Operand::Int(_) => OperandType::Int,
            Operand::Bool(_) => OperandType::Bool,
        }
    }
    
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

impl From<Operand> for OperandType {
    fn from(o: Operand) -> Self {
        match o {
            Operand::Float(_) => OperandType::Float,
            Operand::Int(_) => OperandType::Int,
            Operand::Bool(_) => OperandType::Bool,
        }
    }
}
