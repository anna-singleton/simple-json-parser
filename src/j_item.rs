use std::{collections::HashMap, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub enum JItem {
    Object(HashMap<String, JItem>),
    String(String),
    Array(Vec<JItem>),
    Number(i64),
    True,
    False,
    Null,
}

impl Display for JItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted = match self {
            JItem::Object(hash_map) => &fmt_j_object(hash_map),
            JItem::Array(jitems) => &fmt_j_array(jitems),
            JItem::String(s) => &format!("\"{}\"", s),
            JItem::Number(x) => &format!("{}", x),
            JItem::True => "true",
            JItem::False => "false",
            JItem::Null => "null",
        };

        write!(f, "{}", formatted)
    }
}

fn fmt_j_array(arr: &Vec<JItem>) -> String {
    let formatted_items:Vec<_> = arr
        .iter()
        .map(|i| format!("{}", i))
        .collect();
    format!("[{}]", formatted_items.join(","))
}

fn fmt_j_object(hmap: &HashMap<String, JItem>) -> String {
    let formatted_items:Vec<_> = hmap
        .iter()
        .map(|(k, v)| format!("\"{}\":{}", k, v))
        .collect();
    format!("{{{}}}", formatted_items.join(","))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn jitem_fmt_null() {
        let formatted = format!("{}", JItem::Null);
        assert_eq!(formatted, "null");
    }

    #[test]
    fn jitem_fmt_true() {
        let formatted = format!("{}", JItem::True);
        assert_eq!(formatted, "true");
    }

    #[test]
    fn jitem_fmt_false() {
        let formatted = format!("{}", JItem::False);
        assert_eq!(formatted, "false");
    }

    #[test]
    fn jitem_fmt_number() {
        let formatted = format!("{}", JItem::Number(10));
        assert_eq!(formatted, "10");
    }

    #[test]
    fn jitem_fmt_number_negative() {
        let formatted = format!("{}", JItem::Number(-10));
        assert_eq!(formatted, "-10");
    }

    #[test]
    fn jitem_fmt_str() {
        let formatted = format!("{}", JItem::String("teststring".to_string()));
        assert_eq!(formatted, "\"teststring\"");
    }

    #[test]
    fn jitem_fmt_empty_str() {
        let formatted = format!("{}", JItem::String("".to_string()));
        assert_eq!(formatted, "\"\"");
    }

    #[test]
    fn jitem_fmt_empty_list() {
        let formatted = format!("{}", JItem::Array(vec![]));
        assert_eq!(formatted, "[]");
    }

    #[test]
    fn jitem_fmt_homogenous_list() {
        let formatted = format!("{}", JItem::Array(vec![JItem::Number(10), JItem::Number(5), JItem::Number(-100)]));
        assert_eq!(formatted, "[10,5,-100]");
    }

    #[test]
    fn jitem_fmt_non_homogenous_list() {
        let formatted = format!("{}", JItem::Array(vec![JItem::Number(10), JItem::String("foobar".to_string()), JItem::True, JItem::False, JItem::Null]));
        assert_eq!(formatted, "[10,\"foobar\",true,false,null]");
    }

    #[test]
    fn jitem_fmt_empty_object() {
        let hmap = HashMap::new();
        let formatted = format!("{}", JItem::Object(hmap));
        assert_eq!(formatted, "{}");
    }

    #[test]
    fn jitem_fmt_object() {
        let mut hmap = HashMap::new();
        hmap.insert("one".to_string(), JItem::True);
        let formatted = format!("{}", JItem::Object(hmap));
        assert_eq!(formatted, r#"{"one":true}"#);
    }
}
