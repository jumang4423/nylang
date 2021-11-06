use super::super::object;
use rand::Rng;

pub fn bark(args: std::vec::Vec<object::object::Object>, newline: bool) -> Vec<String> {
  let mut return_output = Vec::new();
  if let object::object::Object::String(str) = &args[0] {
    let array_of_str: Vec<String> = str.as_str().split("\\n").map(|s| s.to_string()).collect();

    for s in array_of_str.iter() {
      return_output.push(format!("{}", s));
      if array_of_str.len() > 1 && newline {
        return_output.push("".to_string());
      }
    }

  } else {
    return_output.push(format!("{}", args[0]));
  }
  if newline {
    return_output.push("".to_string());
  }

  return_output
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
