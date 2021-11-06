////////////////////////////////////////////////////////////////////////////////
///
/// // Array related functions
///
////////////////////////////////////////////////////////////////////////////////
use super::super::object;

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

pub fn assign(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
  // ( array, value, index0 )
  if args.len() == 3 {
    match &args[0] {
      object::object::Object::Array(array) => {
        let index: i32 = match &args[2] {
          object::object::Object::Integer(int) => *int,
          _ => panic!("🎤: index must be an integer"),
        };

        let mut array_copy = array.clone();
        array_copy[index as usize] = args[1].clone();
        object::object::Object::Array(array_copy)
      }
      _ => {
        panic!("🎤: first argument must be an array");
      }
    }
    // ( array, value, index0, index1 )
  } else if args.len() == 4 {
    match &args[0] {
      object::object::Object::Array(array) => {
        match &array[cast!(args[2], object::object::Object::Integer) as usize] {
          object::object::Object::Array(arr_arr) => {
            let mut arr_arr_clone = arr_arr.clone();
            let mut array_clone = array.clone();
            arr_arr_clone[cast!(args[3], object::object::Object::Integer) as usize] =
              args[1].clone();
            array_clone[cast!(args[2], object::object::Object::Integer) as usize] =
              object::object::Object::Array(arr_arr_clone);
            object::object::Object::Array(array_clone)
          }
          _ => {
            panic!("🎤: second dimention must be an array");
          }
        }
      }
      _ => {
        panic!("🎤: first argument must be an array");
      }
    }
  } else {
    panic!("🎤: too many arguments, got {}", args.len());
  }
}

pub fn len(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
  if args.len() == 1 {
    match &args[0] {
      object::object::Object::String(string) => {
        object::object::Object::Integer(string.len() as i32)
      }
      object::object::Object::Array(array) => object::object::Object::Integer(array.len() as i32),
      _ => {
        panic!("len: argument must be a string or array");
      }
    }
  } else {
    panic!("len: too many arguments");
  }
}

pub fn push(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
  if args.len() == 2 {
    match &args[0] {
      object::object::Object::Array(array) => {
        let mut array_clone = array.clone();
        array_clone.push(args[1].clone());
        object::object::Object::Array(array_clone)
      }
      object::object::Object::String(string) => {
        let mut string_clone = string.clone();
        string_clone.push_str(&format!("{}", args[1]));
        object::object::Object::String(string_clone)
      }
      _ => {
        panic!("push: argument must be an array");
      }
    }
  } else {
    panic!("push: too many arguments");
  }
}

pub fn rest(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
  if args.len() == 1 {
    match &args[0] {
      object::object::Object::Array(array) => {
        let mut array_clone = array.clone();
        array_clone.remove(0);
        object::object::Object::Array(array_clone)
      }
      object::object::Object::String(string) => {
        let mut string_clone = string.clone();
        string_clone.remove(0);
        object::object::Object::String(string_clone)
      }
      _ => {
        panic!("rest: argument must be an array");
      }
    }
  } else {
    panic!("rest: too many arguments");
  }
}
