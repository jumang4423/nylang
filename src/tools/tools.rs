//#[allow(clippy::match_str_case_mismatch)]
use git2::Repository;
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

pub fn check_nylang_lib() -> Option<std::path::PathBuf> {
  // check the ~/.nylang is exsists
  let _nylang_path = match dirs::home_dir() {
    Some(path) => path.join(".nylang"),
    None => return None,
  };
  // is _nylang_path exsists
  if !_nylang_path.exists() {
    return Some(_nylang_path);
  } else {
    return None;
  }
}

pub fn auto_install_lib() -> Result<(), String> {
  // auto installing nylang lib
  println!("{}", "-> started auto install!");
  println!("{}", "-> fetching nylang lib into ~/.nylang");
  let url = "https://github.com/jumang4423/nylang.git";
  let _cache_path = match dirs::home_dir() {
    Some(path) => path.join(".nylang"),
    None => return Err("Could not get home directory".to_string()),
  };
  match Repository::clone(url, _cache_path) {
    Err(e) => return Err(format!("failed to fetch lib: {:?}", e).to_string()),
    _ => (),
  };
  println!("{}", "-> removing unnecessary files");
  let _nylang_path = match dirs::home_dir() {
    Some(path) => path.join(".nylang"),
    None => return Err("Could not get home directory".to_string()),
  };

  // remove ~/.nylang/README.md
  match std::fs::remove_file(_nylang_path.join("README.md")) {
    Err(_e) => return Err("Could not remove README.md".to_string()),
    _ => (),
  };
  println!("{}", "-> removed README.md");
  // remove ~/.nylang/Dockerfile
  match std::fs::remove_file(_nylang_path.join("Dockerfile")) {
    Err(_e) => return Err("Could not remove Dockerfile".to_string()),
    _ => (),
  };
  println!("{}", "-> removed Dockerfile");
  // remove ~/.nylang/Cargo.toml
  match std::fs::remove_file(_nylang_path.join("Cargo.toml")) {
    Err(_e) => return Err("Could not remove Cargo.toml".to_string()),
    _ => (),
  };
  println!("{}", "-> removed Cargo.toml");
  // remove ~/.nylang/Cargo.lock
  match std::fs::remove_file(_nylang_path.join("Cargo.lock")) {
    Err(_e) => return Err("Could not remove Cargo.lock".to_string()),
    _ => (),
  };
  println!("{}", "-> removed Cargo.lock");
  // remove ~/.nylang/docker-compose.yml
  match std::fs::remove_file(_nylang_path.join("docker-compose.yml")) {
    Err(_e) => return Err("Could not remove docker-compose.yml".to_string()),
    _ => (),
  };
  println!("{}", "-> removed docker-compose.yml");
  // remove ~/.nylang/_img dir
  match std::fs::remove_dir_all(_nylang_path.join("_img")) {
    Err(_e) => return Err("Could not remove _img dir".to_string()),
    _ => (),
  };
  println!("{}", "-> removed _img dir");
  // remove ~/.nylang/src dir
  match std::fs::remove_dir_all(_nylang_path.join("src")) {
    Err(_e) => return Err("Could not remove src dir".to_string()),
    _ => (),
  };
  println!("{}", "-> removed src dir");
  

  Ok(())
}
