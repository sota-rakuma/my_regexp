use crate::lexer::{Quantifier, Char};

// <regex> ::= <alt>
pub struct Regexp {
    pub val: Alt
}

// <alt> ::= <concat> | <concat> "|" <alt> | ε
// <alt> ::= (<concat>("|" <alt>)?)?
pub struct Alt {
    pub val: Option<(Concat, Option<Box<Alt>>)>
}


// <concat> ::= <factor> <concat> | <factor>
pub struct Concat {
    pub val: Factor,
    pub tail: Option<Box<Concat>>
}

// <factor> ::= <base> <quantifier> | <base>
pub struct Factor {
    pub val: Base,
    pub q: Option<Quantifier>
}

// <base> ::= <character> | "(" <regex> ")"
pub enum Base {
    Char(Char),
    Regexp(Box<Regexp>)
}
