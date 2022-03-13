use crate::ast::*;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    PrimitiveOperation(PrimitiveOperation),
    Closure(Closure),
    SExpression(Box<SExpression>),
}

#[derive(Debug, Clone)]
pub struct Closure {
    pub params: Vec<VariableDeclaration>,
    pub body: Vec<Box<ConstituentExpression>>,
}

#[derive(Debug, Clone)]
pub enum SExpression {
    Nil,
    Symbol(String),
    Compound(Value, Value),
}

impl SExpression {
    pub fn is_empty(&self) -> bool {
        matches!(self, SExpression::Nil)
    }
}
