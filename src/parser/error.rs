use std::{error::Error, fmt::Display};
use crate::parser::Token;

#[derive(Debug)]
pub struct ParseRegexpError {
    cause: Option<Token>,
    idx: usize
}

impl ParseRegexpError {
    pub fn new(cause: Option<Token>, idx: usize) -> ParseRegexpError {
        ParseRegexpError {cause, idx}
    }
}

impl Display for ParseRegexpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cause == None {
            return write!(f, "変なところで終わってんで");
        }
        write!(f, "cannot parse {:?} at {:?}", self.cause, self.idx)
    }
}

impl Error for ParseRegexpError {}
