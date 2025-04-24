use j_item::JItem;
use lexer::lex;

pub mod j_item;
pub mod lexer;
pub mod parser;

pub fn parse(input_string: &str) -> Result<JItem, String> {
    let tokens = lex(input_string)?;
    return parser::parse(tokens);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_simple_list() {
        let input = r#"[true, false, null, "foobar", -10.5, ["no thanks"]]"#;
        let expected = JItem::Array(vec![JItem::True, JItem::False, JItem::Null, JItem::String("foobar".to_string()), JItem::Number(-10.5), JItem::Array(vec![JItem::String("no thanks".to_string())])]);
        let result = parse(input);
        let Ok(output) = result else {
            panic!("failure during parsing. failure: '{}'", result.unwrap_err());
        };
        assert_eq!(expected, output);
    }
}
