use crate::ast::*;
use crate::eval::is_true;
use crate::value::{SExpression, Value};
use crate::SchemeError;
use std::ops::Neg;

pub fn apply_primitive(primop: &PrimitiveOperation, args: &[Value]) -> Result<Value, SchemeError> {
    match primop {
        PrimitiveOperation::Add => add(args),
        PrimitiveOperation::Sub => sub(args),
        PrimitiveOperation::Mul => mul(args),
        PrimitiveOperation::Div => div(args),
        PrimitiveOperation::LessThan => compare(args, |x, y| x < y),
        PrimitiveOperation::LessThanOrEqual => compare(args, |x, y| x <= y),
        PrimitiveOperation::GreaterThan => compare(args, |x, y| x > y),
        PrimitiveOperation::GreaterThanOrEqual => compare(args, |x, y| x >= y),
        PrimitiveOperation::Not => not(args),
        PrimitiveOperation::And => and(args),
        PrimitiveOperation::Or => or(args),
        PrimitiveOperation::IsEq => is_eq(args),
        PrimitiveOperation::Cons => cons(args),
        PrimitiveOperation::Car => car(args),
        PrimitiveOperation::Cdr => cdr(args),
        PrimitiveOperation::List => list(args),
        PrimitiveOperation::IsPair => is_pair(args),
        PrimitiveOperation::IsNumber => is_number(args),
        PrimitiveOperation::IsBoolean => is_boolean(args),
        PrimitiveOperation::IsSymbol => is_symbol(args),
    }
}

fn add(args: &[Value]) -> Result<Value, SchemeError> {
    args.iter()
        .fold(Ok(Value::Number(0.0)), |acc, cur| match (acc, cur) {
            (Ok(Value::Number(sum)), Value::Number(x)) => Ok(Value::Number(sum + x)),
            _ => Err(SchemeError::TypeMismatch(format!(
                "Value {:?} not a number",
                cur
            ))),
        })
}

fn sub(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [] => Err(SchemeError::ArgumentMismatch(
            "Expected at least 1 argument, got 0.".to_owned(),
        )),
        [Value::Number(x)] => Ok(Value::Number(x.neg())),
        [v] => Err(SchemeError::TypeMismatch(format!(
            "Value {:?} not a number",
            v
        ))),
        [v @ Value::Number(_), rest @ ..] => {
            rest.iter()
                .fold(Ok(v.clone()), |acc, cur| match (acc, cur) {
                    (Ok(Value::Number(diff)), Value::Number(x)) => Ok(Value::Number(diff - x)),
                    _ => Err(SchemeError::TypeMismatch(format!(
                        "Value {:?} not a number",
                        cur
                    ))),
                })
        }
        [v, ..] => Err(SchemeError::TypeMismatch(format!(
            "Value {:?} not a number",
            v
        ))),
    }
}

fn mul(args: &[Value]) -> Result<Value, SchemeError> {
    args.iter()
        .fold(Ok(Value::Number(1.0)), |acc, cur| match (acc, cur) {
            (Ok(Value::Number(prod)), Value::Number(x)) => Ok(Value::Number(prod * x)),
            _ => Err(SchemeError::TypeMismatch(format!(
                "Value {:?} not a number",
                cur
            ))),
        })
}

fn div(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [] => Err(SchemeError::ArgumentMismatch(
            "Expected at least 1 argument, got 0.".to_owned(),
        )),
        [Value::Number(x)] => Ok(Value::Number(1.0 / x)),
        [v] => Err(SchemeError::TypeMismatch(format!(
            "Value {:?} not a number",
            v
        ))),
        [v @ Value::Number(_), rest @ ..] => {
            rest.iter()
                .fold(Ok(v.clone()), |acc, cur| match (acc, cur) {
                    (Ok(Value::Number(frac)), Value::Number(x)) => Ok(Value::Number(frac / x)),
                    _ => Err(SchemeError::TypeMismatch(format!(
                        "Value {:?} not a number",
                        cur
                    ))),
                })
        }
        [v, ..] => Err(SchemeError::TypeMismatch(format!(
            "Value {:?} not a number",
            v
        ))),
    }
}

fn compare(args: &[Value], cmp_fn: fn(&f64, &f64) -> bool) -> Result<Value, SchemeError> {
    match args {
        [] => Err(SchemeError::ArgumentMismatch(
            "Expected at least 1 argument, got 0.".to_owned(),
        )),
        [Value::Number(_)] => Ok(Value::Boolean(true)),
        _ => args.windows(2).fold(Ok(Value::Boolean(true)), |acc, cur| {
            let lhs = &cur[0];
            let rhs = &cur[1];
            match (acc, lhs, rhs) {
                (Ok(Value::Boolean(b)), Value::Number(lhs), Value::Number(rhs)) => {
                    Ok(Value::Boolean(b && cmp_fn(lhs, rhs)))
                }
                _ => Err(SchemeError::TypeMismatch(format!(
                    "Value {:?} not a number",
                    cur
                ))),
            }
        }),
    }
}

fn not(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [v] => Ok(Value::Boolean(!is_true(v))),
        _ => Err(SchemeError::ArgumentMismatch(format!(
            "Expected 1 argument, got {}",
            args.len()
        ))),
    }
}

fn and(args: &[Value]) -> Result<Value, SchemeError> {
    let all_true = args.iter().all(is_true);
    if all_true {
        match args.last() {
            None => Ok(Value::Boolean(true)),
            Some(last) => Ok(last.clone()),
        }
    } else {
        Ok(Value::Boolean(false))
    }
}

fn or(args: &[Value]) -> Result<Value, SchemeError> {
    for v in args {
        if is_true(v) {
            return Ok(v.clone());
        }
    }
    Ok(Value::Boolean(false))
}

fn is_eq(args: &[Value]) -> Result<Value, SchemeError> {
    if args.len() != 2 {
        Err(SchemeError::ArgumentMismatch(format!(
            "Expected 2 arguments, got {}",
            args.len()
        )))
    } else {
        let arg1 = &args[0];
        let arg2 = &args[1];
        match (arg1, arg2) {
            (Value::Number(x), Value::Number(y)) => Ok(Value::Boolean(x == y)),
            (Value::Boolean(x), Value::Boolean(y)) => Ok(Value::Boolean(x == y)),
            (Value::SExpression(x), Value::SExpression(y)) => {
                if x.is_empty() && y.is_empty() {
                    Ok(Value::Boolean(true))
                } else {
                    Ok(Value::Boolean(false))
                }
            }
            _ => Ok(Value::Boolean(false)),
        }
    }
}

fn cons(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [v1, v2] => Ok(Value::SExpression(Box::from(SExpression::Compound(
            v1.clone(),
            v2.clone(),
        )))),
        _ => Err(SchemeError::ArgumentMismatch(format!(
            "Expected 2 arguments, got {}",
            args.len()
        ))),
    }
}

fn car(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [Value::SExpression(sexpr)] => match &**sexpr {
            SExpression::Compound(first, _) => Ok(first.clone()),
            _ => Err(SchemeError::TypeMismatch(format!(
                "Expected a compound s-expression, got {:?}",
                **sexpr
            ))),
        },
        [v] => Err(SchemeError::TypeMismatch(format!(
            "Expected a compound s-expression, got {:?}",
            v
        ))),
        _ => Err(SchemeError::ArgumentMismatch(format!(
            "Expected 2 arguments, got {}",
            args.len()
        ))),
    }
}

fn cdr(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [Value::SExpression(sexpr)] => match &**sexpr {
            SExpression::Compound(_, second) => Ok(second.clone()),
            _ => Err(SchemeError::TypeMismatch(format!(
                "Expected a compound s-expression, got {:?}",
                **sexpr
            ))),
        },
        [v] => Err(SchemeError::TypeMismatch(format!(
            "Expected a compound s-expression, got {:?}",
            v
        ))),
        _ => Err(SchemeError::ArgumentMismatch(format!(
            "Expected 2 arguments, got {}",
            args.len()
        ))),
    }
}

fn list(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [] => Ok(Value::SExpression(Box::from(SExpression::Nil))),
        _ => {
            let lst = args.iter().rev().fold(SExpression::Nil, |acc, cur| {
                SExpression::Compound(cur.clone(), Value::SExpression(Box::from(acc)))
            });
            Ok(Value::SExpression(Box::from(lst)))
        }
    }
}

fn is_pair(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [v] => match v {
            Value::SExpression(sexpr) => match **sexpr {
                SExpression::Compound(_, _) => Ok(Value::Boolean(true)),
                _ => Ok(Value::Boolean(false)),
            },
            _ => Ok(Value::Boolean(false)),
        },
        _ => Err(SchemeError::ArgumentMismatch(format!(
            "Expected 1 argument, got {}",
            args.len()
        ))),
    }
}

fn is_number(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [Value::Number(_)] => Ok(Value::Boolean(true)),
        [_] => Ok(Value::Boolean(false)),
        _ => Err(SchemeError::ArgumentMismatch(format!(
            "Expected 1 argument, got {}",
            args.len()
        ))),
    }
}

fn is_boolean(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [Value::Boolean(_)] => Ok(Value::Boolean(true)),
        [_] => Ok(Value::Boolean(false)),
        _ => Err(SchemeError::ArgumentMismatch(format!(
            "Expected 1 argument, got {}",
            args.len()
        ))),
    }
}

fn is_symbol(args: &[Value]) -> Result<Value, SchemeError> {
    match args {
        [v] => match v {
            Value::SExpression(sexpr) => match **sexpr {
                SExpression::Symbol(_) => Ok(Value::Boolean(true)),
                _ => Ok(Value::Boolean(false)),
            },
            _ => Ok(Value::Boolean(false)),
        },
        _ => Err(SchemeError::ArgumentMismatch(format!(
            "Expected 1 argument, got {}",
            args.len()
        ))),
    }
}
