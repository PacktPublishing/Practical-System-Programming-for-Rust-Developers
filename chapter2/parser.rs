// Standard lib
use std::error;
use std::fmt;
// Primary external libs

// Secondary internal libs

// Internal modules
use super::ast::Node;
use super::token::{OperPrec, Token};
use super::tokenizer::Tokenizer;
//Structs and constants

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = lexer.next().unwrap();
        Ok(Parser {
            tokenizer: lexer,
            current_token: cur_token,
        })
    }
    fn next_token(&mut self) -> Result<(), ParseError> {
        self.current_token = self.tokenizer.next().unwrap();
        Ok(())
    }
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);
        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }

    fn generate_ast(&mut self, oper_prec: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_prec < self.current_token.get_oper_prec() {
            if self.current_token == Token::EOF {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr.clone())?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.current_token.clone();
        match token {
            Token::Subtract => {
                self.next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;
                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.next_token()?;
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.expect(Token::RightParen)?;
                if self.current_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }

                Ok(expr)
            }
            _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
        }
    }

    fn expect(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.current_token {
            self.next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidOperator(format!(
                "Expected {:?}, got {:?}",
                expected, self.current_token
            )))
        }
    }

    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.current_token {
            Token::Add => {
                self.next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Subtract => {
                self.next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Multiply => {
                self.next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.next_token()?;
                //Get right-side expression
                let right_expr = self.generate_ast(OperPrec::Power)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(ParseError::InvalidOperator(format!(
                "Please enter valid operator {:?}",
                self.current_token
            ))),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnableToParse(String),
    InvalidOperator(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            self::ParseError::UnableToParse(e) => write!(f, "Error in evaluating {}", e),
            self::ParseError::InvalidOperator(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}
impl error::Error for ParseError {
    fn description(&self) -> &str {
        match &self {
            self::ParseError::UnableToParse(e) => &e,
            self::ParseError::InvalidOperator(e) => &e,
        }
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsemath::ast::Node::{Add, Number};
    #[test]
    fn test_addition() {
        let mut parser = Parser::new("1+2").unwrap();
        let expected = Add(Box::new(Number(1.0)), Box::new(Number(2.0)));
        assert_eq!(parser.parse().unwrap(), expected);
    }
}
