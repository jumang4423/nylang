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
  let mut _lines:String;
  match match env::args().nth(1) {
    // run program from source code
    Some(command) => {
      match command.as_str(){
      "run"=>{
         _lines= tools::tools::check_if_file().unwrap();
       println!("{}",_lines);
        run_program::run_program::run_program(_lines)
      }, 
       
      "lexer" =>{
         _lines= tools::tools::check_if_file().unwrap();
       
        run_program::run_program::lexer_program(_lines)
      },  
      "parser" =>{
        _lines=tools::tools::check_if_file().unwrap();
        run_program::run_program::parse_program(_lines)
      }, _=> {
        help::help::help()
      },
    }
  }
    // run help
    None => help::help::help(),
  } {
    Ok(_) => Ok(()),
    Err(e) => Err(e),
  }
}
