// src/error.rs

use std::fmt;
mod tokenizer;
pub use tokenizer::Tokenizer;

use crate::Expr;

// Token definition 
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Operator(Operator),
    LParen,
    RParen,
    Scientific { base: f64, exponent: i32 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl Operator {
    // Pure function to get operator precedence
    pub fn precedence(&self) -> u8 {
        match self {
            Operator:: Add | Operator::Subtract => 1,
            Operator::Multiply | Operator::Divide => 2,
            Operator::Power => 3,
        }
    }

    // Pure function to get operato symbol
    pub fn symbol(&self) -> char {
        match self {
            Operator::Add => '+',
            Operator::Subtract => '-',
            Operator::Multiply => '*',
            Operator::Divide => '/',
            Operator::Power => '^',
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{}", n),
            Token::Operator(op) => write!(f, "{}", op.symbol()),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Scientific { base, exponent } => write!(f, "{}e{}", base, exponent),
        }
    }
}


impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Literal(value) => write!(f, "{}", value),
            Expr::Scientific { base, exponent } => write!(f, "{}e{}", base, exponent),
            Expr::UnaryMinus(expr) => write!(f, "-({})", expr),
            Expr::BinOp { op, lhs, rhs } => {
                // Handle operator precedence for proper parentheses
                let need_parens_left = match (&**lhs, op) {
                    (Expr::BinOp { op: inner_op, .. }, outer_op) => {
                        inner_op.precedence() < outer_op.precedence()
                    }
                    _ => false,
                };

                let need_parens_right = match (&**rhs, op) {
                    (Expr::BinOp { op: inner_op, .. }, outer_op) => {
                        inner_op.precedence() <= outer_op.precedence()
                    }
                    _ => false,
                };

                // Write left expression with optional parentheses
                if need_parens_left {
                    write!(f, "({})", lhs)?;
                } else {
                    write!(f, "{}", lhs)?;
                }

                // Write operator
                write!(f, " {} ", op.symbol())?;

                // Write right expression with optional parentheses
                if need_parens_right {
                    write!(f, "({})", rhs)
                } else {
                    write!(f, "{}", rhs)
                }
            }
        }
    }
}