// src/parser.rs
use crate::{expr::Expr, MathError, Operator, Result, Token};
// A parser that processes tokens into an expression tree

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    // Creates a new parser from a vector of tokens
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // Parse an expression with a minimum precedence level
    fn parse_expression(&mut self, min_precedence: u8) -> Result<Expr> {
        let mut lhs = self.parse_primary()?; // Parse the left-hand side of the expression

        // Loop to parse binary operators
        while let Some(token) = self.peek() {
            // Peek at the next token
            if let Token::Operator(op) = token {
                // Check if the token is an operator
                let precedence = op.precedence(); // Get the precedence of the operator
                if precedence < min_precedence {
                    // If the precedence is less than the minimum, break
                    break;
                }
                self.advance(); // Consume the operator token
                
                let rhs = self.parse_expression(precedence + 1)?; // Recursively parse the right-hand side
                lhs = Expr::BinOp {
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                };
            } else if matches!(token, Token::RParen) {
                // If we encounter a right parenthesis, break the loop
                break;
            } else {
                return Err(MathError::UnexpectedToken(token.clone()));
            }
        }

        Ok(lhs) // Return the parsed expression
    }


    // Parses the tokens into an expression tree
    pub fn parse(&mut self) -> Result<Expr> {
        self.parse_expression(0)
    }


    // Parses a primary expression (e.g., number, parenthesis, or unary minus etc.)
    fn parse_primary(&mut self) -> Result<Expr> {
        let token = self.next().ok_or_else(|| {
            MathError::InvalidExpression("Unexpected end of input".to_string())
        })?;

        match token {
            Token::Number(n) => Ok(Expr::Literal(n)),
            Token::Scientific { base, exponent } => Ok(Expr::Scientific { base, exponent }), // If it's a scientific notation, return a scientific expression
            Token::Operator(Operator::Subtract) => {
                let expr = self.parse_primary()?; // Recursively parse the expression after the unary minus
                Ok(Expr::UnaryMinus(Box::new(expr))) // Return a unary minus expression
            }

            Token::LParen => {
                let expr = self.parse_expression(0)?; // Recursively parse the expression inside the parenthesis
                match self.next() {
                    Some(Token::RParen) => Ok(expr), // If the next token is a right parenthesis, return the expression
                    _ => Err(MathError::InvalidExpression("Expected ')'".to_string())),
                }
            }
            _ => Err(MathError::UnexpectedToken(token)), // If the token is unexpected, return an error
        }
    }

    // Peeks at the next token without consuming it
    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    // Advances to and returns the next token
    fn next(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.current).cloned();
        self.current += 1;
        token
    }

    // Advances to the current token
    fn advance(&mut self) {
        self.current += 1;
    }
}


#[cfg(test)]
mod tests {
    use super::*; // Import names from the parent module
    use crate::Tokenizer; // Import the Tokenizer

    #[test]
    fn test_basic_arithmetic() {
        let input = "2 + 3 * 4";
        let tokens = Tokenizer::tokenize(input).unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        // Expected: 2 + (3 * 4)

        // Check the expression tree
        assert_eq!(expr, Expr::binary(
            Operator::Add,
            Expr::literal(2.0),
            Expr::binary(
                Operator::Multiply,
                Expr::literal(3.0),
                Expr::literal(4.0),
            ),
        ));

        // Check the string representation
        assert_eq!(expr.to_string(), "2 + 3 * 4");
    }

    #[test]
    fn test_parentheses() {
        let input = "(2 + 3) * 4";
        let tokens = Tokenizer::tokenize(input).unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        // Expected: (2 + 3) * 4

        // Check the expression tree
        assert_eq!(expr, Expr::binary(
            Operator::Multiply,
            Expr::binary(
                Operator::Add,
                Expr::literal(2.0),
                Expr::literal(3.0),
            ),
            Expr::literal(4.0),
        ));

        // Check the string representation
        assert_eq!(expr.to_string(), "(2 + 3) * 4");
    }

    #[test]
    fn test_scientific_notation() {
        let input = "2e10 + 3e-2 + 2";
        let tokens = Tokenizer::tokenize(input).unwrap();
        let mut parser = Parser::new(tokens);
        let expr = parser.parse().unwrap();

        // Expected: 2e10 + 3e-2 + 2

        // Check the expression tree
        assert_eq!(expr, Expr::binary(
            Operator::Add,
            Expr::binary(
                Operator::Add,
                Expr::scientific(2.0, 10),
                Expr::scientific(3.0, -2)
            ),
            Expr::literal(2.0)
        ));

        // Check the string representation
        assert_eq!(expr.to_string(), "2e10 + 3e-2 + 2");
    }
}