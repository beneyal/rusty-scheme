mod ast;
mod env;
mod eval;
mod parser;
mod primitives;
mod substitution;
pub mod value;

pub use eval::{applicative_eval, eval_program};
pub use parser::{parse_cexp, parse_program};

#[derive(Debug)]
pub enum SchemeError {
    ParseError,
    UndefinedVariable(String),
    BadProcedure(String),
    TypeMismatch(String),
    ArgumentMismatch(String),
    EmptyProgram,
}
