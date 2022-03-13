use rusty_scheme::value::Value;
use rusty_scheme::{eval_program, parse_program, SchemeError};

fn run(src: &str) -> Result<Value, SchemeError> {
    let parsed = parse_program(src);
    let program = match parsed {
        Ok((_, p)) => Ok(p),
        Err(e) => {
            dbg!(e);
            Err(SchemeError::ParseError)
        }
    }?;
    eval_program(program)
}

fn main() {
    let src = "(L3 (define x 5) (define sq (lambda (x) (* x x))) (sq 5))";
    println!("Evaluating {}", src);
    println!("Result: {:#?}", run(src));
}
