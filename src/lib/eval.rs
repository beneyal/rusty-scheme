use crate::ast::*;
use crate::env::Environment;
use crate::value::Value;
use crate::SchemeError;

fn applicative_eval(cexp: &ConstituentExpression, env: &Environment) -> Result<Value, SchemeError> {
    match cexp {
        ConstituentExpression::Number(n) => Ok(Value::Number(n.0)),
        ConstituentExpression::Boolean(b) => Ok(Value::Boolean(b.0)),
        ConstituentExpression::Str(s) => Ok(Value::Str(s.0.to_owned())),
        ConstituentExpression::VariableReference(varref) => env.apply(&varref.0),
        ConstituentExpression::PrimitiveOperation(primop) => {
            Ok(Value::PrimitiveOperation(primop.0.to_owned()))
        }
        ConstituentExpression::If(ifexp) => eval_if(ifexp, env),
        ConstituentExpression::Procedure(proc) => eval_procedure(proc, env),
        ConstituentExpression::Let(letexp) => eval_let(letexp, env),
        ConstituentExpression::Applic(applic) => eval_applic(applic, env),
    }
}

fn eval_if(ifexp: &If, env: &Environment) -> Result<Value, SchemeError> {
    todo!()
}

fn eval_procedure(proc: &Procedure, env: &Environment) -> Result<Value, SchemeError> {
    todo!()
}

fn eval_let(letexp: &Let, env: &Environment) -> Result<Value, SchemeError> {
    todo!()
}

fn eval_applic(applic: &Application, env: &Environment) -> Result<Value, SchemeError> {
    todo!()
}
