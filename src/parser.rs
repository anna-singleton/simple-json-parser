use std::{collections::HashMap, iter::Peekable, mem::discriminant, slice::Iter};

use crate::{j_item::JItem, lexer::Token};

pub fn parse(tokens: Vec<Token>) -> Result<JItem, String> {
    let mut i = tokens.iter().peekable();
    let item = parse_jitem(&mut i)?;
    if i.peek().is_some() {
        return Err("Parsing finished with tokens left.".to_string());
    }
    return Ok(item);
}

fn parse_jitem(tokens: &mut Peekable<Iter<Token>>) -> Result<JItem, String> {
    let Some(next) = tokens.next() else {
        return Err("tried to parse JItem, but got EOF.".to_string());
    };
    return match next {
        Token::LBrace => parse_jobject(tokens),
        Token::LSquareBracket => parse_jarray(tokens),
        Token::Number(num) => Ok(JItem::Number(*num)),
        Token::String(s) => Ok(JItem::String(s.to_string())),
        Token::True => Ok(JItem::True),
        Token::False => Ok(JItem::False),
        Token::Null => Ok(JItem::Null),
        _ => Err(format!("Unexpected '{:?}' during parse.", next)),
    };
}

fn parse_jobject(tokens: &mut Peekable<Iter<Token>>) -> Result<JItem, String> {
    let mut hmap:HashMap<String, JItem> = HashMap::new();
    while let Some(next) = tokens.peek() {
        if **next == Token::RBrace {
            tokens.next();
            return Ok(JItem::Object(hmap));
        }

        let Token::String(key) = next else {
            return Err(format!("expected string key for jobject but got {:?}", next));
        };

        if hmap.contains_key(key) {
            return Err(format!("duplicate key found in jobject: '{}'", key));
        }

        tokens.next(); // advance and eat the key token

        expect_token(tokens, &Token::Colon)?; // there needs to be a : between key and item

        let inner_item = parse_jitem(tokens)?;

        hmap.insert(key.to_string(), inner_item);

        if dbg!(tokens.peek()).is_some_and(|t| **t == Token::RBrace) {
            tokens.next();
            return Ok(JItem::Object(hmap));
        }
        expect_token(tokens, &Token::Comma)?;
    }
    return Err("unexpected EOF during parse of array.".to_string());
}

fn parse_jarray(tokens: &mut Peekable<Iter<Token>>) -> Result<JItem, String> {
    let mut elements = vec![];
    while let Some(next) = tokens.peek() {
        if **next == Token::RSquareBracket {
            tokens.next();
            return Ok(JItem::Array(elements));
        }

        let inner_item = parse_jitem(tokens)?;

        dbg!(&inner_item);

        elements.push(inner_item);

        if dbg!(tokens.peek()).is_some_and(|t| **t == Token::RSquareBracket) {
            tokens.next();
            return Ok(JItem::Array(elements));
        }
        expect_token(tokens, &Token::Comma)?;
    }
    return Err("unexpected EOF during parse of array.".to_string());
}

fn expect_token(tokens: &mut Peekable<Iter<Token>>, expected: &Token) -> Result<(), String> {
    if let Some(tok) = tokens.next() {
        if discriminant(tok) == discriminant(expected) {
            return Ok(());
        }
        else {
            return Err(format!("Unexpected token during parse. Expected {:?} but got {:?}", expected, tok));
        }
    };
    return Err(format!("Unexpected EOF during parse. Expected {:?} but got EOF.", expected));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_single_true() {
        assert_successful_parse(vec![Token::True], JItem::True);
    }

    #[test]
    fn parse_single_false() {
        assert_successful_parse(vec![Token::False], JItem::False);
    }

    #[test]
    fn parse_single_null() {
        assert_successful_parse(vec![Token::Null], JItem::Null);
    }

    #[test]
    fn parse_array_empty() {
        assert_successful_parse(vec![Token::LSquareBracket, Token::RSquareBracket], JItem::Array(vec![]));
    }

    #[test]
    fn parse_array_single() {
        assert_successful_parse(vec![Token::LSquareBracket, Token::True, Token::RSquareBracket], JItem::Array(vec![JItem::True]));
    }

    #[test]
    fn parse_array_multi() {
        assert_successful_parse(
            vec![Token::LSquareBracket, Token::True, Token::Comma, Token::Number(5.), Token::Comma, Token::String("foo".to_string()), Token::RSquareBracket],
            JItem::Array(vec![JItem::True, JItem::Number(5.), JItem::String("foo".to_string())])
        );
    }

    #[test]
    fn parse_array_nested() {
        assert_successful_parse(
            vec![Token::LSquareBracket, Token::True, Token::Comma, Token::LSquareBracket, Token::Number(5.), Token::RSquareBracket, Token::RSquareBracket],
            JItem::Array(vec![JItem::True, JItem::Array(vec![JItem::Number(5.)])])
        );
    }

    #[test]
    fn parse_object_empty() {
        assert_successful_parse(
            vec![Token::LBrace, Token::RBrace],
            JItem::Object(HashMap::new())
        );
    }

    #[test]
    fn parse_object_single() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("foo".to_string(), JItem::String("bar".to_string()));
        assert_successful_parse(
            vec![Token::LBrace, Token::String("foo".to_string()), Token::Colon, Token::String("bar".to_string()), Token::RBrace],
            JItem::Object(expected_hashmap)
        );
    }

    #[test]
    fn parse_object_multi() {
        let mut expected_hashmap = HashMap::new();
        expected_hashmap.insert("foo".to_string(), JItem::String("bar".to_string()));
        expected_hashmap.insert("baz".to_string(), JItem::Number(10.));
        assert_successful_parse(
            vec![Token::LBrace, Token::String("foo".to_string()), Token::Colon, Token::String("bar".to_string()), Token::Comma, Token::String("baz".to_string()), Token::Colon, Token::Number(10.), Token::RBrace],
            JItem::Object(expected_hashmap)
        );
    }

    #[test]
    fn parse_object_nested() {
        let mut expected_hashmap = HashMap::new();
        let mut expected_nested_hashmap = HashMap::new();
        expected_nested_hashmap.insert("foo".to_string(), JItem::True);
        expected_hashmap.insert("foo".to_string(), JItem::String("bar".to_string()));
        expected_hashmap.insert("baz".to_string(), JItem::Object(expected_nested_hashmap));
        assert_successful_parse(
            vec![Token::LBrace, Token::String("foo".to_string()), Token::Colon, Token::String("bar".to_string()), Token::Comma, Token::String("baz".to_string()), Token::Colon, Token::LBrace, Token::String("foo".to_string()), Token::Colon, Token::True, Token::RBrace, Token::RBrace],
            JItem::Object(expected_hashmap)
        );
    }

    fn assert_successful_parse(input: Vec<Token>, output: JItem) {
        let result = parse(input);
        let Ok(output_tokens) = result else {
            panic!("parse returned Err: {}", result.unwrap_err());
        };
        assert_eq!(output, output_tokens);
    }

    fn assert_failed_parse(input: Vec<Token>, expected_error_message: &str) {
        let result = parse(input);
        let Err(error_message) = result else {
            panic!("parse returned Ok, but should have responded with an error.");
        };
        assert_eq!(error_message, expected_error_message);
    }
}
