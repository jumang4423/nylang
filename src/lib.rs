mod ast;
mod builtin;
mod enve;
mod eval;
mod lexer;
mod object;
mod parser;
mod token;
mod tools;

use wasm_bindgen::prelude::*;

// excute program then return array of result
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn excute_nyl(_lines: String, mode: i32) -> Vec<JsValue> {
  // show ast
  if mode == 0 {
    // put main func implictly
    let lines = _lines + "main() ;";
    // start compute
    let mut l = lexer::lexer::Lexer::new(lines.as_str());

    let mut tokens: Vec<String> = [].to_vec();
    loop {
      let token = l.next_token();
      if token == token::token::Token::Eof {
        break;
      }
      tokens.push(format!("{:?}", token));
    }

    // translate result to JsValue
    let mut result: Vec<JsValue> = Vec::new();
    for value in tokens.iter() {
      result.push(JsValue::from_str(value));
    }
    result
  } else if mode == 1 {
    let lines = _lines + "main() ;";
    let l = lexer::lexer::Lexer::new(lines.as_str());
    let mut p = parser::parser::Parser::new(l);
    let program = p.program_parser();
    let mut asts: Vec<String> = [].to_vec();
    for statement in program.statements.iter() {
      asts.push(format!("{:?}", statement));
    }
    // []string -> Vec<JsValue>
    let mut result = Vec::new();
    for value in asts.iter() {
      result.push(JsValue::from_str(value));
    }
    result

  // show output
  } else if mode == 2 {
    // put main func implictly
    let lines = _lines + "main() ;";

    // start compute
    let l = lexer::lexer::Lexer::new(lines.as_str());
    let mut p = parser::parser::Parser::new(l);
    let program = p.program_parser();
    let mut env = eval::eval::Evaluator::new();
    env.evaluator(program);

    // translate result to JsValue
    let mut result: Vec<JsValue> = Vec::new();
    for value in env.outputs.iter() {
      result.push(JsValue::from_str(value));
    }
    result
  } else {
    // mode is invalid
    let mut result = Vec::new();
    result.push(JsValue::from_str("mode is invalid"));
    result
  }
}
