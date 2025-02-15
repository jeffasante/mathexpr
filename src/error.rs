// src/error.rs

use thiserror::Error;
use crate::token::Token;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("Invalid token: {0}")]
    UnexpectedToken(Token),

    #[error("Unmatched parenthesis")]
    UnmatchedParenthesis,

    #[error("Invalid number format: {0}")]
    InvalidNumber(String),

    #[error("Divisioin by zero")]
    DivisionByZero,

    #[error("Invalid operator: {0}")]
    InvalidExpression(String),
}

pub type Result<T> = std::result::Result<T, MathError>;
