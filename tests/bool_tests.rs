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
    
    fn test_or() {
        let mut calc = Calculator::new();
        
        let s = "(|| true true)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(true));
        
        let s = "(|| false false)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(false));
        
        let s = "(|| true false)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(true));
        
        let s = "(|| false true)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(true));
        
        let s = "(|| false)";
        assert!(calc.process(s).is_err());
        
        let s = "(|| false false true)";
        assert!(calc.process(s).is_err());
        
        let s = "(|| 1 1)";
        assert!(calc.process(s).is_err());
        
        let s = "(|| true 1.9)";
        assert!(calc.process(s).is_err());
    }
    
    #[test]
    fn test_gt() {
        let mut calc = Calculator::new();
        
        let s = "(> 2 1)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(true));
        
        let s = "(> 2 3)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(false));
        
        let s = "(> -2 -1)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(false));
        
        let s = "(> 1 -1)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(true));
        
        let s = "(> true false)";
        assert!(calc.process(s).is_err());
        
        let s = "(> 23.5 1.2)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(true));
        
        let s = "(> 1.2 23.5)";
        assert_eq!(calc.process(s).unwrap(), Operand::Bool(false));
    }
}