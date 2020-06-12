// Standard lib
use std::error;
use std::fmt;
//Primary external libraries

//utility externa libraries

//internal modules

//structs

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Caret(Box<Node>, Box<Node>),
    LeftParen(Box<Node>, Box<Node>),
    RightParen(Box<Node>, Box<Node>),
    Negative(Box<Node>),
    Absolute(Box<Node>),
    Number(f64),
}

pub fn eval(expr: Node) -> Result<f64, EvaluationError> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(expr1, expr2) => Ok(eval(*expr1)? + eval(*expr2)?),
        Subtract(expr1, expr2) => Ok(eval(*expr1)? - eval(*expr2)?),
        Multiply(expr1, expr2) => Ok(eval(*expr1)? * eval(*expr2)?),
        Divide(expr1, expr2) => Ok(eval(*expr1)? / eval(*expr2)?),
        Negative(expr1) => Ok(-(eval(*expr1)?)),
        Caret(expr1, expr2) => Ok(eval(*expr1)?.powf(eval(*expr2)?)),
        Absolute(expr1) => Ok(eval(*expr1)?.abs()),
        _ => Err(EvaluationError::UnableToEvaluate(
            "No clue, sorry".to_string(),
        )),
    }
}
#[derive(Debug)]
pub enum EvaluationError {
    UnableToEvaluate(String),
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            self::EvaluationError::UnableToEvaluate(e) => write!(f, "Error in evaluating {}", e),
        }
    }
}
impl error::Error for EvaluationError {
    fn description(&self) -> &str {
        match &self {
            self::EvaluationError::UnableToEvaluate(e) => &e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_expr1() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("1+2-3").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 0.0);
    }
    #[test]
    fn test_expr2() {
        use crate::parsemath::parser::Parser;

        let ast = Parser::new("3+2-1*5/4").unwrap().parse().unwrap();
        let value = eval(ast).unwrap();
        assert_eq!(value, 3.75);
    }
}
