mod error;
pub mod ll0_parser;

use crate::lexer::Token;

pub type ParserResult<T> = Result<T, error::ParseRegexpError>;

// <regex> ::= <alt>
#[derive(Debug, PartialEq, Eq)]
pub struct Regexp {
    pub val: Alt
}

// <alt> ::= ε | <concat> | <concat> "|" <alt>
// <alt> ::= (<concat>("|" <alt>)?)?
#[derive(Debug, PartialEq, Eq)]
pub struct Alt {
    pub val: Option<Concat>,
    pub tail: Option<Box<Alt>>,
}

// <concat> ::= <factor> <concat> | <factor>
#[derive(Debug, PartialEq, Eq)]
pub struct Concat {
    pub val: Factor,
    pub tail: Option<Box<Concat>>
}

// <factor> ::= <base> <quantifier> | <base>
#[derive(Debug, PartialEq, Eq)]
pub struct Factor {
    pub val: Base,
    pub q: Option<Token>
}

// <base> ::= <character> | "(" <regex> ")"
#[derive(Debug, PartialEq, Eq)]
pub enum Base {
    Char(Token),
    Alt(Box<Alt>)
}

pub trait Parser {
    fn parse(&mut self, tokens: &Vec<Token>) -> ParserResult<Regexp>;
}
