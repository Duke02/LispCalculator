use crate::calculator::Calculator;

mod functors;
mod operand;
mod result;
mod global_state;
pub mod calculator;

fn main() {
    let mut calc = Calculator::new();
    
    let three = calc.process("(+ 1 2)");

    println!("Calculator computed {three:?}");
}
