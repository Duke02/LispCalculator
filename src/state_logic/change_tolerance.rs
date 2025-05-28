use crate::global_state::GlobalState;
use crate::operand::{Operand, OperandType};
use crate::result::CalcResult;
use crate::state_logic::state_changer::StateChanger;

pub struct ChangeTolerance {}

impl ChangeTolerance {
    pub fn new() -> ChangeTolerance {
        ChangeTolerance {}
    }
}

impl StateChanger for ChangeTolerance {
    fn get_keyword(&self) -> String {
        "ch-tol".to_string()
    }

    fn get_num_args(&self) -> u8 {
        1
    }

    fn get_operand_types(&self) -> Vec<OperandType> {
        vec![OperandType::Float]
    }

    fn operation(&self, orig: &GlobalState, operands: &Vec<Operand>) -> GlobalState {
        let new_tolerance_op = operands.get(0).expect("Our validation didn't work.");
        match new_tolerance_op {
            Operand::Float(f) => {
                let mut out_state = orig.clone();
                out_state.update_tolerance(*f);
                out_state
            }
            _ => panic!("Cannot set tolerance with a non-floating point tolerance. Otherwise everything will be horrible.")
        }
        
    }
}