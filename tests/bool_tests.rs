mod tests {
    use lisp_calculator::calculator::Calculator;
    use lisp_calculator::operand::Operand;
    use super::*;
    
    #[test]
    fn test_and() {
        let mut calc = Calculator::new();
        
        let s = "(&& true true)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(true));
        
        let s = "(&& false false)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(false));
        
        let s = "(&& true false)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(false));
        
        let s = "(&& false true)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(false));
        
        let s = "(&& 1 1)";
        assert!(calc.process(s).is_err());
        
        let s = "(&& true)";
        assert!(calc.process(s).is_err());
        
        let s = "(&& true true true)";
        assert!(calc.process(s).is_err());
    }
}