use std::collections::BTreeMap;

use nom::branch::alt;
use nom::bytes::complete::{is_not, take_while};
use nom::character::complete::char;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{combinator::map, error::Error, number::complete::float, IResult};

use crate::json::JsonItem;

pub fn json_parser(input: &str) -> IResult<&str, JsonItem> {
    alt((number_parser, string_parser, array_parser, object_parser))(input)
}

fn number_parser(input: &str) -> IResult<&str, JsonItem> {
    map(float::<&str, Error<&str>>, |x| (x as f64).into())(input)
}

#[test]
fn test_number_lexer() {
    assert!(number_parser("1").is_ok());
    assert!(number_parser("-1").is_ok());
    assert!(number_parser("1.2").is_ok());
    assert!(number_parser("-1.2").is_ok());
    assert!(number_parser("abc").is_err());
}

fn string_parser(input: &str) -> IResult<&str, JsonItem> {
    map(
        tuple((char::<&str, Error<&str>>('"'), is_not("\""), char('"'))),
        |(_, string, _)| string.into(),
    )(input)
}

#[test]
fn test_string_lexer() {
    assert!(string_parser("\"Hello, world!\"").is_ok());
    assert!(string_parser("Hello, world!").is_err());
}

fn array_parser(input: &str) -> IResult<&str, JsonItem> {
    let first_lexer = json_parser;
    let follows_lexer = many0(map(
        tuple((char(','), whitespace0, json_parser, whitespace0)),
        |(_, _, json, _)| json,
    ));
    alt((
        map(tuple((char('['), whitespace0, char(']'))), |(_, _, _)| {
            vec![].into()
        }),
        map(
            tuple((
                char('['),
                whitespace0,
                first_lexer,
                whitespace0,
                follows_lexer,
                char(']'),
            )),
            |(_, _, first, _, mut follows, _)| {
                let mut v = vec![first];
                v.append(&mut follows);
                v.into()
            },
        ),
    ))(input)
}

#[test]
fn test_array_lexer() {
    assert!(
        array_parser("[1, 2, 3, 4, \"Hello, world!\", [1, 2, 3, 4, \"Hello, world!\"]]").is_ok()
    );
    assert!(array_parser("[]").is_ok());
    assert!(array_parser("1, 2, 3").is_err())
}

fn object_parser(input: &str) -> IResult<&str, JsonItem> {
    fn pair_lexer(input: &str) -> IResult<&str, (&str, JsonItem)> {
        map(
            tuple((
                char('"'),
                is_not("\""),
                char('"'),
                whitespace0,
                char(':'),
                whitespace0,
                json_parser,
            )),
            |(_, key, _, _, _, _, value)| (key, value),
        )(input)
    }
    let first_lexer = pair_lexer;
    let follows_lexer = many0(map(
        tuple((char(','), whitespace0, pair_lexer, whitespace0)),
        |(_, _, pair, _)| pair,
    ));
    alt((
        map(tuple((char('{'), whitespace0, char('}'))), |_| {
            BTreeMap::new().into()
        }),
        map(
            tuple((
                char('{'),
                whitespace0,
                first_lexer,
                whitespace0,
                follows_lexer,
                whitespace0,
                char('}'),
            )),
            |(_, _, first, _, follows, _, _)| {
                let mut bmap = follows
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v))
                    .collect::<BTreeMap<String, JsonItem>>();
                let (k, v) = first;
                bmap.insert(k.to_string(), v);
                bmap.into()
            },
        ),
    ))(input)
}

#[test]
fn test_object_lexer() {
    assert!(
        object_parser("{ \"key0\": 1, \"key1\": \"Hello, world!\", \"key2\": [1, 2, 3], \"key4\": { \"key5\": \"Hello, world!\" } }").is_ok()
    );
}

fn whitespace0(input: &str) -> IResult<&str, &str> {
    take_while(|c: char| c.is_whitespace())(input)
}
