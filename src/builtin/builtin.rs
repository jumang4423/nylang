use super::super::object;
use std::thread;

pub fn bark(args: std::vec::Vec<object::object::Object>) {
    print!("{}", args.iter().map(|arg| format!("{} ", arg)).collect::<String>());
}

pub fn barkln(args: std::vec::Vec<object::object::Object>) {
    println!("{}", args.iter().map(|arg| format!("{} ", arg)).collect::<String>());
}

pub fn sleep(args: std::vec::Vec<object::object::Object>) {
    if let object::object::Object::Integer(seconds) = args[0] {
        thread::sleep_ms(seconds as u32);
    } else {
        panic!("night: argument must be an integer");
    }
}

pub fn looper(args: std::vec::Vec<object::object::Object>, eval: &mut super::super::eval::eval::Evaluator) {
    if let object::object::Object::Closure{ parameters, body, env } = &args[0] {

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
        } else {
            if let object::object::Object::Integer(int) = &args[1] {
                for _ in 0..*int {
                    eval.statement_evaluator(body.clone());
                }
            } else {
                panic!("night: argument must be an integer");
            }
        }        
    } else {
        panic!("loop: first argument must be a function");
    }
}