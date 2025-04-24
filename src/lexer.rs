use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq)]
pub enum Token {
    LBrace,
    RBrace,

    LSquareBracket,
    RSquareBracket,

    Colon,
    Comma,

    Number(f64),
    String(String),

    True,
    False,
    Null
}

pub fn lex(s: &str) -> Result<Vec<Token>, String> {
    let mut i = s.chars().peekable();
    let mut tokens = vec![];
    while let Some(c) = i.next() {
        let token = match c {
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            '[' => Ok(Token::LSquareBracket),
            ']' => Ok(Token::RSquareBracket),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            '-' => lex_number(&mut i, c),
            '"' => lex_string(&mut i),
            'a'..='z' | 'A'..='Z' => lex_ident(&mut i, c),
            '0'..='9' => lex_number(&mut i, c),
            ' ' | '\n' | '\t' | '\r' => continue,
            _ => Err(format!("Unknown symbol '{}'", c)),
        }?;

        tokens.push(token);
    }
    return Ok(tokens);
}

fn lex_string(i: &mut Peekable<Chars>) -> Result<Token, String> {
    // we have consumed the first ", now consume characters until eof or "
    let mut escaped = false;
    let mut built_string = String::new();
    while let Some(c) = i.next() {
        if escaped {
            escaped = false;
            built_string.push(c);
            continue;
        }
        match c {
            '\\' => escaped = true,
            '"' => return Ok(Token::String(built_string)),
            _ => built_string.push(c),
        }
    }
    return Err("unterminated string literal. reached EOF.".to_string());
}

fn lex_ident(i: &mut Peekable<Chars>, c: char) -> Result<Token, String> {
    let mut built_string = String::new();
    built_string.push(c);
    while let Some(c) = i.peek() {
        match c {
            'a'..='z' | 'A'..='Z' => built_string.push(*c),
            _ => break,
        }
        i.next();
    }
    return match built_string.as_str() {
        "true" => Ok(Token::True),
        "false" => Ok(Token::False),
        "null" => Ok(Token::Null),
        _ => Err(format!("unknown keyword '{}'", built_string)),
    };
}

fn lex_number(i: &mut Peekable<Chars>, c: char) -> Result<Token, String> {
    let mut built_string = String::new();
    built_string.push(c);
    let mut has_decimal = false;
    while let Some(c) = i.peek() {
        if c.is_digit(10) {
            built_string.push(*c);
        }
        else if *c == '.' {
            if has_decimal {
                return Err("multiple '.' found in number literal.".to_string());
            }
            else {
                built_string.push(*c);
                has_decimal = true;
            }
        }
        else {
            break;
        }
        i.next();
    }
    return Ok(Token::Number(built_string.parse().unwrap()))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_object() {
        let input = r#"{"foo": 123}"#;
        let tokens = lex(input);
        let expected_tokens = vec![
            Token::LBrace,
            Token::String("foo".to_string()),
            Token::Colon,
            Token::Number(123.0),
            Token::RBrace,
        ];
        assert!(tokens.is_ok(), "encountered error: {}", tokens.unwrap_err());
        assert_eq!(tokens.unwrap(), expected_tokens);
    }

    #[test]
    fn simple_arr() {
        let input = r#"[123, "foobar", true, null, false]"#;
        let tokens = lex(input);
        let expected_tokens = vec![
            Token::LSquareBracket,
            Token::Number(123.0),
            Token::Comma,
            Token::String("foobar".to_string()),
            Token::Comma,
            Token::True,
            Token::Comma,
            Token::Null,
            Token::Comma,
            Token::False,
            Token::RSquareBracket,
        ];
        assert!(tokens.is_ok(), "encountered error: {}", tokens.unwrap_err());
        assert_eq!(tokens.unwrap(), expected_tokens);
    }

    #[test]
    fn decimal() {
        let input = r#"[123.45]"#;
        let tokens = lex(input);
        let expected_tokens = vec![
            Token::LSquareBracket,
            Token::Number(123.45),
            Token::RSquareBracket,
        ];
        assert!(tokens.is_ok(), "encountered error: {}", tokens.unwrap_err());
        assert_eq!(tokens.unwrap(), expected_tokens);
    }

    #[test]
    fn illegal_ident() {
        let input = r#"[notarealident]"#;
        let tokens = lex(input);
        assert!(tokens.is_err());
        assert_eq!("unknown keyword 'notarealident'", tokens.unwrap_err());
    }
}
