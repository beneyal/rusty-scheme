use crate::ast::*;

#[derive(Debug)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    Str(String),
    PrimitiveOperation(String),
    Closure {
        params: Vec<VariableDeclaration>,
        body: Vec<ConstituentExpression>,
    },
    SExpression(Box<SExpression>),
}

#[derive(Debug)]
pub enum SExpression {
    Nil,
    Symbol(String),
    Compound(Value, Value),
}
