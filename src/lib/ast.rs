use crate::value::SExpression;

#[derive(Debug, Clone)]
pub struct Application {
    pub operator: Box<ConstituentExpression>,
    pub operands: Vec<Box<ConstituentExpression>>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub cond: Box<ConstituentExpression>,
    pub then: Box<ConstituentExpression>,
    pub alt: Box<ConstituentExpression>,
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub args: Vec<VariableDeclaration>,
    pub body: Vec<Box<ConstituentExpression>>,
}

#[derive(Debug, Clone)]
pub struct Let {
    pub bindings: Vec<(VariableDeclaration, Box<ConstituentExpression>)>,
    pub body: Vec<Box<ConstituentExpression>>,
}

#[derive(Debug, Clone)]
pub struct Number(pub f64);

#[derive(Debug, Clone)]
pub struct Boolean(pub bool);

#[derive(Debug, Clone)]
pub struct Str(pub String);

#[derive(Debug, Clone)]
pub enum PrimitiveOperation {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Not,
    And,
    Or,
    IsEq,
    Cons,
    Car,
    Cdr,
    List,
    IsPair,
    IsNumber,
    IsBoolean,
    IsSymbol,
}

#[derive(Debug, Clone)]
pub struct VariableReference(pub String);

#[derive(Debug, Clone)]
pub struct VariableDeclaration(pub String);

#[derive(Debug, Clone)]
pub enum ConstituentExpression {
    Applic(Application),
    If(If),
    Procedure(Procedure),
    Let(Let),
    Number(Number),
    Boolean(Boolean),
    Literal(SExpression),
    PrimitiveOperation(PrimitiveOperation),
    VariableReference(VariableReference),
}

#[derive(Debug, Clone)]
pub struct Define {
    pub var: String,
    pub val: ConstituentExpression,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Define(Define),
    ConstituentExpression(ConstituentExpression),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub exps: Vec<Expression>,
}
