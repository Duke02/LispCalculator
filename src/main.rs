use crate::calculator::Calculator;

pub mod calculator;
mod functors;
mod global_state;
mod operand;
mod result;

fn main() {
    let mut calc = Calculator::new();

    let quick_test_strings = vec![
        "(+ 1 2)",
        "(+ 1.1 2.1)",
        "(+ 1.0 2.1)",
        "(+ true false)",
        "(- 2 1)",
        "(* 1 2)",
        "(* 1.1 2)",
        "(/ 1 1)",
        "(/ 1 2)",
    ];

    for s in quick_test_strings {
        let out = calc.process(s);
        println!("{:?}", out);
    }
}
