// define tokens
use super::super::ast;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token {
  // exceptions
  Illegal,
  EOF,

  // identifiers and literals
  Ident(String),
  Integer(i32),
  String(String),

  // operators
  Closure,
  Let,
  // keywords
  Assign,
  Equql,
  NotEquql,

  // operators
  Plus,
  Minus,
  Bang,
  Asterisk,
  Slash,
  Percent,

  // delimiters
  LessThan,
  GreaterThan,
  Comma,
  Semicolon,
  LParen,
  RParen,
  LBrace,
  RBrace,

  // OR and AND
  Or,
  And,

  // for array
  LBRacket,
  RBRacket,

  // if else
  If,
  Else,
  Return,

  // boolean
  True,
  False,
}

impl Token {
  // get indentifier string and then return token
  pub fn ident_to_token(ident: &str) -> Option<Token> {
    match ident {
      "🏨" => Some(Token::Closure),
      "🍙" => Some(Token::Let),
      "👍" => Some(Token::True),
      "👎" => Some(Token::False),
      "🐶" => Some(Token::If),
      "😱" => Some(Token::Else),
      "💨" => Some(Token::Return),
      _ => None,
    }
  }

  pub fn which_the_best(&self) -> ast::ast::WhichTheBest {
    match self {
      Token::And => ast::ast::WhichTheBest::AndOr,
      Token::Or => ast::ast::WhichTheBest::AndOr,
      Token::Equql => ast::ast::WhichTheBest::Equals,
      Token::NotEquql => ast::ast::WhichTheBest::Equals,
      Token::LessThan => ast::ast::WhichTheBest::LessGreater,
      Token::GreaterThan => ast::ast::WhichTheBest::LessGreater,
      Token::Plus => ast::ast::WhichTheBest::Sum,
      Token::Minus => ast::ast::WhichTheBest::Sum,
      Token::Slash => ast::ast::WhichTheBest::Product,
      Token::Asterisk => ast::ast::WhichTheBest::Product,
      Token::Percent => ast::ast::WhichTheBest::Product,
      Token::LParen => ast::ast::WhichTheBest::Call,
      _ => ast::ast::WhichTheBest::Lowest,
    }
  }
}
