use super::super::lexer;
use super::super::object;
use super::super::parser;
use super::super::tools;
use rand::Rng;
use std::fs;
use std::io;
use std::thread;
use std::env;

#[allow(deprecated)]
pub fn import(
    args: std::vec::Vec<object::object::Object>,
    eval_global: &mut super::super::eval::eval::Evaluator,
) -> object::object::Object {
    if let object::object::Object::String(file_name) = &args[0] {
        let file_name_string: String = file_name.clone();
        let home_path = env::home_dir().unwrap().display().to_string();

        let _lines = match fs::read_to_string(format!("{}/{}", &home_path,  &file_name_string)) {
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

pub fn scanf(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
    let mut _type = "string";

    if args.len() != 1 {
        panic!("get_input: wrong number of arguments");
    }

    if let object::object::Object::String(_str) = &args[0] {
        _type = &_str;
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

pub fn bark(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
    print!(
        "{}",
        args.iter()
            .map(|arg| format!("{} ", arg))
            .collect::<String>()
    );

    object::object::Object::Null
}

pub fn barkln(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
    println!(
        "{}",
        args.iter()
            .map(|arg| format!("{} ", arg))
            .collect::<String>()
    );

    object::object::Object::Null
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
                if let object::object::Object::ReturnValue(value) =
                    eval.statement_evaluator(body.clone())
                {
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

    object::object::Object::Null
}

pub fn random_emojis(args: std::vec::Vec<object::object::Object>) -> object::object::Object {
    let emojis = vec!["🐧", "🦄", "🐝", "🐹", "🐰", "🦊", "🐼", "🐨", "🐯", "🐷"];

    if args.len() == 1 {
        if let object::object::Object::Integer(int) = &args[0] {
            let mut emojis_vec = vec![];
            for _ in 0..*int {
                let index = rand::thread_rng().gen_range(0..emojis.len());
                emojis_vec.push(emojis[index].to_string());
            }

            return object::object::Object::String(emojis_vec.join(" "));
        } else {
            panic!("random_emoji: argument must be an integer");
        }
    } else {
        panic!("random_emoji: too many arguments");
    }
}