mod tests {
    use lisp_calculator::calculator::Calculator;

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
}
