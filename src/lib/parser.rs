use crate::ast::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, multispace0, multispace1};
use nom::combinator::{cut, map};
use nom::multi::{many0, many1};
use nom::number::complete::double;
use nom::sequence::{delimited, preceded, separated_pair, terminated, tuple};
use nom::IResult;

pub fn parse_program(i: &str) -> IResult<&str, Program> {
    let inner = map(
        preceded(
            terminated(tag("L3"), multispace1),
            cut(many1(parse_expression)),
        ),
        |exps| Program { exps },
    );
    delimited(char('('), inner, char(')'))(i)
}

fn parse_expression(i: &str) -> IResult<&str, Expression> {
    preceded(
        multispace0,
        alt((
            parse_define,
            map(parse_cexp, Expression::ConstituentExpression),
        )),
    )(i)
}

fn parse_define(i: &str) -> IResult<&str, Expression> {
    let inner = map(
        preceded(
            terminated(tag("define"), multispace1),
            cut(tuple((alpha1, parse_cexp))),
        ),
        |(var, val)| {
            Expression::Define(Define {
                var: var.to_string(),
                val,
            })
        },
    );
    delimited(char('('), inner, char(')'))(i)
}

pub fn parse_cexp(i: &str) -> IResult<&str, ConstituentExpression> {
    preceded(
        multispace0,
        alt((
            parse_primop,
            parse_boolean,
            parse_number,
            parse_var,
            parse_if,
            parse_proc,
            parse_let,
            parse_applic,
        )),
    )(i)
}

fn parse_applic(i: &str) -> IResult<&str, ConstituentExpression> {
    let inner = map(
        tuple((parse_cexp, many0(parse_cexp))),
        |(operator, operands)| {
            ConstituentExpression::Applic(Application {
                operator: Box::from(operator),
                operands: operands
                    .iter()
                    .map(|cexp| Box::from(cexp.clone()))
                    .collect(),
            })
        },
    );
    delimited(char('('), inner, char(')'))(i)
}

fn parse_if(i: &str) -> IResult<&str, ConstituentExpression> {
    let inner = map(
        preceded(
            terminated(tag("if"), multispace1),
            cut(tuple((parse_cexp, parse_cexp, parse_cexp))),
        ),
        |(cond, then, alt)| {
            ConstituentExpression::If(If {
                cond: Box::from(cond),
                then: Box::from(then),
                alt: Box::from(alt),
            })
        },
    );
    delimited(char('('), inner, char(')'))(i)
}

fn parse_proc(i: &str) -> IResult<&str, ConstituentExpression> {
    let inner = map(
        preceded(
            terminated(tag("lambda"), multispace1),
            cut(tuple((
                delimited(char('('), many0(preceded(multispace0, alpha1)), char(')')),
                many1(parse_cexp),
            ))),
        ),
        |(params, body)| {
            ConstituentExpression::Procedure(Procedure {
                args: params
                    .iter()
                    .map(|arg| VariableDeclaration(arg.to_owned().to_owned()))
                    .collect(),
                body: body.iter().map(|cexp| Box::from(cexp.clone())).collect(),
            })
        },
    );
    delimited(char('('), inner, char(')'))(i)
}

fn parse_binding(i: &str) -> IResult<&str, (VariableDeclaration, Box<ConstituentExpression>)> {
    map(
        delimited(
            char('('),
            separated_pair(preceded(multispace0, alpha1), multispace1, parse_cexp),
            char(')'),
        ),
        |(vd, cexp)| (VariableDeclaration(vd.to_owned()), Box::from(cexp)),
    )(i)
}

fn parse_let(i: &str) -> IResult<&str, ConstituentExpression> {
    let inner = map(
        preceded(
            terminated(tag("let"), multispace1),
            cut(tuple((
                delimited(char('('), many0(parse_binding), char(')')),
                many1(parse_cexp),
            ))),
        ),
        |(bindings, body)| {
            ConstituentExpression::Let(Let {
                bindings,
                body: body.iter().map(|cexp| Box::from(cexp.clone())).collect(),
            })
        },
    );
    delimited(char('('), inner, char(')'))(i)
}

fn parse_number(i: &str) -> IResult<&str, ConstituentExpression> {
    map(double, |x: f64| ConstituentExpression::Number(Number(x)))(i)
}

fn parse_boolean(i: &str) -> IResult<&str, ConstituentExpression> {
    alt((
        map(tag("#t"), |_| ConstituentExpression::Boolean(Boolean(true))),
        map(tag("#f"), |_| {
            ConstituentExpression::Boolean(Boolean(false))
        }),
    ))(i)
}

fn parse_var(i: &str) -> IResult<&str, ConstituentExpression> {
    map(alpha1, |v: &str| {
        ConstituentExpression::VariableReference(VariableReference(v.to_owned()))
    })(i)
}

fn parse_quoted(i: &str) -> IResult<&str, ConstituentExpression> {
    todo!()
}

fn parse_primop(i: &str) -> IResult<&str, ConstituentExpression> {
    use PrimitiveOperation::*;
    let parse_math_op = alt((
        map(tag("+"), |_| ConstituentExpression::PrimitiveOperation(Add)),
        map(tag("-"), |_| ConstituentExpression::PrimitiveOperation(Sub)),
        map(tag("*"), |_| ConstituentExpression::PrimitiveOperation(Mul)),
        map(tag("/"), |_| ConstituentExpression::PrimitiveOperation(Div)),
        map(tag("<"), |_| {
            ConstituentExpression::PrimitiveOperation(LessThan)
        }),
        map(tag("<="), |_| {
            ConstituentExpression::PrimitiveOperation(LessThanOrEqual)
        }),
        map(tag(">"), |_| {
            ConstituentExpression::PrimitiveOperation(GreaterThan)
        }),
        map(tag(">="), |_| {
            ConstituentExpression::PrimitiveOperation(GreaterThanOrEqual)
        }),
    ));
    let parse_logical_op = alt((
        map(tag("not"), |_| {
            ConstituentExpression::PrimitiveOperation(Not)
        }),
        map(tag("and"), |_| {
            ConstituentExpression::PrimitiveOperation(And)
        }),
        map(tag("or"), |_| ConstituentExpression::PrimitiveOperation(Or)),
    ));
    let parse_pair_op = alt((
        map(tag("cons"), |_| {
            ConstituentExpression::PrimitiveOperation(Cons)
        }),
        map(tag("car"), |_| {
            ConstituentExpression::PrimitiveOperation(Car)
        }),
        map(tag("cdr"), |_| {
            ConstituentExpression::PrimitiveOperation(Cdr)
        }),
        map(tag("list"), |_| {
            ConstituentExpression::PrimitiveOperation(List)
        }),
    ));
    let parse_predicate = alt((
        map(tag("eq?"), |_| {
            ConstituentExpression::PrimitiveOperation(IsEq)
        }),
        map(tag("pair?"), |_| {
            ConstituentExpression::PrimitiveOperation(IsPair)
        }),
        map(tag("number?"), |_| {
            ConstituentExpression::PrimitiveOperation(IsNumber)
        }),
        map(tag("boolean?"), |_| {
            ConstituentExpression::PrimitiveOperation(IsBoolean)
        }),
        map(tag("symbol?"), |_| {
            ConstituentExpression::PrimitiveOperation(IsSymbol)
        }),
    ));

    alt((
        parse_math_op,
        parse_logical_op,
        parse_pair_op,
        parse_predicate,
    ))(i)
}
