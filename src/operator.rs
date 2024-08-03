
use nom::{
    branch::alt, bytes::complete::tag, combinator::{map, map_res}, IResult
};



#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum BinaryOperator {
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    And,
    Or,
    RegexMatch,
}


impl TryFrom<&str> for BinaryOperator {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "==" => Ok(BinaryOperator::Equals),
            "!=" => Ok(BinaryOperator::NotEquals),
            "<" => Ok(BinaryOperator::LessThan),
            ">" => Ok(BinaryOperator::GreaterThan),
            "<=" => Ok(BinaryOperator::LessEqual),
            ">=" => Ok(BinaryOperator::GreaterEqual),
            "&&" => Ok(BinaryOperator::And),
            "||" => Ok(BinaryOperator::Or),
            "=~" => Ok(BinaryOperator::RegexMatch),
            _ => Err(format!("Unknown operator: {}", value)),
        }
    }
    type Error = String;
}

pub(crate) fn binary_operator_number(input: &str) -> IResult<&str, BinaryOperator> {
    map_res(alt((tag("=="), tag("!="), tag("<"), tag(">"), tag("<="), tag(">="))), BinaryOperator::try_from)(input)
}

pub(crate) fn binary_operator_string(input: &str) -> IResult<&str, BinaryOperator> {
    map_res(alt((tag("=="), tag("!="), tag("=~"))), BinaryOperator::try_from)(input)
}
pub(crate) fn binary_and_operator(input: &str) -> IResult<&str, BinaryOperator> {
    map(tag("&&"), |_| BinaryOperator::And)(input)
}
pub(crate) fn binary_or_operator(input: &str) -> IResult<&str, BinaryOperator> {
    map(tag("||"), |_| BinaryOperator::Or)(input)
}

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum UnaryOperator {
    Not
}

impl TryFrom<&str> for UnaryOperator {
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "!" => Ok(UnaryOperator::Not),
            _ => Err(format!("Unknown operator: {}", value)),
        }
    }
    type Error = String;
}


pub(crate) fn unary_operator_primary(input: &str) -> IResult<&str, UnaryOperator> {
    map_res(tag("!"), UnaryOperator::try_from)(input)
}


#[test]
fn test_operator() {
    // Test conversion of various operators to the corresponding enum value.
    let tests = [
        ("==", BinaryOperator::Equals),
        ("!=", BinaryOperator::NotEquals),
        ("<", BinaryOperator::LessThan),
        (">", BinaryOperator::GreaterThan),
        ("<=", BinaryOperator::LessEqual),
        (">=", BinaryOperator::GreaterEqual),
        ("&&", BinaryOperator::And),
        ("||", BinaryOperator::Or),
        ("=~", BinaryOperator::RegexMatch),
    ];

    for (input, expected) in tests.iter() {
        assert_eq!(BinaryOperator::try_from(*input).unwrap(), *expected);
    }
    
    assert_eq!(BinaryOperator::try_from("invalid").unwrap_err(), "Unknown operator: invalid");
}

#[test]
fn test_operator_primary() {
  assert_eq!(
    binary_operator_number("=="),
    Ok(("", BinaryOperator::Equals))
  );
  assert_eq!(
    binary_operator_number(">"),
    Ok(("", BinaryOperator::GreaterThan))
  )
}

