use super::super::lexer;
use super::super::object;
use super::super::parser;
use std::env;
use std::fs;
use std::thread;

#[allow(deprecated)]
pub fn import(
  args: std::vec::Vec<object::object::Object>,
  eval_global: &mut super::super::eval::eval::Evaluator,
) -> object::object::Object {
  if let object::object::Object::String(file_name) = &args[0] {
    let file_name_string: String = file_name.clone();
    let home_path = env::home_dir().unwrap().display().to_string();

    let _lines = match fs::read_to_string(format!("{}/{}", &home_path, &file_name_string)) {
      Ok(lines) => lines,
      Err(_e) => panic!("Error reading file"),
    };
    let l = lexer::lexer::Lexer::new(_lines.as_str());
    let mut p = parser::parser::Parser::new(l);
    let program = p.program_parser();

    // loop through the program
    for statement in program.statements.iter() {
      // evaluate the statement
      eval_global.statement_evaluator(statement.clone());
    }

    object::object::Object::Null
  } else {
    panic!("import: argument must be a string");
  }
}

#[allow(deprecated)]
pub fn sleep(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
  if let object::object::Object::Integer(seconds) = args[0] {
    thread::sleep_ms(seconds as u32);
  } else {
    panic!("night: argument must be an integer");
  }

  object::object::Object::Null
}

pub fn looper(
  args: std::vec::Vec<object::object::Object>,
  eval: &mut super::super::eval::eval::Evaluator,
) -> object::object::Object {
  if let object::object::Object::Closure {
    parameters: _,
    body,
    env: _,
  } = &args[0]
  {
    if args.len() == 1 {
      loop {
        if let object::object::Object::ReturnValue(value) = eval.statement_evaluator(body.clone()) {
          if let object::object::Object::Boolean(b_vle) = *value {
            if !b_vle {
              break;
            }
          } else {
            panic!("loop: function must return a boolean since this loop function doesnt have a loop number");
          }
        } else {
          panic!("loop: function must return something since this loop function doesnt have a loop number");
        }
      }
    } else if let object::object::Object::Integer(int) = &args[1] {
      for _ in 0..*int {
        eval.statement_evaluator(body.clone());
      }
    } else {
      panic!("loop: argument must be an integer");
    }
  } else {
    panic!("loop: first argument must be a function");
  }

  object::object::Object::Null
}
pub fn clear() -> object::object::Object {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
  object::object::Object::Null
}

// ////////////////////////////////////////////////////////////////////////////////
// ///
// /// // types
// ///
// ////////////////////////////////////////////////////////////////////////////////

// pub fn type_check(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
//   #[derive(PartialEq)]
//   enum CheckableEnum {
//     String,
//     Integer,
//     Boolean,
//     Array,
//     Closure,
//   }

//   // obj to type
//   let type_wrap_closure = |obj: object::object::Object| -> CheckableEnum {
//     match obj {
//       object::object::Object::Integer(_) => CheckableEnum::Integer,
//       object::object::Object::String(_) => CheckableEnum::String,
//       object::object::Object::Array(_) => CheckableEnum::Array,
//       object::object::Object::Boolean(_) => CheckableEnum::Boolean,
//       object::object::Object::Closure {
//         parameters: _,
//         body: _,
//         env: _,
//       } => CheckableEnum::Closure,
//       _ => {
//         panic!("type_check: argument must be a string, integer, array, function or boolean");
//       }
//     }
//   };

//   let left_str: CheckableEnum = type_wrap_closure(args[0].clone());
//   let right_str: CheckableEnum = type_wrap_closure(args[1].clone());
//   object::object::Object::Boolean(left_str == right_str)
// }
