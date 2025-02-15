//src/expr.rs
use crate::Operator;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // A literal number value
    Literal(f64),

    // Note: We use Box<Expr> to store the expression on the heap and also avoid excessive memory usage.
    /*

        // Example: AST Representation for `-(3 + 2)`
        Mathematical expression:

        -(3 + 2)

        How it looks in an AST:

        Expr::UnaryMinus(Box::new(
            Expr::BinOp {
                op: Operator::Add,
                lhs: Box::new(Expr::Literal(3.0)),
                rhs: Box::new(Expr::Literal(2.0)),
            }
        ))

    */

    // A binary operation (e.g., addition, subtraction, etc.)
    BinOp {
        op: Operator,   // The operator to apply
        lhs: Box<Expr>, // The left-hand side of the operation
        rhs: Box<Expr>, // The right-hand side of the operation
    },

    // A unary minus operation (e.g., -5)
    UnaryMinus(Box<Expr>),

    // Scientific notation (e.g., 1e3)
    Scientific {
        base: f64,     // The base value
        exponent: i32, // The exponent value
    },
}

impl Expr {
    // Creates a new literal expression
    pub fn literal(value: f64) -> Self {
        Expr::Literal(value)
    }

    // Creates a new bianry operation expression
    pub fn binary(op: Operator, lhs: Expr, rhs: Expr) -> Self {
        Expr::BinOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    // Creates a new unary minus expression
    pub fn unary_minus(expr: Expr) -> Self {
        Expr::UnaryMinus(Box::new(expr))
    }

    // Creates a new scientific notation expression
    pub fn scientific(base: f64, exponent: i32) -> Self {
        Expr::Scientific { base, exponent }
    }
}
