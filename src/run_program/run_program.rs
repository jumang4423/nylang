use colored::*;

use std::io::{self};

use super::super::eval;
use super::super::lexer;
use super::super::parser;
use super::super::token;

pub fn run_program(mut _lines: String) -> Result<(), io::Error> {

  _lines = _lines + "main() ;";

  // let blue_color_res: Result<Color, ()> = "magenta".parse();
  // println!(
  //   "\n-! {}\n",
  //   "excuting program"
  //     .color(blue_color_res.unwrap_or(Color::Green))
  //     .bold()
  // );

  let mut env = eval::eval::Evaluator::new();
  let l = lexer::lexer::Lexer::new(_lines.as_str());
  let mut p = parser::parser::Parser::new(l);
  let program = p.program_parser();
  env.evaluator(program);

  // println!(
  //   "\n-! {}",
  //   "finished"
  //     .color(blue_color_res.unwrap_or(Color::Green))
  //     .bold()
  // );

  Ok(())
}

pub fn parse_program(mut _lines: String) -> Result<(), io::Error> {
 
  _lines = _lines + "main() ;";

  let blue_color_res: Result<Color, ()> = "magenta".parse();
  println!(
    "\n-! {}\n",
    "builging ast tree..."
      .color(blue_color_res.unwrap_or(Color::Green))
      .bold()
  );

  let l = lexer::lexer::Lexer::new(_lines.as_str());
  let mut p = parser::parser::Parser::new(l);
  let program = p.program_parser();

  let mut _line_cnt: i32 = 1;

  program.statements.iter().for_each(|statement| {
    println!("{}: {:?}\n", _line_cnt, statement);
    _line_cnt += 1;
  });

  println!(
    "\n-! {}",
    "finished"
      .color(blue_color_res.unwrap_or(Color::Green))
      .bold()
  );

  Ok(())
}

pub fn lexer_program(mut _lines: String) -> Result<(), io::Error> {
 
  _lines = _lines + "main() ;";
  let mut l = lexer::lexer::Lexer::new(_lines.as_str());
  let mut tok = l.next_token();

  let blue_color_res: Result<Color, ()> = "magenta".parse();
  println!(
    "\n-! {}\n",
    "parse started"
      .color(blue_color_res.unwrap_or(Color::Green))
      .bold()
  );

  loop {
    match &tok {
      token::token::Token::Eof => break,
      _ => println!("{:?}", tok),
    }

    tok = l.next_token();
  }

  println!(
    "\n-! {}",
    "finished"
      .color(blue_color_res.unwrap_or(Color::Green))
      .bold()
  );

  Ok(())
}
