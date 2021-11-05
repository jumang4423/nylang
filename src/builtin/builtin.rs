use super::super::object;

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

pub fn clear() -> Vec<String> {
  let mut array: Vec<String> = [].to_vec();
  array.push(format!("{esc}[2J{esc}[1;1H", esc = 27 as char));
  array
}
