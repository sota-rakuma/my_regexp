// mod error;
mod error;

use crate::lexer::Token;

use self::error::ParseRegexpError;

pub type ParserResult<T> = Result<T, error::ParseRegexpError>;

// <regex> ::= <alt>
pub struct Regexp {
    pub val: Alt
}

// <alt> ::= <concat> | <concat> "|" <alt>
// <alt> ::= (<concat>("|" <alt>)?)?
pub struct Alt {
    pub val: (Concat, Option<Box<Alt>>)
}

// <concat> ::= <factor> <concat> | <factor>
pub struct Concat {
    pub val: Factor,
    pub tail: Option<Box<Concat>>
}

// <factor> ::= <base> <quantifier> | <base>
pub struct Factor {
    pub val: Base,
    pub q: Option<Token>
}

// <base> ::= <character> | "(" <regex> ")"
pub enum Base {
    Char(Token),
    Regexp(Box<Regexp>)
}

pub trait Parser {
    fn parse(&mut self, tokens: &Vec<Token>) -> ParserResult<Regexp>;
}

pub struct LL1Parser {
    cur_idx: usize,
}

// 先読みがいるかどうか考える
// 今のままだと、エラーがBaseでしか出せない
// 多分今のままだと、a**みたいなのを受容してしまう気がする。
// ↑しないわ。baseでエラー吐く。
impl LL1Parser {
    pub fn new() -> LL1Parser {
        LL1Parser { cur_idx: 0 }
    }

    // <alt> ::= <concat> | <concat> "|" <alt> [$, ")"]
    // <alt> ::= <concat>("|" <alt>)?
    fn parse_alt(&mut self, tokens: &Vec<Token>) -> ParserResult<Alt> {
        let alt = self.parse_concat(tokens)?;
        let alt = match self.get_cur_token(tokens) {
            Some(Token::Selector) => {
                let tail = self.parse_alt(tokens)?;
                Ok(Alt { val: (alt, Some(Box::new(tail)))})
            },
            _ => Ok(Alt{ val: (alt, None)})
        };
        self.cur_idx += 1;
        alt
    }

    // <concat> ::= <factor> <concat> | <factor>
    fn parse_concat(&mut self, tokens: &Vec<Token>) -> ParserResult<Concat> {
        let factor = self.parse_factor(tokens)?;
        let factor = match self.get_cur_token(tokens) {
            Some(Token::Char(_)) | Some(Token::Lparen) => {
                let tail = self.parse_concat(tokens)?;
                Ok(Concat{val: factor, tail: Some(Box::new(tail)) })
            },
            _ => Ok(Concat{val: factor, tail: None})
        };
        self.cur_idx += 1;
        factor
    }

    // <factor> ::= <base> <quantifier> | <base>
    fn parse_factor(&mut self, tokens: &Vec<Token>) -> ParserResult<Factor> {
        let base = self.parse_base(tokens)?;
        let base = match self.get_cur_token(tokens) {
            Some(Token::Quantifier(c)) => {
                Ok(Factor{val: base, q: Some(Token::Quantifier(c))})
            },
            _ => Ok(Factor{val: base, q: None})
        };
        self.cur_idx += 1;
        base
    }

    // <base> ::= <character> | "(" <regex> ")"
    fn parse_base(&mut self, tokens: &Vec<Token>) -> ParserResult<Base> {
        let base = match self.get_cur_token(tokens) {
            Some(Token::Char(c)) => Ok(Base::Char(Token::Char(c))),
            Some(Token::Lparen) => {
                self.cur_idx += 1;
                let regexp = Ok(Base::Regexp(Box::new(self.parse(tokens)?)));
                let next = self.get_cur_token(tokens);
                if next != Some(Token::Rparen) {
                    return Err(ParseRegexpError::new(next, self.cur_idx));
                }
                self.cur_idx += 1;
                regexp
            },
            c => Err(ParseRegexpError::new(c, self.cur_idx)),
        };
        self.cur_idx += 1;
        base
    }

    fn get_cur_token(&self, tokens: &Vec<Token>) -> Option<Token> {
        if self.cur_idx + 1 >= tokens.len() {
            None
        } else {
            Some(tokens[self.cur_idx])
        }
    }
}

impl Parser for LL1Parser {
    // <regex> ::= <alt>
    fn parse(&mut self, tokens: &Vec<Token>) -> ParserResult<Regexp> {
        let alt = self.parse_alt(tokens)?;
        Ok(Regexp { val: alt })
    }
}

// テスト書いたらリファクタリングしよ。おやすみ
#[cfg(test)]
mod test {}