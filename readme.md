# Introduction

logical_expr is a simple lib which allows for evaluation of logical expression.  
As input it takes a String and as an Output you get a bool.  
You can use a context containing variables which will be used during evaluation.
All you need to do is call the evaluate() function and if you want to use a context setup a context.

# Example

```rust
use logical_expr::{Context, ContextValue, evaluate};

let mut context = Context::new();
context.insert("foo".to_string(), ContextValue::String("baaaar".to_string()));
context.insert("length".to_string(), ContextValue::Integer(1));

let result = evaluate("foo =~ 'ba+r' && 2 > length", &context);
assert_eq!(result, Ok(true));
```

# Accepted Grammar of &str is:

```markdown
boolean_expression  
 boolean_value || boolean_vale || .. || boolean_value  
 boolean_value && boolean_vale && .. && boolean_value  
 boolean_value

boolean_value  
 value operator value  
 boolean // true, false  
 unary_operator boolean_value  
 identifier  
 (boolean_expression)

value  
 identifier // mode (accesses context)  
 string // 'normal'  
 integer // 5  
 float // 5.0

operator  
 == // string, integer, float  
 != // string, integer, float  
 < // integer, float
 \> // integer, float
 <= // integer, float  
 = // integer, float  
 && // boolean  
 || // boolean  
 =~ // string (regex)

unary_operator  
 ! // boolean
```
