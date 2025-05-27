mod tests {
    use lisp_calculator::calculator::Calculator;
    use lisp_calculator::functors::add::Add;
    use lisp_calculator::operand::Operand;

    #[test]
    fn innermost_parentheses() {
        let calc = Calculator::new();
        let s = "";
        assert_eq!(calc.get_innermost_statement(s), None);

        let s = "(+ 1 2)";
        assert_eq!(
            calc.get_innermost_statement(s),
            Some((s.to_string(), 0, s.len()))
        );

        let s = "(+ 1 (+ 2 3))";
        assert_eq!(
            calc.get_innermost_statement(s),
            Some(("(+ 2 3)".to_string(), 5, s.len() - 1))
        );
        
        // (- 1 2) => 3 -> 9
        // (+ 2 1) => 14 -> 20
        let s = "(+ (- 1 2) (* (+ 2 1) 3))";
        assert_eq!(
            calc.get_innermost_statement(s),
            Some(("(+ 2 1)".to_string(), 14, 21))
        );
    }

    #[test]
    fn get_functor() {
        let calc = Calculator::new();
        
        let s = "(+ 1 2)";
        assert_eq!(calc.get_functor(s).unwrap().get_operator(), "+".to_string());
        
        // TODO: Change this when we've added subtraction.
        let s = "(- 1 2)";
        assert!(calc.get_functor(s).is_none());
        
        let s = "(max 1 2)";
        assert!(calc.get_functor(s).is_none());
    }
    
    #[test]
    fn get_operands() {
        let calc = Calculator::new();
        
        let s = "(+ 1 2)";
        assert_eq!(calc.parse_operands(s), vec![Operand::Int(1), Operand::Int(2)]);
        
        let s = "(- 1 2 3)";
        assert_eq!(calc.parse_operands(s), vec![Operand::Int(1), Operand::Int(2), Operand::Int(3)]);
        
        let s = "(+ 1.0 2.0)";
        assert_eq!(calc.parse_operands(s), vec![Operand::Float(1.0), Operand::Float(2.0)]);
        
        let s = "(any true)";
        assert_eq!(calc.parse_operands(s), vec![Operand::Bool(true)]);
        
        let s = "(any one out there)";
        assert_eq!(calc.parse_operands(s), vec![]);
    }
    
    #[test]
    fn process() {
        let mut calc = Calculator::new();
        
        let s = "(+ 1 2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(3));
        
        let s = "(+ 1 (+ 2 3))";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(6));
        
        let s = "(+ (+ 2 3) (+ 23 4))";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(32));
        
        // Error due to non-closed parentheses 
        let s = "(+ 1 2";
        assert!(calc.process(s).is_err());
        
        // Invalid num operands
        let s = "(+ 1 (+ 2 3 4))";
        assert!(calc.process(s).is_err());
        
        // Unsupported operation
        let s = "(- 1 (+ 2 3))";
        assert!(calc.process(s).is_err());
        
        // Invalid num operands
        let s = "(+ (+ 1 2))";
        assert!(calc.process(s).is_err());
    }
}
