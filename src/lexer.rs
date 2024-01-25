/*
char: [a-zA-z0-9\s\.]
quantifier: [*, ?]
selector: [|]
*/

pub type Char = char;
pub type Quantifier = char;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Token {
    Char(char),
    Quantifier(char),
    Selector,
    Lparen,
    Rparen,
}

impl Token {
    pub fn to_char(&self) -> char {
        match *self {
            Token::Char(c) | Token::Quantifier(c) => c,
            Token::Selector => '|',
            Token::Lparen => '(',
            Token::Rparen => ')',
        }
    }
}

fn get_token(raw_token: char) -> Token {
    match raw_token {
        c if c == '*' || c == '?' => Token::Quantifier(c),
        c if c == '|' => Token::Selector,
        c if c == '(' => Token::Lparen,
        c if c == ')' => Token::Rparen,
        c => Token::Char(c)
    }
}

pub fn get_tokens(raw: &str) -> Vec<Token> {
    raw.chars()
    .map(|ch| {
        get_token(ch)
    })
    .collect()
}

#[cfg(test)]
mod test {
    use crate::lexer::{get_tokens, Token};

    #[test]
    fn valid_string_only_with_literal_get_tokens() {
        let raw = "a";
        let expect = vec![Token::Char('a')];
        let actual = get_tokens(raw);
        assert_eq!(expect, actual);

        let raw = "ab";
        let expect = vec![Token::Char('a'), Token::Char('b')];
        let actual = get_tokens(raw);
        assert_eq!(expect, actual);

        let raw = "a*a";
        let expect = vec![Token::Char('a'), Token::Quantifier('*'), Token::Char('a')];
        let actual = get_tokens(raw);
        assert_eq!(expect, actual);

        let raw = "a*";
        let expect = vec![Token::Char('a'), Token::Quantifier('*')];
        let actual = get_tokens(raw);
        assert_eq!(expect, actual);

        let raw = "*a";
        let expect = vec![Token::Quantifier('*'), Token::Char('a')];
        let actual = get_tokens(raw);
        assert_eq!(expect, actual);

        let raw = "(ab|c )*d?";
        let expect = vec![
            Token::Lparen,
            Token::Char('a'),
            Token::Char('b'),
            Token::Selector,
            Token::Char('c'),
            Token::Char(' '),
            Token::Rparen,
            Token::Quantifier('*'), 
            Token::Char('d'),
            Token::Quantifier('?')
        ];
        let actual = get_tokens(raw);
        assert_eq!(expect, actual);
    }

}