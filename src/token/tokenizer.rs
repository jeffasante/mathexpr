//src/token/tokenizer.rs

use crate::{MathError, Operator, Result, Token};

// A function tokenizer that processes input characters into tokens
pub struct Tokenizer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>, // iterate over the characters of the input string
}

impl<'a> Tokenizer<'a> {
    // Create a new tokenizer from input string
    pub fn new(input: &'a str) -> Self {
        Self {
            // input,
            // index: 0,
            chars: input.chars().peekable(),
        }
    }

    // Static method to tokenize an entire string
    pub fn tokenize(input: &'a str) -> Result<Vec<Token>> {
        let mut tokenizer = Self::new(input);
        tokenizer.tokenize_all()
    }

    // Pure function to tokenize the entire input
    pub fn tokenize_all(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }

    // Gets the next token from the input stream
    fn next_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespace();

        match self.chars.peek() {
            None => Ok(None),
            Some(&ch) => match ch {
                '0'..='9' | '.' => self.tokenize_number(), // Delegates number parsing
                '+' => {
                    self.chars.next();
                    Ok(Some(Token::Operator(Operator::Add)))
                }
                '-' => {
                    self.chars.next();
                    Ok(Some(Token::Operator(Operator::Subtract)))
                }
                '*' => {
                    self.chars.next();
                    Ok(Some(Token::Operator(Operator::Multiply)))
                }
                '/' => {
                    self.chars.next();
                    Ok(Some(Token::Operator(Operator::Divide)))
                }
                '^' => {
                    self.chars.next();
                    Ok(Some(Token::Operator(Operator::Power)))
                }
                '(' => {
                    self.chars.next();
                    Ok(Some(Token::LParen))
                }
                ')' => {
                    self.chars.next();
                    Ok(Some(Token::RParen))
                }
                _ => Err(MathError::InvalidExpression(format!(
                    "Unexpected character: {}",
                    ch
                ))),
            },
        }
    }

    // Pure function to tokenize a number, handling both regular and scientific notation
    fn tokenize_number(&mut self) -> Result<Option<Token>> {
        let mut number = String::new();
        let mut is_scientific = false;
        let mut has_decimal = false;

        while let Some(&ch) = self.chars.peek() {
            match ch {
                '0'..='9' => {
                    number.push(ch);
                    self.chars.next();
                }
                '.' => {
                    if has_decimal {
                        return Err(MathError::InvalidExpression(
                            "Multiple decimal points in number".to_string(),
                        ));
                    }
                    has_decimal = true;
                    number.push(ch);
                    self.chars.next();
                }
                'e' | 'E' => {
                    if is_scientific {
                        return Err(MathError::InvalidExpression(
                            "Multiple scientific notation in number".to_string(),
                        ));
                    }
                    is_scientific = true;
                    number.push(ch);
                    self.chars.next();

                    // Handle optional sign in exponent
                    if let Some(&next_ch) = self.chars.peek() {
                        if next_ch == '+' || next_ch == '-' {
                            number.push(next_ch);
                            self.chars.next();
                        }
                    }
                }
                _ if ch.is_whitespace() || "+-*/^()".contains(ch) => break,
                _ => return Err(MathError::InvalidNumber(number)),
            }
        }

        if number.is_empty() {
            return Err(MathError::InvalidExpression("Empty number".to_string()));
        }

        // If it's scientific notation, parse it as such
        if is_scientific {
            self.parse_scientific_notation(&number)
        } else {
            // Otherwise parse as regular number
            number
                .parse::<f64>()
                .map(Token::Number)
                .map(Some)
                .map_err(|_| MathError::InvalidNumber(number))
        }
    }

    // Pure function to parse scientific notation
    fn parse_scientific_notation(&self, number: &str) -> Result<Option<Token>> {
        let parts: Vec<&str> = number.split('e').collect();
        if parts.len() != 2 {
            return Err(MathError::InvalidNumber(number.to_string()));
        }

        let base = parts[0]
            .parse::<f64>()
            .map_err(|_| MathError::InvalidNumber(number.to_string()))?;
        let exponent = parts[1]
            .parse::<i32>()
            .map_err(|_| MathError::InvalidNumber(number.to_string()))?;

        Ok(Some(Token::Scientific {
            base,
            exponent: exponent as i32,
        }))
    }

    // Skip whitespace characters
    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.chars.next();
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let input = "1 + 2.5 * (3-4)";
        let tokens = Tokenizer::tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Number(1.0),                  // 1
                Token::Operator(Operator::Add),      // +
                Token::Number(2.5),                  // 2.5
                Token::Operator(Operator::Multiply), // *
                Token::LParen,                       // (
                Token::Number(3.0),                  // 3
                Token::Operator(Operator::Subtract), // -
                Token::Number(4.0),                  // 4
                Token::RParen,                       // )
            ]
        );
    }

    #[test]
    fn test_scientific_notation() {
        let input = "1.5e3 + 2e-2";
        let tokens = Tokenizer::tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Scientific {
                    base: 1.5,
                    exponent: 3
                },
                Token::Operator(Operator::Add),
                Token::Scientific {
                    base: 2.0,
                    exponent: -2
                },
            ]
        );
    }

    #[test]
    fn test_scientific_notation_2() {
        let input = "1.23e-4 + 5.67e+8";
        let tokens = Tokenizer::tokenize(input).unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Scientific {
                    base: 1.23,
                    exponent: -4
                },
                Token::Operator(Operator::Add),
                Token::Scientific {
                    base: 5.67,
                    exponent: 8
                },
            ]
        );
    }
}

/*
output: 


// Input: "1.5e3 + 2 * (3.7 - 4)^2"
Tokens: [
    Scientific { base: 1.5, exponent: 3 },  // 1.5e3 is preserved as scientific
    Operator(Add),                          // +
    Number(2.0),                           // 2
    Operator(Multiply),                    // *
    LParen,                               // (
    Number(3.7),                          // 3.7
    Operator(Subtract),                   // -
    Number(4.0),                          // 4
    RParen,                               // )
    Operator(Power),                      // ^
    Number(2.0)                           // 2
]
*/


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_tokenize_number() {
//         let mut tokenizer = Tokenizer::new("123");
//         assert_eq!(tokenizer.tokenize_number().unwrap(), Some(Token::Number(123.0)));

//         let mut tokenizer = Tokenizer::new("123.456");
//         assert_eq!(tokenizer.tokenize_number().unwrap(), Some(Token::Number(123.456)));

//         let mut tokenizer = Tokenizer::new("1.23e-4");
//         assert_eq!(tokenizer.tokenize_number().unwrap(), Some(Token::Scientific { base: 1.23, exponent: -4.0 }));

//         let mut tokenizer = Tokenizer::new("1.23E+4");
//         assert_eq!(tokenizer.tokenize_number().unwrap(), Some(Token::Scientific { base: 1.23, exponent: 4.0 }));
//     }

//     #[test]
//     fn test_tokenize() {
//         let mut tokenizer = Tokenizer::new("1 + 2 * 3");
//         assert_eq!(
//             tokenizer.tokenize().unwrap(),
//             vec![
//                 Token::Number(1.0),
//                 Token::Operator(Operator::Add),
//                 Token::Number(2.0),
//                 Token::Operator(Operator::Multiply),
//                 Token::Number(3.0),
//             ]
//         );

//         let mut tokenizer = Tokenizer::new("1.23e-4 + 5.67e+8");
//         assert_eq!(
//             tokenizer.tokenize().unwrap(),
//             vec![
//                 Token::Scientific { base: 1.23, exponent: -4.0 },
//                 Token::Operator(Operator::Add),
//                 Token::Scientific { base: 5.67, exponent: 8.0 },
//             ]
//         );
//     }
// }
