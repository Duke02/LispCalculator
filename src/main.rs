use crate::calculator::Calculator;

mod functors;
mod operand;
mod result;
mod global_state;
pub mod calculator;

fn main() {
    let mut calc = Calculator::new();

    let quick_test_strings = vec!["(+ 1 2)", "(+ 1.1 2.1)", "(+ 1.0 2.1)", "(+ true false)"];

    for s in quick_test_strings {
        let out = calc.process(s);
        println!("{:?}", out);
    }
}
