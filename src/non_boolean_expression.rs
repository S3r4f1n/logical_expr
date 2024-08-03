use nom::{branch::alt, character::complete::multispace0, combinator::map, sequence::{delimited, tuple}, IResult};
use regex::Regex;

use crate::{operator::{binary_operator_number, binary_operator_string, BinaryOperator}, value::*};

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) struct NonBooleanExpression(pub(crate) Value, pub (crate) BinaryOperator, pub (crate) Value);
impl NonBooleanExpression {
  pub(crate) fn evaluate(&self) -> Result<bool, String> {
    if let Value::StringLiteral(_) = self.0 {
      self.eval_string()
    } else if let Value::IntegerLiteral(_) = self.0 {
      self.eval_integer()
    } else {
      self.eval_float()
    }
  }
  fn eval_string(&self) -> Result<bool, String> {
    if let NonBooleanExpression(Value::StringLiteral(lhs), op, Value::StringLiteral(rhs))  = &self{
      Ok(match op {
        BinaryOperator::Equals => lhs == rhs,
        BinaryOperator::NotEquals => lhs != rhs,
        BinaryOperator::RegexMatch => 
          Regex::new(&rhs).map_err(|_| format!("Invalid regex: {}", rhs))?.is_match(&lhs),
        _ => return Err(format!("Invalid binary operator for string: {:?}", op))
      })
    } else {
      Err(format!("Not a Binary String expression: {:?}", self))
    }
  }
  fn eval_integer(&self) -> Result<bool, String> {
    if let NonBooleanExpression(Value::IntegerLiteral(lhs), op, Value::IntegerLiteral(rhs)) = &self{

      Ok(match op {
        BinaryOperator::Equals => lhs == rhs,
        BinaryOperator::NotEquals => lhs != rhs,
        BinaryOperator::LessThan => lhs < rhs,
        BinaryOperator::GreaterThan => lhs > rhs,
        BinaryOperator::LessEqual => lhs <= rhs,
        BinaryOperator::GreaterEqual => lhs >= rhs,
        _ => return Err(format!("Invalid binary operator for number: {:?}", op))
      })
    } else {
      Err(format!("Not a Binary Integer expression: {:?}", self))
    }
  }
  fn eval_float(&self) -> Result<bool, String> {
    if let NonBooleanExpression(Value::FloatLiteral(lhs), op, Value::FloatLiteral(rhs)) = &self{
      
      Ok(match op {
        BinaryOperator::Equals => lhs == rhs,
        BinaryOperator::NotEquals => lhs != rhs,
        BinaryOperator::LessThan => lhs < rhs,
        BinaryOperator::GreaterThan => lhs > rhs,
        BinaryOperator::LessEqual => lhs <= rhs,
        BinaryOperator::GreaterEqual => lhs >= rhs,
        _ => return Err(format!("Invalid binary operator for number: {:?}", op))
      })
    } else {
      Err(format!("Not a Binary Integer expression: {:?}", self))
    }
  }
  
  pub(crate) fn use_context(self, context: &std::collections::HashMap<String, crate::ContextValue>) -> Result<Self, String> {
    Ok(NonBooleanExpression(self.0.use_context(context)?, self.1, self.2.use_context(context)?))
    }
}

pub(crate) fn binary_non_bool(input: &str) -> IResult<&str, NonBooleanExpression> {
  alt((
    map(tuple((integer, delimited(multispace0, binary_operator_number, multispace0), integer)), |(first, op, second)| NonBooleanExpression(first, op, second)),
    map(tuple((float, delimited(multispace0, binary_operator_number, multispace0), float)), |(first, op, second)| NonBooleanExpression(first, op, second)),
    map(tuple((string, delimited(multispace0, binary_operator_string, multispace0), string)), |(first, op, second)| NonBooleanExpression(first, op, second)),
  ))(input)
}


#[cfg(test)]
mod test_non_bool_expression {
  use super::*;
  
  #[test]
  fn parse_test() {
    let e = binary_non_bool("1 == 2").unwrap().1;
    assert_eq!(e, NonBooleanExpression(Value::IntegerLiteral(1), BinaryOperator::Equals, Value::IntegerLiteral(2)));
  }
  #[test]
  fn parse_test_1() {
    let e = binary_non_bool("1 == mode").unwrap().1;
    assert_eq!(e, NonBooleanExpression(Value::IntegerLiteral(1), BinaryOperator::Equals, Value::Identifier(Identifier::from("mode"))));
  }
  #[test]
  fn parse_test_2() {
    let e = binary_non_bool("valla == mode").unwrap().1;
    assert_eq!(e, NonBooleanExpression(Value::Identifier(Identifier::from("valla")), BinaryOperator::Equals, Value::Identifier(Identifier::from("mode"))));
  }
  #[test]
  fn parse_test_3() {
    let e = binary_non_bool("'valla' == mode").unwrap().1;
    assert_eq!(e, NonBooleanExpression(Value::StringLiteral("valla".to_string()), BinaryOperator::Equals, Value::Identifier(Identifier::from("mode"))));
  }
  #[test]
  fn parse_test_4() {
    let e = binary_non_bool("2.0 == mode").unwrap().1;
    assert_eq!(e, NonBooleanExpression(Value::FloatLiteral(2.0), BinaryOperator::Equals, Value::Identifier(Identifier::from("mode"))));
  }
  #[test]
  fn parse_test_error() {
    let e = binary_non_bool("2.0 == 1");
    assert!(e.is_err())
  }

  #[test]
  fn test_eval_string() {
    let e = NonBooleanExpression(Value::StringLiteral("test".to_string()), BinaryOperator::Equals, Value::StringLiteral("test".to_string()));
    assert_eq!(e.eval_string(), Ok(true));
    let e = NonBooleanExpression(Value::StringLiteral("test".to_string()), BinaryOperator::NotEquals, Value::StringLiteral("test".to_string()));
    assert_eq!(e.eval_string(), Ok(false));
    let e = NonBooleanExpression(Value::StringLiteral("test".to_string()), BinaryOperator::RegexMatch, Value::StringLiteral("t.*t".to_string()));
    assert_eq!(e.eval_string(), Ok(true));
    let e = NonBooleanExpression(Value::StringLiteral("test".to_string()), BinaryOperator::RegexMatch, Value::StringLiteral("t.t".to_string()));
    assert_eq!(e.eval_string(), Ok(false));
    let e = NonBooleanExpression(Value::StringLiteral("test".to_string()), BinaryOperator::Equals, Value::StringLiteral("nope".to_string()));
    assert_eq!(e.eval_string(), Ok(false));
    let e = NonBooleanExpression(Value::StringLiteral("test".to_string()), BinaryOperator::RegexMatch, Value::StringLiteral("t..t".to_string()));
    assert_eq!(e.eval_string(), Ok(true));
    let e = NonBooleanExpression(Value::StringLiteral("test".to_string()), BinaryOperator::LessEqual, Value::StringLiteral("t..t".to_string()));
    assert_eq!(e.eval_string(), Err("Invalid binary operator for string: LessEqual".to_string()));
  }

  #[test]
  fn test_eval_number() {
    let e = NonBooleanExpression(Value::IntegerLiteral(1), BinaryOperator::Equals, Value::IntegerLiteral(1));
    assert_eq!(e.eval_integer(), Ok(true));
    let e = NonBooleanExpression(Value::IntegerLiteral(1), BinaryOperator::NotEquals, Value::IntegerLiteral(1));
    assert_eq!(e.eval_integer(), Ok(false));
    let e = NonBooleanExpression(Value::IntegerLiteral(1), BinaryOperator::LessThan, Value::IntegerLiteral(2));
    assert_eq!(e.eval_integer(), Ok(true));
    let e = NonBooleanExpression(Value::IntegerLiteral(1), BinaryOperator::GreaterThan, Value::IntegerLiteral(2));
    assert_eq!(e.eval_integer(), Ok(false));
    let e = NonBooleanExpression(Value::IntegerLiteral(1), BinaryOperator::LessEqual, Value::IntegerLiteral(2));
    assert_eq!(e.eval_integer(), Ok(true));
    let e = NonBooleanExpression(Value::IntegerLiteral(1), BinaryOperator::GreaterEqual, Value::IntegerLiteral(2));
    assert_eq!(e.eval_integer(), Ok(false));
    let e = NonBooleanExpression(Value::IntegerLiteral(1), BinaryOperator::Equals, Value::IntegerLiteral(2));
    assert_eq!(e.eval_integer(), Ok(false));
    let e = NonBooleanExpression(Value::FloatLiteral(1.0), BinaryOperator::LessThan, Value::FloatLiteral(2.0));
    assert_eq!(e.eval_float(), Ok(true));
    let e = NonBooleanExpression(Value::FloatLiteral(1.0), BinaryOperator::GreaterThan, Value::FloatLiteral(2.0));
    assert_eq!(e.eval_float(), Ok(false));
    let e = NonBooleanExpression(Value::FloatLiteral(1.0), BinaryOperator::LessEqual, Value::FloatLiteral(2.0));
    assert_eq!(e.eval_float(), Ok(true));
    let e = NonBooleanExpression(Value::FloatLiteral(1.0), BinaryOperator::GreaterEqual, Value::FloatLiteral(2.0));
    assert_eq!(e.eval_float(), Ok(false));
  }
}
