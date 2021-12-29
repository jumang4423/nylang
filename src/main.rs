// REPL aka run source code

// lib
use std::env::args;
use std::io;
use std::io::prelude::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;

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
  let mut _lines: String;
  match match args().nth(1) {
    // run program from source code
    Some(command) => {
      match command.as_str() {
        "new" => {
          let project_name = args().nth(2).unwrap();

          // create a file with project_name.nyl
          match std::fs::File::create(project_name.clone() + ".nylsh") {
            Ok(mut file) => {
              match file.write_all(
                b"
#!/bin/sh
if ! [ -x \"$(command -v nylang)\" ]; then
  echo '-! nylang excutable not found in your PATH' >&2
  exit 1
fi

nylang run -c '
\xF0\x9F\x8D\x99 main = \xF0\x9F\x8F\xA8 () { 
  \xF0\x9F\x8E\xA4\xF0\x9F\x8E\xB6( \"hello world!\" ) ;
} ;
'
",
              ) {
                Ok(_) => {
                  println!("{} created", project_name);
                                // set excutable the file
                  fs::set_permissions(project_name.clone() + ".nylsh", fs::Permissions::from_mode(0o755)).unwrap();
                }
                Err(e) => {
                  println!("{}", e);
                }
              }
            }
            Err(e) => {
              println!("{}", e);
            }
          }

          Ok(())
        }
        "run" => {
          _lines = tools::tools::check_if_file().unwrap();

          run_program::run_program::run_program(_lines)
        }
        "lexer" => {
          _lines = tools::tools::check_if_file().unwrap();

          run_program::run_program::lexer_program(_lines)
        }
        "parser" => {
          _lines = tools::tools::check_if_file().unwrap();
          run_program::run_program::parse_program(_lines)
        }
        _ => help::help::help(),
      }
    }
    // run help
    None => help::help::help(),
  } {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}
