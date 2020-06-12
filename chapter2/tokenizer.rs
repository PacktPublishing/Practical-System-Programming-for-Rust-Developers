// Standard lib
use std::error;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

// Primary external libraries

// Utility external libraries

//Other internal modules
use super::token::Token;

// Other structs

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..='9') => {
                let mut number = next_char.unwrap().to_string();
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next().unwrap());
                    } else {
                        break;
                    }
                }
                Some(Token::Num(number.parse::<f64>().unwrap()))
            }
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}

#[derive(Debug)]
/// Defines the various errors that can occur during evaluation.
pub enum TokenizerError {
    CharacterIsInvalid(String),
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::TokenizerError::*;

        match *self {
            CharacterIsInvalid(ref e) => write!(f, "Lexing error: {}", e),
        }
    }
}

impl error::Error for TokenizerError {
    fn description(&self) -> &str {
        use self::TokenizerError::*;

        match *self {
            CharacterIsInvalid(ref e) => e,
        }
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0))
    }
    #[test]
    fn test_decimal_number() {
        let mut tokenizer = Tokenizer::new("34.5");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5))
    }
    #[test]
    #[ignore]
    fn test_invalid_char() {
        let mut tokenizer = Tokenizer::new("#$%");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.5));
    }
}
