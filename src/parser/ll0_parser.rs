use crate::lexer::Token;

use super::{error::ParseRegexpError, Alt, Base, Concat, Factor, Parser, ParserResult, Regexp};

pub struct LL0Parser {
    next_idx: usize,
}

impl LL0Parser {
    pub fn new() -> LL0Parser {
        LL0Parser { next_idx: 0 }
    }

    // <alt> ::= ε | <concat> | <alt> "|" <alt>

    // <alt> ::= ε | <concat><alt'>
    // <alt'> ::= ε | "|" <alt><alt'>
    fn parse_alt(&mut self, tokens: &Vec<Token>) -> ParserResult<Alt> {
        let concat = match self.get_next_token(tokens) {
            None | Some(Token::Selector) | Some(Token::Rparen) => None,
            Some(_c) =>  Some(self.parse_concat(tokens)?)
        };

        match self.get_next_token(tokens) {
            Some(Token::Selector) => {
                self.next_idx += 1;
                let tail = self.parse_alt(tokens)?;
                Ok(Alt { val: concat, tail: Some(Box::new(tail)) })
            },
            _ => Ok(Alt{ val: concat, tail: None }),
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
            cause => Err(ParseRegexpError::new(cause, self.next_idx)),
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
    fn parse(&mut self, tokens: &Vec<Token>) -> ParserResult<Regexp> {
        let alt = self.parse_alt(tokens)?;
        if let Some(c) = self.get_next_token(tokens) {
            return Err(ParseRegexpError::new(Some(c), self.next_idx));
        }
        Ok(Regexp { val: alt })
    }
}

#[cfg(test)]
mod test {
    mod invalid {
        use crate::{lexer::Token, parser::{error::ParseRegexpError, ll0_parser::LL0Parser, Parser}};

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

            let tokens = vec![Token::Char('a'), Token::Char('a'), Token::Quantifier('*'), Token::Char('a'), Token::Quantifier('*'), Token::Quantifier('*')];
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual.unwrap_err(), ParseRegexpError::new(Some(tokens[5]), 5usize));
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

        use crate::{lexer::Token, parser::{ll0_parser::LL0Parser, Alt, Base, Concat, Factor, Parser, Regexp}};

        fn wrap_regexp(val: Alt) -> Regexp {
            Regexp { val }
        }

        fn wrap_alt(val: Option<Concat>, tail: Option<Alt>) -> Alt {
            match tail {
                Some(v) => Alt { val, tail: Some(Box::new(v)) },
                None => Alt {val, tail: None}
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
        fn empty() {
            let tokens = vec![];
            let expected = wrap_regexp(
                wrap_alt(
                    None,
                    None
                ), 
            );
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));
        }

        #[test]
        fn only_char() {
            let tokens = vec![Token::Char('a')];
            let expected = wrap_regexp(
                wrap_alt(
                    Some(wrap_concat(
                        wrap_factor(Base::Char(tokens[0]), None), 
                        None)), 
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
            let expected = Regexp { val: (Alt { val:Some(expected_concat), tail: None }) };
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));
        }

        #[test]
        fn start_with_selector() {
            let tokens = vec![Token::Selector];
            let expected = wrap_regexp(
                wrap_alt(
                    None,
                    Some(Alt { val: None, tail: None })
                ), 
            );
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));

            let tokens = vec![Token::Selector, Token::Char('c')];
            let expected = wrap_regexp(
                wrap_alt(
                    None,
                    Some(
                        wrap_alt(
                            Some(wrap_concat(
                                wrap_factor(Base::Char(Token::Char('c')), None), 
                                None)
                            ), 
                            None,
                        )
                    ),
                ), 
            );
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));

            let tokens = vec![Token::Lparen, Token::Selector, Token::Rparen];
            let expected = wrap_regexp(
                wrap_alt(
                    Some(wrap_concat(
                        wrap_factor(
                            Base::Alt(
                                Box::new(wrap_alt(
                                    None,
                                    Some(Alt { val: None, tail: None })
                                )),
                            ),
                        None),
                        None
                    )),
                    None
                )
            );
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));
        }

        #[test]
        fn continuous_selector() {
            let tokens = vec![Token::Char('c'), Token::Selector, Token::Selector];
            let expected = wrap_regexp(
                wrap_alt(
                    Some(
                        wrap_concat(
                            wrap_factor(Base::Char(Token::Char('c')), None), 
                            None
                        )
                    ),
                    Some(
                        wrap_alt(
                            None, 
                            Some(
                                wrap_alt(None, None)
                            )
                        )
                    )
                ), 
            );
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));

            let tokens = vec![Token::Char('c'), Token::Selector, Token::Selector, Token::Char('c')];
            let expected = wrap_regexp(
                wrap_alt(
                    Some(
                        wrap_concat(
                            wrap_factor(Base::Char(Token::Char('c')), None), 
                            None
                        )
                    ),
                    Some(
                        wrap_alt(
                            None,
                            Some(
                                wrap_alt(
                                    Some(wrap_concat(
                                        wrap_factor(Base::Char(Token::Char('c')), None), 
                                        None
                                    )),
                                    None
                                ),
                            ),
                        )
                    )
                )
            );
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));
        }

        #[test]
        fn group() {
            let tokens = vec![Token::Lparen, Token::Char('a'), Token::Quantifier('*'), Token::Rparen];
            let expected_inner_node = Alt {
                val: Some(Concat{
                        val: Factor { 
                            val: Base::Char(Token::Char('a')), 
                            q: Some(Token::Quantifier('*')),
                        },
                        tail: None,
                    }, 
                ),
                tail: None
            };

            let expected = Regexp {
                val: Alt {
                    val: Some(
                        Concat{
                            val: Factor { 
                                val: Base::Alt(Box::new(expected_inner_node)), 
                                q: None,
                            },
                            tail: None,
                        }, 
                    ),
                    tail: None
                }
            };
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));
        }

        #[test]
        fn complexed_test() {
            // a(bc|(def|ghi*)j*)*|kl.*mn*
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
                Token::Quantifier('*'),
                Token::Rparen,
                Token::Quantifier('*'),
                Token::Selector,
                Token::Char('k'),
                Token::Char('l'),
                Token::Char('.'),
                Token::Quantifier('*'),
                Token::Char('m'),
                Token::Char('n'),
                Token::Quantifier('*'),
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
                (Token::Char('j'), Token::Quantifier('*')),
                (Token::Char('.'), Token::Quantifier('*')),
                (Token::Char('n'), Token::Quantifier('*')),
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

            //a, bc, def, ghi*, j*, kl.*mn*
            let mut concats = vec![
                vec![factor_map.remove("b").unwrap(), factor_map.remove("c").unwrap()],
                vec![factor_map.remove("d").unwrap(), factor_map.remove("e").unwrap(), factor_map.remove("f").unwrap()],
                vec![factor_map.remove("g").unwrap(), factor_map.remove("h").unwrap(), factor_map.remove("i*").unwrap()],
                vec![factor_map.remove("j*").unwrap()],
                vec![factor_map.remove("k").unwrap(), factor_map.remove("l").unwrap(), factor_map.remove(".*").unwrap(), factor_map.remove("m").unwrap(), factor_map.remove("n*").unwrap()],
            ].into_iter().map(|v| {
                create_concat(v)
            }).collect::<Vec<Concat>>();

            //(def|ghi*)
            let innermost = Alt{
                val: Some(
                    concats.remove(1), 
                ),
                tail: Some(Box::new(Alt { val: Some(concats.remove(1)), tail: None })),
            };

            // (def|ghi*)j*
            let right_alt_in_left_group = Alt{
                val: Some(
                    Concat{
                        val: Factor { val: Base::Alt(Box::new(innermost)), q: None },
                        tail: Some(Box::new(concats.remove(1)))
                    },
                ),
                tail: None
            };

            // (bc|(def|ghi*)j*)
            let left_group = Alt{
                val: Some(
                    concats.remove(0),
                ),
                tail: Some(Box::new(right_alt_in_left_group)),
            };

            // a(bc|(def|ghi*)j*)*|kl.*mn*
            let alt = Alt{
                val: Some(
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
                ),
                tail: Some(Box::new(Alt { val: Some(concats.remove(0)), tail: None })),
            };

            let expected = Regexp{val: alt};
            let actual = LL0Parser::new().parse(&tokens);
            assert_eq!(actual, Ok(expected));
        }
    }
}