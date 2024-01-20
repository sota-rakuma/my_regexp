/*
char: [a-zA-z0-9\.]
quantifier: [*, ?]
selector: [|]
*/

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Char(char),
    Quantifier(char),
    Selector,
}

fn get_token(raw_token: char) -> Token {
    if raw_token == '*' || raw_token == '?' {
        return Token::Quantifier(raw_token);
    } else if raw_token == '|' {
        return  Token::Selector;
    }
    Token::Char(raw_token)
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
    use crate::lexer::{Token, get_tokens};

    fn create_ans(s: &str) -> Vec<Token> {
        s.chars().map(|ch| {
            if ch == '*' || ch == '?' {
                return Token::Quantifier(ch);
            } else if ch == '|' {
                return  Token::Selector;
            }
            Token::Char(ch)
        }).collect()
    }

    #[test]
    fn valid_string_only_with_literal_get_tokens() {
        let raws = vec!["a", "abc", "abc 12"];


        for raw in raws {
            let expect = create_ans(raw);
            let actual = get_tokens(raw);
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn valid_string_with_quauntifier_get_tokens() {
        let raw = "a*a";
        let expect = create_ans(raw);
        let actual = get_tokens(raw);
        assert_eq!(expect, actual);

        let raw = "a*";
        let expect = create_ans(raw);
        let actual = get_tokens(raw);
        assert_eq!(expect, actual);

        let raw = "*a";
        let expect = create_ans(raw);
        let actual = get_tokens(raw);
        assert_eq!(expect, actual);
    }
}