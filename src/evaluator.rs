// src/evaluator.rs
use crate::{Expr, MathError, Operator, Result};

pub struct Evaluator;

impl Evaluator {
    // Evaluates an expression tree to produce final result
    pub fn evaluate(expr: &Expr) -> Result<f64> {
        match expr {
            // Return the literal value
            Expr::Literal(value) => Ok(*value),

            // Evaluate the base value multiplied by 10 raised to the power of the exponent
            Expr::Scientific { base, exponent } => Ok(base * (10f64.powi(*exponent))),

            // Evaluate the expression inside the parentheses and return the result
            // Expr::Parenthesized(expr) => Self::evaluate(expr),
            Expr::UnaryMinus(expr) => {
                let value = Self::evaluate(expr)?;
                Ok(-value)
            }

            // Evaluate the left and right expressions and apply the operator
            Expr::BinOp { op, lhs, rhs } => {
                let left = Self::evaluate(lhs)?;
                let right = Self::evaluate(rhs)?;

                match op {
                    // Apply the operator to the left and right values
                    Operator::Add => Ok(left + right),
                    Operator::Subtract => Ok(left - right),
                    Operator::Multiply => Ok(left * right),
                    Operator::Divide => {
                        if right == 0.0 {
                            Err(MathError::DivisionByZero)
                        } else {
                            Ok(left / right)
                        }
                    }
                    Operator::Power => Ok(left.powf(right)), // Raise left to the power of right
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import names from the parent module
    use crate::{Parser, Tokenizer};

    fn eval_str(input: &str) -> Result<f64> {
        let tokens = Tokenizer::tokenize(input)?;
        let mut parser = Parser::new(tokens);
        let expr = parser.parse()?;
        Evaluator::evaluate(&expr)
    }

    // Helper function to compare floating point numbers
    fn assert_float_eq(a: f64, b: f64) {
        let epsilon = 1e-10; // Adjust this value based on required precision
        assert!(
            (a - b).abs() < epsilon,
            "Values not equal within epsilon: {} != {}",
            a,
            b
        );
    }

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(eval_str("1 + 2").unwrap(), 3.0);
        assert_eq!(eval_str("3 - 2").unwrap(), 1.0);
        assert_eq!(eval_str("2 * 3").unwrap(), 6.0);
        assert_eq!(eval_str("6 / 3").unwrap(), 2.0);
        assert_eq!(eval_str("2 ^ 3").unwrap(), 8.0);
    }

    #[test]
    fn test_operator_precedence() {
        // USING BODMAS RULE
        assert_eq!(eval_str("1 + 2 * 3").unwrap(), 7.0);
        assert_eq!(eval_str("(1 * 2) + 3").unwrap(), 5.0);
        assert_eq!(eval_str("2 * 3 + 4").unwrap(), 10.0);
        assert_eq!(eval_str("1 ^ 2 + 3").unwrap(), 4.0);
    }

    #[test]
    fn test_scientific_notation() {
        assert_eq!(eval_str("1.5e3").unwrap(), 1500.0);
        assert_eq!(eval_str("2e-1").unwrap(), 0.2);
        assert_eq!(eval_str("3.5e2 + 2.5e1").unwrap(), 350.0 + 25.0);
    }

    #[test]
    fn test_unary_minus() {
        assert_eq!(eval_str("-5").unwrap(), -5.0);
        assert_eq!(eval_str("-(2 + 3)").unwrap(), -5.0);
        assert_eq!(eval_str("2 + -3").unwrap(), -1.0);
        assert_eq!(eval_str("-2 ^ 3").unwrap(), -8.0);
    }

    #[test]
    fn test_complex_expressions() {
        // Calculate the expected result of 1.5e3 + 2 * (3.7 - 4)^2
        let expected = 1500.0 + 2.0 * (3.7_f64 - 4.0_f64).powi(2);
        assert_float_eq(eval_str("1.5e3 + 2 * (3.7 - 4)^2").unwrap(), expected);

        assert_float_eq(eval_str("2 * -(3 + 4) * 2^3").unwrap(), -112.0);

        // Additional test cases with exact calculations
        assert_float_eq(eval_str("1 + 2 * (3 - 4)").unwrap(), -1.0);

        // Test with explicit floating point calculations
        let expr = "1.5e3 + 2.0 * (3.7 - 4.0)^2";
        let expected = 1500.0_f64 + 2.0_f64 * (3.7_f64 - 4.0_f64).powi(2);
        assert_float_eq(eval_str(expr).unwrap(), expected);
    }

    #[test]
    fn test_division_by_zero() {
        assert!(matches!(eval_str("1 / 0"), Err(MathError::DivisionByZero)));
        assert!(matches!(
            eval_str("1 / (2 - 2)"),
            Err(MathError::DivisionByZero)
        ));
    }

    #[test]
    fn test_invalid_expression() {
        assert!(matches!(
            eval_str("1 +"),
            Err(MathError::InvalidExpression(_))
        ));
        assert!(matches!(
            eval_str("1 + 2 *"),
            Err(MathError::InvalidExpression(_))
        ));
        assert!(matches!(
            eval_str("1 + 2 * (3 - 4"),
            Err(MathError::InvalidExpression(_))
        ));
        assert!(matches!(
            eval_str("1 + 2 * (3 - 4) +"),
            Err(MathError::InvalidExpression(_))
        ));
        assert!(matches!(
            eval_str("1 + 2 * (3 - 4) + 5 *"),
            Err(MathError::InvalidExpression(_))
        ));
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(eval_str("(1 + 2) * 3").unwrap(), 9.0);
        assert_eq!(eval_str("2 * (3 + 4)").unwrap(), 14.0);
        assert_eq!(eval_str("(1 + 2) * (3 + 4)").unwrap(), 21.0);
    }
}
