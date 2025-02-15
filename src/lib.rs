// src/lib.rs

// Export our modules
pub mod token;
pub mod error;
pub mod parser;
pub mod expr;
pub mod evaluator;

// Re-export commonly used types for easier access
pub use crate::token::{Token, Operator, Tokenizer};
pub use crate::error::{MathError, Result};
pub use crate::parser::Parser;
pub use crate::expr::Expr;
pub use crate::evaluator::Evaluator;