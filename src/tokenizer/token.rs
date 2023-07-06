use num_rational::BigRational;
use num_traits::ToPrimitive;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(BigRational),
    Unit(String),
    Add,
    Sub,
    Mul,
    Div,
    VarStore(String),
    VarRecall(String),
    Operator(char),
    MacroInvoke((String, String)),
    Comment(String),
}

impl Token {
    pub fn roughly_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::Number(a), Token::Number(b)) => a.to_f64().unwrap() == b.to_f64().unwrap(),
            (Token::Unit(a), Token::Unit(b)) => a == b,
            (Token::Add, Token::Add) => true,
            (Token::Sub, Token::Sub) => true,
            (Token::Mul, Token::Mul) => true,
            (Token::Div, Token::Div) => true,
            (Token::Operator(a), Token::Operator(b)) => a == b,
            _ => false,
        }
    }
}
