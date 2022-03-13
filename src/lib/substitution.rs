use crate::ast::*;
use crate::value::Value;

fn make_var_gen() -> impl FnMut(&str) -> String {
    let mut count = 0;
    move |v| {
        count += 1;
        format!("{}__{}", v, count)
    }
}

fn replace(
    cexp: &ConstituentExpression,
    var_gen: &mut impl FnMut(&str) -> String,
) -> ConstituentExpression {
    match cexp {
        ConstituentExpression::Applic(applic) => {
            let applic = Application {
                operator: Box::from(replace(&applic.operator, var_gen)),
                operands: applic
                    .operands
                    .iter()
                    .map(|x| Box::from(replace(x, var_gen)))
                    .collect(),
            };
            ConstituentExpression::Applic(applic)
        }
        ConstituentExpression::If(ifexp) => {
            let cond = replace(&ifexp.cond, var_gen);
            let then = replace(&ifexp.then, var_gen);
            let alt = replace(&ifexp.alt, var_gen);
            ConstituentExpression::If(If {
                cond: Box::from(cond),
                then: Box::from(then),
                alt: Box::from(alt),
            })
        }
        ConstituentExpression::Let(_) => {
            unreachable!("let expressions don't exist at this point")
        }
        ConstituentExpression::Procedure(proc) => {
            let old_args: Vec<_> = proc.args.iter().map(|vd| vd.0.to_owned()).collect();
            let new_args: Vec<_> = old_args.iter().map(|v| var_gen(v)).collect();
            let new_body: Vec<_> = proc
                .body
                .iter()
                .map(|x| Box::from(replace(x, var_gen)))
                .collect();
            ConstituentExpression::Procedure(Procedure {
                args: new_args
                    .iter()
                    .map(|v| VariableDeclaration(v.to_string()))
                    .collect(),
                body: new_body,
            })
        }
        _ => cexp.clone(),
    }
}

pub(crate) fn rename(exps: &[Box<ConstituentExpression>]) -> Vec<Box<ConstituentExpression>> {
    exps.iter()
        .map(|x| Box::from(replace(x, &mut make_var_gen())))
        .collect()
}

pub(crate) fn value_to_literal(value: &Value) -> ConstituentExpression {
    match value {
        Value::Number(n) => ConstituentExpression::Number(Number(*n)),
        Value::Boolean(b) => ConstituentExpression::Boolean(Boolean(*b)),
        Value::PrimitiveOperation(p) => ConstituentExpression::PrimitiveOperation(p.clone()),
        Value::Closure(c) => ConstituentExpression::Procedure(Procedure {
            args: c.params.to_vec(),
            body: c.body.to_vec(),
        }),
        Value::SExpression(s) => ConstituentExpression::Literal(*s.to_owned()),
    }
}

fn substitute_one(
    cexp: &ConstituentExpression,
    vars: &[String],
    exps: &[ConstituentExpression],
) -> ConstituentExpression {
    match cexp {
        ConstituentExpression::VariableReference(v) => {
            let pos = vars.iter().position(|var| v.0 == **var);
            match pos {
                Some(i) => exps[i].clone(),
                None => cexp.clone(),
            }
        }
        ConstituentExpression::If(ifexp) => {
            let cond = substitute_one(&*ifexp.cond, vars, exps);
            let then = substitute_one(&*ifexp.then, vars, exps);
            let alt = substitute_one(&*ifexp.alt, vars, exps);
            ConstituentExpression::If(If {
                cond: Box::from(cond),
                then: Box::from(then),
                alt: Box::from(alt),
            })
        }
        ConstituentExpression::Applic(applic) => {
            let operator = Box::from(substitute_one(&applic.operator, vars, exps));
            let operands = applic
                .operands
                .iter()
                .map(|x| Box::from(substitute_one(x, vars, exps)))
                .collect();
            ConstituentExpression::Applic(Application { operator, operands })
        }
        ConstituentExpression::Procedure(proc) => {
            let args: Vec<_> = proc.args.iter().map(|a| a.0.to_owned()).collect();
            let subst = vars.iter().zip(exps.iter());
            let free_subst: Vec<_> = subst.filter(|(v, _)| !args.contains(v)).collect();
            let (vars, exps): (Vec<_>, Vec<_>) = free_subst.iter().cloned().unzip();
            ConstituentExpression::Procedure(Procedure {
                args: proc.args.to_vec(),
                body: substitute(
                    &proc.body.to_vec(),
                    vars.iter()
                        .map(|s| s.to_owned().clone())
                        .collect::<Vec<_>>()
                        .as_slice(),
                    exps.iter()
                        .map(|e| e.to_owned().clone())
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .iter()
                .map(|cexp| Box::from(cexp.clone()))
                .collect(),
            })
        }
        ConstituentExpression::Let(_) => {
            unreachable!("let expressions don't exist at this point")
        }
        _ => cexp.clone(),
    }
}

pub(crate) fn substitute(
    body: &[Box<ConstituentExpression>],
    vars: &[String],
    exps: &[ConstituentExpression],
) -> Vec<ConstituentExpression> {
    body.iter()
        .map(|cexp| substitute_one(cexp, vars, exps))
        .collect()
}
