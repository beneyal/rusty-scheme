pub mod ast;
mod env;
pub mod eval;
pub mod parser;
pub mod value;

pub enum SchemeError {
    UndefinedVariable(String),
    BadProcedure(String),
}
