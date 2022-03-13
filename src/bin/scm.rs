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
    eval_program(&program)
}

fn main() {
    let newton_src = "
    (L3
        (define sqrt (lambda (x) (sqrt-iter 1 x)))

        (define sqrt-iter
          (lambda (guess x)
             (if (good-enough? guess x)
                 guess
                 (sqrt-iter (improve guess x)
                            x))))

        (define abs (lambda (x) (if (< x 0) (- x) x)))
        (define square (lambda (x) (* x x)))
        (define epsilon 0.0001)

        (define good-enough?
          (lambda (guess x)
             (< (abs (- (square guess) x)) epsilon)))

        (define average
          (lambda (x y) (/ (+ x y) 2.0)))

        (define improve
          (lambda (guess x)
            (average guess (/ x guess))))

        (sqrt 2))";
    let _fact_src = "(L3 (define fact (lambda (n) (if (= n 0) 1 (* n (fact (- n 1)))))) (fact 5))";
    println!("Result: {:#?}", run(newton_src));
}
