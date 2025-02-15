//! # Math Expression Evaluator
//! Author: Jeffrey Oduro Asante
//! GitHub: [https://github.com/jeffasante](https://github.com/jeffasante)
//!
//! This program tokenizes, parses, and evaluates mathematical expressions.

//src/main.rs
use mathexpr::{Evaluator, Parser, Tokenizer};
use std::env;


fn process_expression(input: &str) {
    println!("Input: {}", input);

    // First tokenize
    match Tokenizer::tokenize(input) {
        Ok(tokens) => {
            println!("\nTokens: {:#?}", tokens);
            
            // Then parse
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(expr) => {
                    println!("\nParsed Expression: {}", expr);
                    println!("\nExpression Tree: {:#?}", expr);

                    // Finally evaluate
                    match Evaluator::evaluate(&expr) {
                        Ok(result) => println!("\nResult: {}", result),
                        Err(e) => println!("Evaluation Error: {}", e),
                    }
                }
                Err(e) => println!("Parsing Error: {}", e),
            }
        }
        Err(e) => println!("Tokenization Error: {}", e),
    }
}


fn print_usage() {
    println!("Usage: mathexpr [EXPRESSION]");
    println!("\nExamples:");
    println!("  mathexpr \"2 + 3 * 4\"");
    println!("  mathexpr \"1.5e3 + 2 * (3.7 - 4)^2\"");
    println!("  mathexpr \"(2 + 3) * 4\"");
    println!("\nIf no expression is provided, a default example will be used.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        // No arguments provided - use default example
        1 => process_expression("1.5e3 + 2 * (3.7 - 4)^2"),
        
        // Expression provided as argument
        2 => {
            if args[1] == "-h" || args[1] == "--help" {
                print_usage();
            } else {
                process_expression(&args[1]);
            }
        },
        
        // Too many arguments
        _ => {
            println!("Error: Too many arguments provided.");
            print_usage();
        }
    }
}