use std::collections::HashMap;

use nom::{branch::alt, character::complete::{char, multispace0}, combinator::{map, map_res}, sequence::{delimited, tuple}, IResult};

use crate::{operator::{binary_and_operator, binary_or_operator, unary_operator_primary, BinaryOperator, UnaryOperator}, value::*, ContextValue, non_boolean_expression::{binary_non_bool, NonBooleanExpression}};

#[derive(Debug, PartialEq, PartialOrd)]
pub(crate) enum BooleanExpression {
  Identifier(Identifier),
  Boolean(bool),
  NonBooleanExpression(NonBooleanExpression),
  Binary(Box<BooleanExpression>, BinaryOperator, Box<BooleanExpression>),
  Unary(UnaryOperator, Box<BooleanExpression>),
}
impl TryFrom <&str> for BooleanExpression {
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    parse_whole_boolean_expression(value).map_err(|e| format!("{:?}", e))
  }
  type Error = String;
}
impl BooleanExpression {
  pub(crate) fn evaluate(&self) -> Result<bool, String> {
    match self {
      BooleanExpression::Boolean(b) => Ok(*b),
      BooleanExpression::Identifier(ident) => Err(format!("Context should be used before evaluation: {:?}", ident)),
      BooleanExpression::NonBooleanExpression(nbe) => nbe.evaluate(),
      BooleanExpression::Binary(lhs, op, rhs) => self.evaluate_binary(lhs, op, rhs),
      BooleanExpression::Unary(op, rhs) => self.evaluate_unary(op, rhs),
    }
  }
  fn evaluate_binary(&self, lhs: &BooleanExpression, op: &BinaryOperator, rhs: &BooleanExpression) -> Result<bool, String> {
    match op {
      BinaryOperator::And => Ok(lhs.evaluate()? && rhs.evaluate()?),
      BinaryOperator::Or => Ok(lhs.evaluate()? || rhs.evaluate()?),
      _ => Err(format!("Invalid binary operator for boolean: {:?}", op))
    }
  }
  fn evaluate_unary(&self, op: &UnaryOperator, rhs: &BooleanExpression) -> Result<bool, String> {
    match op {
      UnaryOperator::Not => Ok(!rhs.evaluate()?),
      _ => Err(format!("Invalid unary operator for boolean: {:?}", op))
    }
  }
  
  pub(crate) fn use_context(self, context: &HashMap<String, ContextValue>) -> Result<Self, String> {
    match self {
        BooleanExpression::Identifier(ident) => {
          if let Ok(Value::Boolean(b)) = ident.use_context(context) {
            Ok(BooleanExpression::Boolean(b))
          } else {
            Err(format!("Value should be a boolean: {:?}", ident))
          }},
        BooleanExpression::Boolean(_) => Ok(self),
        BooleanExpression::NonBooleanExpression(nbe) => Ok(BooleanExpression::NonBooleanExpression(nbe.use_context(context)?)), 
        BooleanExpression::Binary(lhs, op, rhs) => Ok(BooleanExpression::Binary(Box::new(lhs.use_context(context)?), op, Box::new(rhs.use_context(context)?))),
        BooleanExpression::Unary(op, value) => Ok(BooleanExpression::Unary(op, Box::new(value.use_context(context)?))),
    }
  }
}

fn boolean_value(input: &str) -> IResult<&str, BooleanExpression> {
  alt((
    map( binary_non_bool, |nbe| BooleanExpression::NonBooleanExpression(nbe)),
    delimited(tuple((char('('), multispace0)), boolean_expression, tuple((multispace0, char(')')))), 
    map_res( boolean, |b| {
        if let Value::Boolean(b) = b {
          Ok(BooleanExpression::Boolean(b))
        }else if let Value::Identifier(ident) = b {
          Ok(BooleanExpression::Identifier(ident))
        }else{
          Err(format!("Value should be a boolean: {:?}", b))
        }
      }
    ), 
    map(tuple((unary_operator_primary, multispace0, boolean_value)), 
    |(op, _, value)| BooleanExpression::Unary(op, Box::new(value))
    )
  ))(input)
}
fn boolean_expression(input: &str) -> IResult<&str, BooleanExpression> {
  alt((
    boolean_and,
    boolean_or,
    boolean_value,
  ))(input)
}
fn parse_whole_boolean_expression(input: &str) -> Result<BooleanExpression, String> {
  match boolean_expression(input) {
    Ok((remaining, parsed)) if remaining.is_empty() => Ok(parsed),
    Ok((remaining, _)) => Err(format!("Expected end of input, found: {:?}", remaining)),
    Err(err) => Err(format!("{:?}", err)),
  }
}

fn boolean_and(input: &str) -> IResult<&str, BooleanExpression> {
  alt((
    map(tuple((boolean_value, multispace0, binary_and_operator, multispace0, boolean_and)),
      |(lhs, _, op, _, rhs)| BooleanExpression::Binary(Box::new(lhs), op, Box::new(rhs))
    ),
    map(tuple((boolean_value, multispace0, binary_and_operator, multispace0, boolean_value)),
      |(lhs, _, op, _, rhs)| BooleanExpression::Binary(Box::new(lhs), op, Box::new(rhs))
    ),
  ))(input)
}

fn boolean_or(input: &str) -> IResult<&str, BooleanExpression> {
  alt((
    map(tuple((boolean_value, multispace0, binary_or_operator, multispace0, boolean_or)),
    |(lhs, _, op, _, rhs)| BooleanExpression::Binary(Box::new(lhs), op, Box::new(rhs))
    ),
    map(tuple((boolean_value, multispace0, binary_or_operator, multispace0, boolean_value)),
    |(lhs, _, op, _, rhs)| BooleanExpression::Binary(Box::new(lhs), op, Box::new(rhs))
    ),
  ))(input)
}


#[cfg(test)]
mod test {
  use super::*;
  use crate::{operator::{BinaryOperator, UnaryOperator}, value::{Value,Identifier}};

  #[test]
  fn test_boolean_value() {
    let value = "(true)";
    let result = boolean_value(value);
    assert_eq!(result.is_ok(), true);
    let (_, boolean_exp) = result.unwrap();
    assert_eq!(boolean_exp, BooleanExpression::Boolean(true));
  }
  #[test]
  fn test_boolean_value_2() {
    let value = "!  ( !   true)  ";
    let result = boolean_value(value);
    assert_eq!(result.is_ok(), true);
    let (_, boolean_exp) = result.unwrap();
    assert_eq!(boolean_exp, BooleanExpression::Unary(UnaryOperator::Not, Box::new(BooleanExpression::Unary(UnaryOperator::Not, Box::new(BooleanExpression::Boolean(true))))));
  }
  #[test]
  fn test_boolean_value_3() {
    let value = "4 == mode";
    let result = boolean_value(value);
    assert_eq!(result.is_ok(), true);
    let (_, boolean_exp) = result.unwrap();
    assert_eq!(boolean_exp, BooleanExpression::NonBooleanExpression(NonBooleanExpression(Value::IntegerLiteral(4), BinaryOperator::Equals, Value::Identifier(Identifier::from("mode")))));
  }
  #[test]
  fn test_boolean_value_err() {
    let value = "4 && mode";
    let result = boolean_value(value);
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn test_boolean_expression() {
    let value = "false || true || false";
    let result = boolean_expression(value);
    assert_eq!(result.is_ok(), true);
    let (_, boolean_exp) = result.unwrap();
    assert_eq!(boolean_exp, 
      BooleanExpression::Binary(
        Box::new(BooleanExpression::Boolean(false)),
        BinaryOperator::Or,
        Box::new(BooleanExpression::Binary(
          Box::new(BooleanExpression::Boolean(true)),
          BinaryOperator::Or,
          Box::new(BooleanExpression::Boolean(false))
        )),
      )
    );
  }

  #[test]
  fn test_boolean_and() {
    let value = "true && false";
    let result = boolean_and(value);
    assert_eq!(result.is_ok(), true);
    let (_, boolean_exp) = result.unwrap();
    assert_eq!(boolean_exp, 
      BooleanExpression::Binary(
        Box::new(BooleanExpression::Boolean(true)),
        BinaryOperator::And,
        Box::new(BooleanExpression::Boolean(false))
      )
    );
  }

  #[test]
  fn test_boolean_or() {
    let value = "true || false";
    let result = boolean_or(value);
    assert_eq!(result.is_ok(), true);
    let (_, boolean_exp) = result.unwrap();
    assert_eq!(boolean_exp, 
      BooleanExpression::Binary(
        Box::new(BooleanExpression::Boolean(true)),
        BinaryOperator::Or,
        Box::new(BooleanExpression::Boolean(false))
      )
    );
  }

  #[test]
  fn test_boolean_value_error() {
    let value = "identifier < true";
    let result = parse_whole_boolean_expression(value);
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn test_boolean_expression_identifier() {
    let value = "(identifier)";
    let result = boolean_expression(value);
    assert_eq!(result.is_ok(), true);
  }

  #[test]
  fn test_boolean_and_or_mix_error() {
    let value = "identifier && identifier || identifier";
    let result = parse_whole_boolean_expression(value);
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn test_boolean_and_or_mix() {
    let value = "identifier && (identifier || identifier)";
    let result = parse_whole_boolean_expression(value);
    assert_eq!(result.is_ok(), true);
  }
}
