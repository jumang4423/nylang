//#[allow(clippy::match_str_case_mismatch)]
use std::env::args;
use std::fs::read_to_string;
use std::io;
pub fn str_to_bool(_str: String) -> bool {
  match _str.to_lowercase().as_str() {
    "true" => true,
    "false" => false,
    "1" => true,
    "0" => false,
    _ => false,
  }
}
pub fn check_if_file() -> Result<String, io::Error> {
  let mut _lines: String;
  if args().nth(2).unwrap() == "-c" {
    _lines = args().nth(3).unwrap()
  } else {
    _lines = match read_to_string(args().nth(2).unwrap()) {
      Ok(_lines) => _lines,
      Err(_e) => return Err(io::Error::new(io::ErrorKind::Other, "Could not read file")),
    };
  }

  Ok(_lines)
}
