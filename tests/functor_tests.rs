mod tests {
    use lisp_calculator::calculator::Calculator;
    use lisp_calculator::operand::Operand;
    use super::*;
    
    #[test]
    fn test_addition() {
        // Supposed to test just addition
        // base_tests is supposed to test Calculator and stuff 
        // So we can be more lenient here.
        let mut calc = Calculator::new();
        
        let s = "(+ 1 2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(3));
        
        let s = "(+ -1 2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(1));
        
        let s = "(+ 1.5 2.0)";
        assert_eq!(calc.process(s).unwrap(), Operand::Float(3.5));
        
        let s = "(+ 1 2.5)";
        assert_eq!(calc.process(s).unwrap(), Operand::Float(3.5));
        
        let s = "(+ 1.5 2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Float(3.5));
        
        let s = "(+ 1 2 3)";
        assert!(calc.process(s).is_err());
        
        let s = "(+ 1)";
        assert!(calc.process(s).is_err());
    }
    
    #[test]
    fn test_subtraction() {
        let mut calc = Calculator::new();
        
        let s = "(- 1 2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(-1));
        
        let s = "(- 2 1)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(1));
        
        let s = "(- -1 5)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(-6));
        
        let s = "(- 5 -1)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(6));
        
        let s = "(- 2.5 1.0)";
        assert_eq!(calc.process(s).unwrap(), Operand::Float(1.5));
        
        let s = "(- 5 4.5)";
        assert_eq!(calc.process(s).unwrap(), Operand::Float(0.5));
        
        let s = "(- 5.5 5)";
        assert_eq!(calc.process(s).unwrap(), Operand::Float(0.5));
        
        let s = "(- 1)";
        assert!(calc.process(s).is_err());
        
        let s = "(- 1 1 1)";
        assert!(calc.process(s).is_err());
    }
    
    #[test]
    fn test_multiplication() {
        let mut calc = Calculator::new();
        
        let s = "(* 1 2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(2));
        
        let s = "(* 2 1)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(2));
        
        let s = "(* -1 2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(-2));
        
        let s = "(* 1 -2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(-2));
        
        let s = "(* 1)";
        assert!(calc.process(s).is_err());
        
        let s = "(* 1 1 1)";
        assert!(calc.process(s).is_err());
    }
    
    #[test]
    fn test_division() {
        let mut calc = Calculator::new();
        
        let s = "(/ 1 1)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(1));
        
        let s = "(/ 1 2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(0));
        
        let s = "(/ -4 2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(-2));
        
        let s = "(/ 2 -1)";
        assert_eq!(calc.process(s).unwrap(), Operand::Int(-2));
        
        let s = "(/ 3.5 1.4)";
        assert_eq!(calc.process(s).unwrap(), Operand::Float(2.5));
        
        let s = "(/ 2.5 1)";
        assert_eq!(calc.process(s).unwrap(), Operand::Float(2.5));
        
        let s = "(/ 4.9 0.8)";
        assert_eq!(calc.process(s).unwrap(), Operand::Float(6.125));
        
        let s = "(/ 2 0.5 0.5)";
        assert!(calc.process(s).is_err());
        
        let s = "(/ 2)";
        assert!(calc.process(s).is_err());
    }
}