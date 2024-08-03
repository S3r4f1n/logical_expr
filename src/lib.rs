use std::collections::HashMap;

mod operator;
mod expression;
mod value;
mod non_boolean_expression;

use expression::BooleanExpression;

// todo
// allow for a && b && c instead of (a && b) && c
// implement operator precedence
// improve float integer evaluation

/// # Introduction
/// This functions sits at the core of the library. It takes an expression as a string and returns a boolean.
/// 
/// # Examples
/// ```
/// use logical_expr::{Context, ContextValue, evaluate};
/// 
/// let mut context = Context::new();
/// context.insert("foo".to_string(), ContextValue::String("baaaar".to_string()));
/// context.insert("length".to_string(), ContextValue::Integer(1));
///
/// let result = evaluate("foo =~ 'ba+r' && 2 > length", &context);
/// assert_eq!(result, Ok(true));
/// ```
/// # Accepted Grammar of &str is:  
/// ```markdown
///  boolean_expression  
///     boolean_value || boolean_vale || .. || boolean_value  
///     boolean_value && boolean_vale && .. && boolean_value  
///     boolean_value  
///
///  boolean_value  
///     value operator value  
///     boolean   // true, false  
///     unary_operator boolean_value  
///     identifier  
///     (boolean_expression)  
/// 
///  value  
///    identifier // mode (accesses context)  
///    string     // 'normal'  
///    integer    // 5  
///    float      // 5.0  
///
///  operator   
///    ==         // string, integer, float  
///    !=         // string, integer, float  
///    <          // integer, float  
///    >          // integer, float  
///    <=         // integer, float  
///    >=         // integer, float  
///    &&         // boolean  
///    ||         // boolean  
///    =~         // string (regex)  
///   
///  unary_operator   
///    !          // boolean  
/// ```
///  

pub fn evaluate(expression: &str, context: &Context) -> Result<bool, String> {
    let expr = BooleanExpression::try_from(expression)?;
    let expr = expr.use_context(context)?;
    expr.evaluate()
}

/// This is a type alias for a hashmap of strings and context values
pub type Context = HashMap<String, ContextValue>;

/// This is an enum containing valid context value types.
pub enum ContextValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    #[test]
    fn regex_and_context() {
        let mut context = HashMap::new();
        context.insert("foo".to_string(), ContextValue::String("bbbbbaaaarrrrrr".to_string()));
        let result = evaluate("foo =~ 'ba+r'", &context).unwrap();
        assert!(result);
        
        let mut context = HashMap::new();
        context.insert("foo".to_string(), ContextValue::String("bbbbbaaaarrrrrr".to_string()));
        let result = evaluate("foo =~ '^ba+r$'", &context).unwrap();
        assert!(!result);

        let mut context = HashMap::new();
        context.insert("foo".to_string(), ContextValue::String("baaaar".to_string()));
        let result = evaluate("foo =~ '^ba+r$'", &context).unwrap();
        assert!(result);
    }
    #[test]
    fn not_boolean() {
        let mut context = HashMap::new();
        context.insert("foo".to_string(), ContextValue::Boolean(false));
        let result = evaluate("!foo", &context).unwrap();
        assert!(result);
    }
}

