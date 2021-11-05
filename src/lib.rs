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
pub fn excute_nyl(_lines: String) -> Vec<JsValue> {
  let lines = _lines + "main() ;";
  let mut env = eval::eval::Evaluator::new();
  let l = lexer::lexer::Lexer::new(lines.as_str());
  let mut p = parser::parser::Parser::new(l);
  let program = p.program_parser();
  env.evaluator(program);

  // []string -> Vec<JsValue>
  let mut result = Vec::new();
  for value in env.outputs.iter() {
    result.push(JsValue::from_str(value));
  }
  result
}