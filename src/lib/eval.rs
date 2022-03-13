use crate::ast::*;
use crate::env::Environment;
use crate::value::{Closure, Value};
use crate::{primitives, substitution, SchemeError};

pub fn eval_program(program: &Program) -> Result<Value, SchemeError> {
    eval_sequence(program.exps.as_slice(), &Environment::Empty)
}

pub fn applicative_eval(
    cexp: &ConstituentExpression,
    env: &Environment,
) -> Result<Value, SchemeError> {
    match cexp {
        ConstituentExpression::Number(n) => Ok(Value::Number(n.0)),
        ConstituentExpression::Boolean(b) => Ok(Value::Boolean(b.0)),
        ConstituentExpression::Literal(sexpr) => Ok(Value::SExpression(Box::from(sexpr.clone()))),
        ConstituentExpression::VariableReference(varref) => match env.apply(&varref.0) {
            Ok(v) => Ok(v.clone()),
            Err(e) => Err(e),
        },
        ConstituentExpression::PrimitiveOperation(primop) => {
            Ok(Value::PrimitiveOperation(primop.clone()))
        }
        ConstituentExpression::If(ifexp) => eval_if(ifexp, env),
        ConstituentExpression::Procedure(proc) => eval_procedure(proc, env),
        ConstituentExpression::Let(letexp) => eval_let(letexp, env),
        ConstituentExpression::Applic(applic) => eval_applic(applic, env),
    }
}

pub(crate) fn is_true(v: &Value) -> bool {
    !matches!(v, Value::Boolean(false))
}

fn eval_if(ifexp: &If, env: &Environment) -> Result<Value, SchemeError> {
    if is_true(&applicative_eval(&ifexp.cond, env)?) {
        applicative_eval(&ifexp.then, env)
    } else {
        applicative_eval(&ifexp.alt, env)
    }
}

fn eval_procedure(proc: &Procedure, _env: &Environment) -> Result<Value, SchemeError> {
    Ok(Value::Closure(Closure {
        params: proc.args.to_vec(),
        body: proc.body.to_vec(),
    }))
}

fn eval_let(letexp: &Let, env: &Environment) -> Result<Value, SchemeError> {
    let proc = Procedure {
        args: letexp
            .bindings
            .iter()
            .map(|(vardecl, _)| vardecl.clone())
            .collect(),
        body: letexp.body.to_vec(),
    };
    let proc = ConstituentExpression::Procedure(proc);
    let app = Application {
        operator: Box::from(proc),
        operands: letexp
            .bindings
            .iter()
            .map(|(_, cexp)| cexp.clone())
            .collect(),
    };
    let app = ConstituentExpression::Applic(app);
    applicative_eval(&app, env)
}

fn eval_applic(applic: &Application, env: &Environment) -> Result<Value, SchemeError> {
    let operator = applicative_eval(&applic.operator, env)?;
    let operands = applic
        .operands
        .iter()
        .map(|cexp| applicative_eval(cexp, env))
        .collect::<Result<Vec<Value>, SchemeError>>()?;
    apply_procedure(&operator, &operands, env)
}

fn apply_procedure(
    operator: &Value,
    operands: &[Value],
    env: &Environment,
) -> Result<Value, SchemeError> {
    match operator {
        Value::PrimitiveOperation(primop) => primitives::apply_primitive(primop, operands),
        Value::Closure(closure @ Closure { .. }) => apply_closure(closure, operands, env),
        _ => Err(SchemeError::BadProcedure(format!("{:?}", operator))),
    }
}

fn apply_closure(proc: &Closure, args: &[Value], env: &Environment) -> Result<Value, SchemeError> {
    let vars: Vec<_> = proc.params.iter().map(|vd| vd.0.to_owned()).collect();
    let body = substitution::rename(&proc.body);
    let lit_args: Vec<_> = args.iter().map(substitution::value_to_literal).collect();
    let exps = substitution::substitute(body.as_slice(), vars.as_slice(), lit_args.as_slice())
        .iter()
        .map(|e| Expression::ConstituentExpression(e.clone()))
        .collect::<Vec<_>>();
    eval_sequence(exps.as_slice(), env)
}

fn eval_sequence(exps: &[Expression], env: &Environment) -> Result<Value, SchemeError> {
    match exps {
        [] => Err(SchemeError::EmptyProgram),
        [Expression::Define(def @ Define { .. }), rest @ ..] => eval_define_exps(def, rest, env),
        [Expression::ConstituentExpression(cexp), es @ ..] => eval_cexps(cexp, es, env),
    }
}

fn eval_cexps(
    first: &ConstituentExpression,
    rest: &[Expression],
    env: &Environment,
) -> Result<Value, SchemeError> {
    let value = applicative_eval(first, env)?;
    if rest.is_empty() {
        Ok(value)
    } else {
        eval_sequence(rest, env)
    }
}

fn eval_define_exps(
    def: &Define,
    exps: &[Expression],
    env: &Environment,
) -> Result<Value, SchemeError> {
    let val = applicative_eval(&def.val, env)?;
    eval_sequence(
        exps,
        &Environment::NonEmpty(def.var.to_string(), val, Box::from(env.clone())),
    )
}
