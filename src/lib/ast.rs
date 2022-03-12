#[derive(Debug)]
pub struct Application {
    pub operator: Box<ConstituentExpression>,
    pub operands: Vec<ConstituentExpression>,
}

#[derive(Debug)]
pub struct If {
    pub cond: Box<ConstituentExpression>,
    pub then: Box<ConstituentExpression>,
    pub alt: Box<ConstituentExpression>,
}

#[derive(Debug)]
pub struct Procedure {
    pub args: Vec<VariableDeclaration>,
    pub body: Vec<ConstituentExpression>,
}

#[derive(Debug)]
pub struct Let {
    pub bindings: Vec<(VariableDeclaration, Box<ConstituentExpression>)>,
    pub body: Vec<ConstituentExpression>,
}

#[derive(Debug)]
pub struct Number(pub f64);

#[derive(Debug)]
pub struct Boolean(pub bool);

#[derive(Debug)]
pub struct Str(pub String);

#[derive(Debug)]
pub struct PrimitiveOperation(pub String);

#[derive(Debug)]
pub struct VariableReference(pub String);

#[derive(Debug)]
pub struct VariableDeclaration(pub String);

#[derive(Debug)]
pub enum ConstituentExpression {
    Applic(Application),
    If(If),
    Procedure(Procedure),
    Let(Let),
    Number(Number),
    Boolean(Boolean),
    Str(Str),
    PrimitiveOperation(PrimitiveOperation),
    VariableReference(VariableReference),
}

#[derive(Debug)]
pub enum Expression {
    Define { var: String, val: String },
    ConstituentExpression(ConstituentExpression),
}

#[derive(Debug)]
pub struct Program {
    exps: Vec<Expression>,
}
