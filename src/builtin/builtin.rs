use super::super::lexer;
use super::super::object;
use super::super::parser;
use std::{env, io};
use std::cell::{RefCell, RefMut};
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
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

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        visit_dirs(&path, cb)?;
      } else {
        cb(&entry);
      }
    }
  }
  Ok(())
}

#[allow(deprecated)]
pub fn quick_import(
  args: std::vec::Vec<object::object::Object>,
  eval_global: &mut super::super::eval::eval::Evaluator,
) -> object::object::Object {
  if let object::object::Object::String(file_name) = &args[0] {

    // find one file from the current directory
    let seen_cell = RefCell::new("".to_string());
    let cur_dir = env::current_dir().unwrap().display().to_string();
    let mut is_found = false;
    let is_found_cell = RefCell::new(is_found);
    visit_dirs(Path::new(&cur_dir), &|entry| {
      let path = entry.path();
      if path.is_file() {
        if path.to_str().unwrap().contains(&file_name.clone()) {
          let mut borrowed = seen_cell.borrow_mut();
          let mut borrowed_is_found = is_found_cell.borrow_mut();
          borrowed = RefMut::map(borrowed, |s| {
            *s = path.to_str().unwrap().to_string();
            s
          });
          borrowed_is_found = RefMut::map(borrowed_is_found, |s| {
            *s = true;
            s
          });
        }
      }
    });

    if !is_found_cell.borrow().clone() {
      panic!("quick_import: file not found");
    }

    let _lines = match fs::read_to_string(format!("{}", &seen_cell.borrow())) {
      Ok(lines) => lines,
      Err(_e) => panic!("Error reading file"),
    };
    let l = lexer::lexer::Lexer::new(_lines.as_str());
    let mut p = parser::parser::Parser::new(l);
    let program = p.program_parser();

    if let object::object::Object::String(func_name) = &args[1] {
      // loop through the program
      for statement in program.statements.iter() {
        // evaluate the statement
        let evaluated = eval_global.statement_evaluator(statement.clone());
        if let object::object::Object::String(identifier) = evaluated {
          // the function found!
          if identifier == *func_name {
            return object::object::Object::Null
          }
        }
      }
    }

    // no expected function, return err
    object::object::Object::String("no func found".to_string())
  } else {
    panic!("quick_import: argument must be (string, string)");
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
