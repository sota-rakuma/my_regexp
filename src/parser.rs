// mod error;
mod error;

use crate::lexer::Token;

use self::error::ParseRegexpError;

pub type ParserResult<T> = Result<T, error::ParseRegexpError>;

// <regex> ::= <alt>
#[derive(Debug, PartialEq, Eq)]
pub struct Regexp {
    pub val: Alt
}

// <alt> ::= <concat> | <concat> "|" <alt>
// <alt> ::= (<concat>("|" <alt>)?)?
#[derive(Debug, PartialEq, Eq)]
pub struct Alt {
    pub val: (Concat, Option<Box<Alt>>)
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

pub struct LL0Parser {
    next_idx: usize,
}

impl LL0Parser {
    pub fn new() -> LL0Parser {
        LL0Parser { next_idx: 0 }
    }

    // <alt> ::= <concat> | <concat> "|" <alt> [$, ")"]
    // <alt> ::= <concat>("|" <alt>)?
    fn parse_alt(&mut self, tokens: &Vec<Token>) -> ParserResult<Alt> {
        let alt = self.parse_concat(tokens)?;
        match self.get_next_token(tokens) {
            Some(Token::Selector) => {
                self.next_idx += 1;
                let tail = self.parse_alt(tokens)?;
                Ok(Alt { val: (alt, Some(Box::new(tail)))})
            },
            _ => Ok(Alt{ val: (alt, None)}),
        }
    }

    // <concat> ::= <factor> <concat> | <factor> [$, "|"]
    fn parse_concat(&mut self, tokens: &Vec<Token>) -> ParserResult<Concat> {
        let factor = self.parse_factor(tokens)?;
        match self.get_next_token(tokens) {
            Some(Token::Char(_)) | Some(Token::Lparen) => {
                let tail = self.parse_concat(tokens)?;
                Ok(Concat{val: factor, tail: Some(Box::new(tail)) })
            },
            _ => Ok(Concat{val: factor, tail: None})
        }
    }

    // <factor> ::= <base> <quantifier> | <base> [$, "|", Char, "("]
    fn parse_factor(&mut self, tokens: &Vec<Token>) -> ParserResult<Factor> {
        let base = self.parse_base(tokens)?;
        match self.get_next_token(tokens) {
            Some(Token::Quantifier(c)) => {
                self.next_idx += 1;
                Ok(Factor{val: base, q: Some(Token::Quantifier(c))})
            },
            _ => Ok(Factor{val: base, q: None})
        }
    }

    // <base> ::= <character> | "(" <alt> ")" [$, "|", Char, "(", Quantifier]
    fn parse_base(&mut self, tokens: &Vec<Token>) -> ParserResult<Base> {
        match self.get_next_token(tokens) {
            Some(Token::Char(c)) => {
                self.next_idx += 1;
                Ok(Base::Char(Token::Char(c)))
            },
            Some(Token::Lparen) => {
                self.next_idx += 1;
                let regexp = Ok(Base::Alt(Box::new(self.parse_alt(tokens)?)));
                let next = self.get_next_token(tokens);
                if next != Some(Token::Rparen) {
                    return Err(ParseRegexpError::new(next, self.next_idx));
                }
                self.next_idx += 1;
                regexp
            },
            c => Err(ParseRegexpError::new(c, self.next_idx)),
        }
    }

    fn get_next_token(&self, tokens: &Vec<Token>) -> Option<Token> {
        if self.next_idx >= tokens.len() {
            None
        } else {
            Some(tokens[self.next_idx])
        }
    }
}

impl Parser for LL0Parser {
    // <regex> ::= <alt>
    fn parse(&mut self, tokens: &Vec<Token>) -> ParserResult<Regexp> {
        let alt = self.parse_alt(tokens)?;
        if let Some(c) = self.get_next_token(tokens) {
            return Err(ParseRegexpError::new(Some(c), self.next_idx));
        }
        Ok(Regexp { val: alt })
    }
}

// テスト書いたらリファクタリングしよ。おやすみ
#[cfg(test)]
mod test {
    mod invalid {
        use crate::lexer::Token;
        use super::super::{error::ParseRegexpError, LL0Parser, Parser};

        #[test]
        fn start_with_quantifier() {
            let tokens = vec![Token::Quantifier('*')];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[0]), 0usize));
            
            let tokens = vec![Token::Quantifier('*'), Token::Char('c')];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[0]), 0usize));

            let tokens = vec![Token::Lparen, Token::Quantifier('*'), Token::Rparen];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[1]), 1usize));
        }

        #[test]
        fn token_before_quantifier_is_not_char() {
            let tokens = vec![Token::Char('a'), Token::Quantifier('*'), Token::Quantifier('*')];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[2]), 2usize));

            let tokens = vec![Token::Char('a'), Token::Quantifier('|'), Token::Quantifier('*')];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[2]), 2usize));

            let tokens = vec![Token::Char('a'), Token::Quantifier('('), Token::Quantifier('*')];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[2]), 2usize));

            let tokens = vec![Token::Char('a'), Token::Char('a'), Token::Quantifier('?'), Token::Char('a'), Token::Quantifier('?'), Token::Quantifier('*')];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[5]), 5usize));
        }

        #[test]
        fn start_with_selector() {
            let tokens = vec![Token::Selector];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[0]), 0usize));

            let tokens = vec![Token::Selector, Token::Char('c')];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[0]), 0usize));

            let tokens = vec![Token::Lparen, Token::Quantifier('|'), Token::Rparen];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[1]), 1usize));
        }

        #[test]
        fn continuous_selector() {
            let tokens = vec![Token::Char('c'), Token::Selector, Token::Selector];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[2]), 2usize));

            let tokens = vec![Token::Char('c'), Token::Selector, Token::Selector, Token::Char('c')];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[2]), 2usize));
        }

        #[test]
        fn terminated_by_selector() {
            let tokens = vec![Token::Char('a'), Token::Char('a'), Token::Selector];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(None, 3usize));

            let tokens = vec![Token::Char('a'), Token::Char('a'), Token::Char('a'), Token::Char('a'), Token::Selector];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(None, 5usize));

            let tokens = vec![Token::Lparen, Token::Char('a'), Token::Quantifier('?'), Token::Rparen, Token::Selector];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(None, 5usize));
        }

        #[test]
        fn not_grouped() {
            let tokens = vec![Token::Rparen];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(Token::Rparen), 0usize));

            let tokens = vec![Token::Char('c'), Token::Rparen];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(Token::Rparen), 1usize));

            let tokens = vec![Token::Lparen];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(None, 1usize));

            let tokens = vec![Token::Lparen, Token::Char('c')];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(None, 2usize));

            let tokens = vec![Token::Lparen, Token::Char('a'), Token::Rparen, Token::Rparen];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(Token::Rparen), 3usize));

            let tokens = vec![Token::Lparen, Token::Lparen, Token::Char('a'), Token::Rparen];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(None, 4usize));
        }
    }

    mod valid {
        use std::collections::HashMap;

        use crate::{lexer::Token, parser::{Alt, Base, Concat, Factor, LL0Parser, Parser, Regexp}};

        fn wrap_regexp(val: Alt) -> Regexp {
            Regexp { val }
        }

        fn wrap_alt(val: Concat, tail: Option<Alt>) -> Alt {
            match tail {
                Some(v) => Alt { val: (val, Some(Box::new(v))) },
                None => Alt {val: (val, None)}
            }
            
        }

        fn wrap_concat(val: Factor, tail: Option<Concat>) -> Concat {
            match tail {
                Some(v) => Concat{val, tail: Some(Box::new(v))},
                None => Concat{val, tail: None}
            }
        }

        fn wrap_factor(val: Base, q: Option<Token>) -> Factor {
            Factor{val, q}
        }

        fn create_concat(mut v: Vec<Factor>) -> Concat {
            v.reverse();
            let concat = v.into_iter().fold(None, |concat, val| {
                let parent = Concat {
                    val,
                    tail: concat,
                };
                Some(Box::new(parent))
            });

            *concat.unwrap()
        }

        #[test]
        fn only_char() {
            let tokens = vec![Token::Char('a')];
            let expected = wrap_regexp(
                wrap_alt(
                    wrap_concat(
                        wrap_factor(Base::Char(tokens[0]), None), 
                        None), 
                None), 
            );
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));


            let tokens = vec![Token::Char('a'), Token::Char('a'), Token::Char('a'), Token::Char('a')];
            let expected_factors = tokens.clone().into_iter().map(|v| {
                Factor{
                    val: Base::Char(v),
                    q: None,
                }
            })
            .collect();
            let expected_concat = create_concat(expected_factors);
            let expected = Regexp { val: (Alt { val:(expected_concat, None) }) };
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));
        }

        #[test]
        fn group() {
            let tokens = vec![Token::Lparen, Token::Char('a'), Token::Quantifier('?'), Token::Rparen];
            let expected_inner_node = Alt {
                val: (
                    Concat{
                        val: Factor { 
                            val: Base::Char(Token::Char('a')), 
                            q: Some(Token::Quantifier('?')),
                        },
                        tail: None,
                    }, 
                    None
                ),
            };

            let expected = Regexp {
                val: Alt {
                    val: (
                        Concat{
                            val: Factor { 
                                val: Base::Alt(Box::new(expected_inner_node)), 
                                q: None,
                            },
                            tail: None,
                        }, 
                        None
                    ),
                }
            };
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));
        }

        #[test]
        fn complexed_test() {
            // a(bc|(def|ghi*)j?)*|kl.*mn?
            let tokens = vec![
                Token::Char('a'),
                Token::Lparen,
                Token::Char('b'),
                Token::Char('c'),
                Token::Selector,
                Token::Lparen,
                Token::Char('d'),
                Token::Char('e'),
                Token::Char('f'),
                Token::Selector,
                Token::Char('g'),
                Token::Char('h'),
                Token::Char('i'),
                Token::Quantifier('*'),
                Token::Rparen,
                Token::Char('j'),
                Token::Quantifier('?'),
                Token::Rparen,
                Token::Quantifier('*'),
                Token::Selector,
                Token::Char('k'),
                Token::Char('l'),
                Token::Char('.'),
                Token::Quantifier('*'),
                Token::Char('m'),
                Token::Char('n'),
                Token::Quantifier('?'),
                // Token
            ];
            let one_chars = "bcdefghklm".chars().map(|c| {
                (
                    c.to_string(),
                    Factor{
                        val: Base::Char(Token::Char(c)),
                        q: None
                    }
                )
            })
            .collect::<HashMap<String, Factor>>();

            let with_qs = vec![
                (Token::Char('i'), Token::Quantifier('*')),
                (Token::Char('j'), Token::Quantifier('?')),
                (Token::Char('.'), Token::Quantifier('*')),
                (Token::Char('n'), Token::Quantifier('?')),
            ].into_iter().map(|tuple| {
                (
                    String::from_iter(vec![tuple.0.to_char(), tuple.1.to_char()]),
                    Factor{
                        val: Base::Char(tuple.0),
                        q: Some(tuple.1),
                    }
                )
            })
            .collect::<HashMap<String, Factor>>();

            let mut factor_map = one_chars.into_iter().chain(with_qs).collect::<HashMap<String, Factor>>();

            //a, bc, def, ghi*, j?, kl.*mn?
            let mut concats = vec![
                vec![factor_map.remove("b").unwrap(), factor_map.remove("c").unwrap()],
                vec![factor_map.remove("d").unwrap(), factor_map.remove("e").unwrap(), factor_map.remove("f").unwrap()],
                vec![factor_map.remove("g").unwrap(), factor_map.remove("h").unwrap(), factor_map.remove("i*").unwrap()],
                vec![factor_map.remove("j?").unwrap()],
                vec![factor_map.remove("k").unwrap(), factor_map.remove("l").unwrap(), factor_map.remove(".*").unwrap(), factor_map.remove("m").unwrap(), factor_map.remove("n?").unwrap()],
            ].into_iter().map(|v| {
                create_concat(v)
            }).collect::<Vec<Concat>>();

            //(def|ghi*)
            let innermost = Alt{
                val: (
                    concats.remove(1), 
                    Some(Box::new(Alt { val: (concats.remove(1), None) }))
                )
            };

            // (def|ghi*)j?
            let right_alt_in_left_group = Alt{
                val: (
                    Concat{
                        val: Factor { val: Base::Alt(Box::new(innermost)), q: None },
                        tail: Some(Box::new(concats.remove(1)))
                    },
                    None
                ),
            };

            // (bc|(def|ghi*)j?)
            let left_group = Alt{
                val: (
                    concats.remove(0),
                    Some(Box::new(right_alt_in_left_group)),
                ),
            };

            // a(bc|(def|ghi*)j?)*|kl.*mn?
            let alt = Alt{
                val: (
                    Concat{
                        val: Factor { val: Base::Char(Token::Char('a')), q: None },
                        tail: Some(
                            Box::new(Concat{
                                val: Factor { 
                                    val: Base::Alt(Box::new(left_group)), 
                                    q: Some(Token::Quantifier('*')),
                                },
                                tail: None,
                            })
                        ),
                    },
                    Some(Box::new(Alt { val: (concats.remove(0), None) })),
                )
            };

            let expected = Regexp{val: alt};
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));
        }
    }
}