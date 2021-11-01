////////////////////////////////////////////////////////////////////////////////
///
/// // error handling
///
////////////////////////////////////////////////////////////////////////////////

use super::super::object;
use colored::Colorize;

pub fn panipani(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
  if args.len() == 1 {
    if let object::object::Object::String(string) = &args[0] {
      println!(
        "{} {}",
        "panic:".on_truecolor(255, 150, 150).bold(),
        string.truecolor(255, 0, 0).bold()
      );
      std::process::exit(1);
    } else {
      panic!("panic: argument must be a string");
    }
  } else {
    panic!("🎤: too many arguments");
  }
}