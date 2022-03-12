use crate::value::Value;
use crate::SchemeError;

#[derive(Debug)]
pub enum Environment {
    Empty,
    NonEmpty(String, Value, Box<Environment>),
}

impl Environment {
    pub fn apply(&self, var: &str) -> Result<&Value, SchemeError> {
        match self {
            Environment::Empty => Err(SchemeError::UndefinedVariable(format!(
                "Undefined variable: {}",
                var
            ))),
            Environment::NonEmpty(v, val, next_env) => {
                if var == v {
                    Ok(val)
                } else {
                    next_env.apply(var)
                }
            }
        }
    }
}
