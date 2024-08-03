use nom::{branch::alt, bytes::complete::{tag, take_while1}, character::complete::char, combinator::{map, map_res}, sequence::{delimited, tuple}, IResult};

use crate::ContextValue;


#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub(crate) enum Value{
  Identifier(Identifier),
  StringLiteral(String),
  IntegerLiteral(i64),
  FloatLiteral(f64),
  Boolean(bool),
}
impl Value {
    pub(crate) fn use_context(self, context: &std::collections::HashMap<String, ContextValue>) -> Result<Value, String> {
        match self {
            Value::Identifier(identifier) => identifier.use_context(context),
            _ => Ok(self),
        }
    }
}

impl From<&ContextValue> for Value {
    fn from(value: &ContextValue) -> Self {
        match value {
            ContextValue::String(s) => Value::StringLiteral(s.to_owned()),
            ContextValue::Integer(i) => Value::IntegerLiteral(*i),
            ContextValue::Float(f) => Value::FloatLiteral(*f),
            ContextValue::Boolean(b) => Value::Boolean(*b),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub(crate) struct Identifier(String);
impl Identifier {
    pub(crate) fn use_context(&self, context: &std::collections::HashMap<String, ContextValue>) -> Result<Value, String> {
        if let Some(val) = context.get(&self.0) {
            Ok(val.into())
        } else {
            Err(format!("Identifier not found in context: {}", &self.0))
        }
    }
}
impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier(value.to_string())
    }
}
impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier(value)
    }
}


pub(crate) fn integer(input: &str) -> IResult<&str, Value> {
    alt((map(take_while1(|c: char| c.is_ascii_digit()), |s: &str| Value::IntegerLiteral(s.parse::<i64>().unwrap())),
    identifier))(input)
}

pub(crate) fn float(input: &str) -> IResult<&str, Value> {
    alt((map(
        tuple((
            take_while1(|c: char| c.is_ascii_digit()),
            char('.'),
            take_while1(|c: char| c.is_ascii_digit()),
        )),
        |(int, _, frac)| {
            Value::FloatLiteral(format!("{}.{}", int, frac).parse::<f64>().unwrap())
        },
    ), identifier))(input)
}

pub(crate) fn string(input: &str) -> IResult<&str, Value> {
    alt((map(delimited(char('\''), take_while1(|c: char| c != '\''), char('\'')), |s: &str| Value::StringLiteral(s.to_string())), identifier))(input)
}

pub(crate) fn identifier(input: &str) -> IResult<&str, Value> {
    map_res(take_while1(|c: char| c.is_ascii_alphabetic() || c == '.' || c == '_'), |s: &str| {
        if s == "true" || s == "false" {
            return Err(format!("Identifier should not be true or false: {}", s))
        } 
        Ok(Value::Identifier(Identifier(s.to_string())))
    })(input)
}

pub(crate) fn boolean(input: &str) -> IResult<&str, Value> {
    alt((map(alt((tag("true"), tag("false"))), |c: &str| Value::Boolean(c == "true")), identifier))(input)
}

#[test]
fn test_value() {
    // Test conversion of various values to the corresponding enum value.
    assert_eq!(identifier("foo").unwrap().1, Value::Identifier(Identifier("foo".to_string())));
    assert_eq!(string("'foo'").unwrap().1, Value::StringLiteral("foo".to_string()));
    assert_eq!(integer("1").unwrap().1, Value::IntegerLiteral(1));
    assert_eq!(float("1.0").unwrap().1, Value::FloatLiteral(1.0));
    assert_eq!(boolean("true").unwrap().1, Value::Boolean(true));
    assert_eq!(boolean("false").unwrap().1, Value::Boolean(false));
}