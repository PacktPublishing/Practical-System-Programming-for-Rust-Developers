#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(f64),
    EOF,
}

#[derive(Debug, PartialEq, PartialOrd)]
/// Defines all the OperPrec levels, from lowest to highest.
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;
        match *self {
            Add | Subtract => AddSub,
            Multiply | Divide => MulDiv,
            Caret => Power,

            _ => DefaultZero,
        }
    }
}
