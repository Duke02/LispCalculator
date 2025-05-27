use crate::functors::add::Add;
use crate::functors::divide::Divide;
use crate::functors::multiply::Multiply;
use crate::functors::subtract::Subtract;
use crate::functors::Functor;
use crate::global_state::GlobalState;
use crate::operand::Operand;
use crate::result::{CalcError, CalcResult};

/// The actual reason for this repo's existence - the Calculator.
pub struct Calculator {
    global_state: GlobalState,
    /// The supported operators for the Calculator
    operators: Vec<Box<dyn Functor>>,
}

impl Calculator {
    /// Creates a new Calculator.
    pub fn new() -> Self {
        Self {
            global_state: GlobalState::new(),
            operators: vec![
                Box::new(Add::new()),
                Box::new(Subtract::new()),
                Box::new(Multiply::new()),
                Box::new(Divide::new()),
            ],
        }
    }

    /// Parses the provided string to find the innermost statement and its location.
    ///
    /// # Arguments
    /// * s: The statement to parse.
    ///
    /// # Returns
    /// Either:
    ///    - Some((statement, start index, end index)) if a valid statement was found.
    /// or
    ///    - None if there's uneven number of parenthesis in the provided statement
    /// The resulting statement on a valid parse will contain the parenthesis of the original statement.
    pub fn get_innermost_statement(&self, s: &str) -> Option<(String, usize, usize)> {
        let mut stack: Vec<usize> = Vec::new();
        let mut innermost: Vec<(u32, usize, usize)> = vec![];
        let mut counter: u32 = 0;

        for (i, c) in s.char_indices() {
            match c {
                '(' => {
                    stack.push(i);
                    counter += 1
                }
                ')' => {
                    if let Some(start) = stack.pop() {
                        innermost.push((counter, start, i));
                        if counter == 0 {
                            // If we have an unbalanced set of parentheses, then we've lost the game chief.
                            return None;
                        }
                        counter -= 1;
                    }
                }
                _ => {}
            }
        }

        if innermost.is_empty() {
            None
        } else {
            let &(_depth, start, end) = innermost
                .iter()
                .max_by_key(|(c, _s, _e)| *c)
                .expect("Check above did not work.");
            let innermost_statement = s[start..(end + 1)].to_string();
            Some((innermost_statement, start, end + 1))
        }
    }

    /// Gets the operator to use for the non-nested statement.
    ///
    /// # Returns
    /// None if there's no operator appropriate for the statement. Some(...) if the Calculator supports the statement's operation.
    pub fn get_functor(&self, statement: &str) -> Option<&Box<dyn Functor>> {
        for op in &self.operators {
            if op.can_operate(statement) {
                return Some(op);
            }
        }
        None
    }

    /// Parses the operands from the non-nested statement.
    pub fn parse_operands(&self, statement: &str) -> Vec<Operand> {
        statement[1..statement.len() - 1]
            .split(" ")
            .filter_map(|po| Operand::try_from(po).ok())
            .collect::<Vec<_>>()
    }

    /// Processes the full, possibly nested input string and gets the final result of its operations.
    ///
    /// # Arguments
    /// * in_str: The statement that contains operations to do. Can be nested to the wazzoo if you want to do multiple operations.
    ///
    /// # Returns
    /// Gives you an Err if you:
    /// - gave us a bad string with unbalanced parenthesis
    /// - requested we perform an operation on an unsupported data type
    /// - gave us a non-parsable operand
    /// - other reasons, we should tell you the reason with the internal error.
    /// Gives you an Ok(...) with the final computed value if we didn't have any issues with your input.
    pub fn process(&mut self, in_str: &str) -> CalcResult<Operand> {
        let mut s = in_str.to_string();
        while let Some((statement, start, end)) = self.get_innermost_statement(s.as_str()) {
            if let Some(functor) = self.get_functor(statement.as_str()) {
                let operands = self.parse_operands(statement.as_str());
                match functor.perform_operation(operands) {
                    Ok(result) => {
                        s = s.replace(statement.as_str(), result.to_string().as_str());
                    }
                    Err(e) => return Err(e),
                }
            } else {
                return Err(CalcError::InvalidOperator {
                    provided: statement,
                });
            }
        }
        Ok(Operand::try_from(s.as_str())?)
    }
}
