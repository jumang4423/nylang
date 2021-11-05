////////////////////////////////////////////////////////////////////////////////
///
/// // error handling
///
////////////////////////////////////////////////////////////////////////////////

use super::super::object;

pub fn panipani(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
  if args.len() == 1 {
    if let object::object::Object::String(string) = &args[0] {
      println!(
        "{} {}",
        "panic:",
        string
      );
      std::process::exit(1);
    } else {
      panic!("panic: argument must be a string");
    }
  } else {
    panic!("🎤: too many arguments");
  }
}