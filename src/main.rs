// REPL aka run source code

// lib
use std::env;
use std::io;

// mod
mod ast;
mod builtin;
mod enve;
mod eval;
mod help;
mod lexer;
mod object;
mod parser;
mod run_program;
mod token;
mod tools;

fn main() -> io::Result<()> {
  match match env::args().nth(1) {
    // run program from source code
    Some(command) => {
      return if command == "run" {
        run_program::run_program::run_program(env::args().nth(2).unwrap())
      } else if command == "lexer" {
        run_program::run_program::lexer_program(env::args().nth(2).unwrap())
      } else if command == "parser" {
        run_program::run_program::parse_program(env::args().nth(2).unwrap())
      } else {
        help::help::help()
      }
    }
    // run help
    None => help::help::help(),
  } {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}
