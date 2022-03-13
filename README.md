# A Very Rusty Scheme

## Intro
This crate implements a minimal (and incomplete) Scheme interpreter based on the code from the
"Principles of Programming Languages" course from Ben-Gurion University of the Negev.
Original code is in TypeScript and can be found [here](https://github.com/bguppl/interpreters),
course lectures can be found [here](https://bguppl.github.io/interpreters/).
The version of the language I implemented is called "L2", which supports
primitive operations, function application, `define` expressions,
`lambda` expressions, `if` expressions, and `let` expressions.

I know there's a ton of room for improvement, as I'm still a young Rustacean,
but who knows, maybe I'll get to refactoring this project some time.

## How to Use
There is currently no REPL interface, I'll maybe add one once I have some spare time.
To run a program, change the `src` variable in `bin/scm.rs`.
Programs must be enclosed in `(L3 ...)`.

## Dependencies
This crate only depends on [nom](https://github.com/Geal/nom) for parsing the S-expression language.