use super::super::object;
use super::super::tools;
use rand::Rng;
use std::io;

macro_rules! cast {
  ($target: expr, $pat: path) => {{
    if let $pat(a) = $target {
      // #1
      a
    } else {
      panic!("mismatch variant when cast to {}", stringify!($pat)); // #2
    }
  }};
}

pub fn scanf(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
  let mut _type = "string";
  assert!(!(args.len() != 1), "get_input: wrong number of arguments");

  if let object::object::Object::String(_str) = &args[0] {
    _type = _str;
  }

  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("cannot get input");

  // remove last enter from input

  return match _type {
    "number" => object::object::Object::Integer(input.trim().parse::<i32>().unwrap()),
    "boolean" | "bool" => {
      object::object::Object::Boolean(tools::tools::str_to_bool(input.trim().to_string()))
    }
    _ => object::object::Object::String(input.trim().to_string()),
  };
}

pub fn bark(args: std::vec::Vec<object::object::Object>, newline: bool) -> object::object::Object {
  if let object::object::Object::String(str) = &args[0] {
    let array_of_str: Vec<String> = str.as_str().split("\\n").map(|s| s.to_string()).collect();

    match args.len() {
      1 => {
        for s in array_of_str.iter() {
          print!("{}", s);
          if array_of_str.len() > 1 && newline {
            println!();
          }
        }
      }
      5 => {
        // unwrap colors from objects
        let red = cast!(args[1], object::object::Object::Integer);
        let green = cast!(args[2], object::object::Object::Integer);
        let blue = cast!(args[3], object::object::Object::Integer);
        let is_text_coloring = cast!(args[4], object::object::Object::Boolean);

        if is_text_coloring {
          for s in array_of_str.iter() {
            print!("{}", s.to_string());
            if array_of_str.len() > 1 && newline {
              println!();
            }
          }
        } else {
          for s in array_of_str.iter() {
            print!("{}", s.to_string());
            if array_of_str.len() > 1 && newline {
              println!();
            }
          }
        }
      }
      _ => {
        panic!("🎤: arguments are invalid, arg len should be 1 or 5")
      }
    }
  } else {
    match args.len() {
      1 => {
        print!("{}", args[0]);
      }
      5 => {
        // unwrap colors from objects
        let red = cast!(args[1], object::object::Object::Integer);
        let green = cast!(args[2], object::object::Object::Integer);
        let blue = cast!(args[3], object::object::Object::Integer);
        let is_text_coloring = cast!(args[4], object::object::Object::Boolean);

        if is_text_coloring {
          print!("{}", format!("{}", args[0]));
        } else {
          print!("{}", format!("{}", args[0]));
        }
      }
      _ => {
        panic!("🎤: arguments are invalid, arg len should be 1 or 5")
      }
    }
  }
  if newline {
    println!();
  }

  object::object::Object::Null
}

pub fn random_emojis(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
  let emojis = vec!["🐧", "🦄", "🐝", "🐹", "🐰", "🦊", "🐼", "🐨", "🐯", "🐷"];

  if args.len() == 1 {
    if let object::object::Object::Integer(int) = &args[0] {
      let mut emojis_vec = vec![];
      for _ in 0..*int {
        let index = rand::thread_rng().gen_range(0, emojis.len());
        emojis_vec.push(emojis[index].to_string());
      }

      object::object::Object::String(emojis_vec.join(" "))
    } else {
      panic!("random_emoji: argument must be an integer");
    }
  } else {
    panic!("random_emoji: too many arguments");
  }
}
