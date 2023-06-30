use std::{collections::BTreeMap, fmt::Display};

#[derive(Debug, PartialEq)]
pub enum JsonItem {
    JsonNumber(f64),
    JsonString(String),
    JsonArray(Vec<JsonItem>),
    JsonObject(BTreeMap<String, JsonItem>),
}

impl From<f64> for JsonItem {
    fn from(value: f64) -> Self {
        JsonItem::JsonNumber(value as f64)
    }
}

impl From<&str> for JsonItem {
    fn from(value: &str) -> Self {
        JsonItem::JsonString(value.to_string())
    }
}

impl From<String> for JsonItem {
    fn from(value: String) -> Self {
        JsonItem::JsonString(value)
    }
}

impl From<Vec<JsonItem>> for JsonItem {
    fn from(value: Vec<JsonItem>) -> Self {
        JsonItem::JsonArray(value)
    }
}

impl From<BTreeMap<String, JsonItem>> for JsonItem {
    fn from(value: BTreeMap<String, JsonItem>) -> Self {
        JsonItem::JsonObject(value)
    }
}

impl Display for JsonItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonItem::JsonNumber(num) => write!(f, "{}", num),
            JsonItem::JsonString(string) => write!(f, "\"{}\"", string),
            JsonItem::JsonArray(arr) => write!(
                f,
                "[{}]",
                arr.iter()
                    .map(|json| json.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            JsonItem::JsonObject(obj) => {
                write!(
                    f,
                    "{{{}}}",
                    obj.iter()
                        .map(|(key, value)| format!("{}:{}", key, value.to_string()))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
        }
    }
}

#[test]
fn test_json() {
    let num: JsonItem = 1.0.into();
    assert_eq!(num.to_string(), "1");
    let string: JsonItem = "Hello, world!".into();
    assert_eq!(string.to_string(), "\"Hello, world!\"");
    let arr: JsonItem = vec![
        JsonItem::JsonNumber(1.0),
        JsonItem::JsonString("Hello, world!".to_string()),
    ]
    .into();
    assert_eq!(arr.to_string(), "[1,\"Hello, world!\"]");
    let mut map = BTreeMap::new();
    map.insert("key0".to_string(), JsonItem::JsonNumber(1.0));
    map.insert(
        "key1".to_string(),
        JsonItem::JsonString("Hello, world!".to_string()),
    );
    let map: JsonItem = map.into();
    assert_eq!(map.to_string(), "{key0:1,key1:\"Hello, world!\"}");
    let num0: JsonItem = 1.0.into();
    let num1: JsonItem = 1.0.into();
    assert_eq!(num0, num1);
    let str0: JsonItem = "Hello, world!".into();
    let str1: JsonItem = "Hello, world!".into();
    assert_eq!(str0, str1);
}
