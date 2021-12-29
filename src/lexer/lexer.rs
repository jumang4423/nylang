use super::super::token;

const ZERO_CHAR: char = '\u{0}';

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
  input: std::str::Chars<'a>,
  cur_char: char,
  peek_char: char,
}

fn is_letter(c: char) -> bool {
  ('a'..='z').contains(&c)
    || ('A'..='Z').contains(&c)
    || c == '_'
    || c == '🏨'
    || c == '🍙'
    || c == '👍'
    || c == '👎'
    || c == '🐶'
    || c == '😱'
    || c == '💨'
    || c == '😪'
    || c == '🎤'
    || c == '🎶'
    || c == '🌸'
    || c == '🌹'
    || c == '👀'
    || c == '🐽'
    || c == '🍄'
    || c == '📏'
    || c == '🥌'
    || c == '🌛'
    || c == '❌'
    || c == '🥚'
    || c == '🗿'
    || c == '👽'
}

impl<'a> Lexer<'a> {
  // Constructor
  pub fn new(input: &'a str) -> Self {
    let mut lexer = Lexer {
      input: input.chars(),
      cur_char: ZERO_CHAR,
      peek_char: ZERO_CHAR,
    };
    lexer.read_char();
    lexer.read_char();
    lexer
  }

  pub fn next_token(&mut self) -> token::token::Token {
    self.skip_whitespace();
    let token = match self.cur_char {
      '=' => {
        if self.peek_char == '=' {
          self.read_char();
          token::token::Token::Equql
        } else {
          token::token::Token::Assign
        }
      }
      '&' => {
        if self.peek_char == '&' {
          self.read_char();
          token::token::Token::And
        } else {
          token::token::Token::And
        }
      }
      '|' => {
        if self.peek_char == '|' {
          self.read_char();
          token::token::Token::Or
        } else {
          token::token::Token::Or
        }
      }
      '+' => token::token::Token::Plus,
      '-' => token::token::Token::Minus,
      '!' => {
        if self.peek_char == '=' {
          self.read_char();
          token::token::Token::NotEquql
        } else {
          token::token::Token::Bang
        }
      }
      '*' => token::token::Token::Asterisk,
      '/' => token::token::Token::Slash,
      '%' => token::token::Token::Percent,
      '<' => token::token::Token::LessThan,
      '>' => token::token::Token::GreaterThan,
      ';' => token::token::Token::Semicolon,
      '(' => token::token::Token::LParen,
      ')' => token::token::Token::RParen,
      '[' => token::token::Token::LBRacket,
      ']' => token::token::Token::RBRacket,
      ',' => token::token::Token::Comma,
      '{' => token::token::Token::LBrace,
      '}' => token::token::Token::RBrace,
      '"' => self.check_string(),
      ZERO_CHAR => token::token::Token::Eof,
      char => {
        if is_letter(self.cur_char) {
          return self.check_identifier();
        } else if char.is_numeric() {
          return self.check_number();
        } else {
          token::token::Token::Illegal
        }
      }
    };
    self.read_char();
    token
  }

  ////////////////////////////////////////////////////////////////////////////////
  ///
  /// // Methods
  ///
  ////////////////////////////////////////////////////////////////////////////////

  fn skip_whitespace(&mut self) {
    while self.cur_char == ' '
      || self.cur_char == '\t'
      || self.cur_char == '\n'
      || self.cur_char == '\r'
    {
      self.read_char();
    }
  }

  //
  fn read_char(&mut self) -> char {
    let return_char = self.cur_char;
    self.cur_char = self.peek_char;
    self.peek_char = match self.input.next() {
      Some(c) => c,
      None => ZERO_CHAR,
    };
    return_char
  }

  ////////////////////////////////////////////////////////////////////////////////
  ///
  /// // read and check tokens are valid
  ///
  ////////////////////////////////////////////////////////////////////////////////
  fn check_identifier(&mut self) -> token::token::Token {
    let mut identifier = String::new();
    while is_letter(self.cur_char) {
      identifier.push(self.read_char());
    }

    match token::token::Token::ident_to_token(&identifier) {
      Some(t) => t,
      None => token::token::Token::Ident(identifier),
    }
  }

  fn check_number(&mut self) -> token::token::Token {
    let mut number = String::new();
    let mut is_double = false;

    while self.cur_char.is_ascii_digit() || self.cur_char == '.' {
      if self.cur_char == '.' {
        is_double = true;
      }
      number.push(self.read_char());
    }

    match is_double {
      true => token::token::Token::Double(number.parse::<f64>().unwrap()),
      false => token::token::Token::Integer(number.parse().unwrap()),
    }
  }

  fn check_string(&mut self) -> token::token::Token {
    let mut string = String::new();
    self.read_char();
    while self.cur_char != '"' && self.peek_char != ZERO_CHAR {
      string.push(self.read_char());
    }
    token::token::Token::String(string) // WEIRD
  }
}

#[test]
fn test_lexer() {
  let input = "
🍙 five = 5 ;
🍄🍄 ( \"welcome to nylang!\" ) ;
🐶 ( animal == \"cat\" ) { 💨 👍 ; } 😱 { 💨 👎 ; }  } ;";
  let mut lexer = Lexer::new(input);
  let mut tokens = vec![];
  while lexer.cur_char != ZERO_CHAR {
    tokens.push(lexer.next_token());
  }
  tokens.push(token::token::Token::Eof);
  assert_eq!(
    tokens,
    vec![
      token::token::Token::Let,
      token::token::Token::Ident("five".to_string()),
      token::token::Token::Assign,
      token::token::Token::Integer(5),
      token::token::Token::Semicolon,
      token::token::Token::Ident("🍄🍄".to_string()),
      token::token::Token::LParen,
      token::token::Token::String("welcome to nylang!".to_string()),
      token::token::Token::RParen,
      token::token::Token::Semicolon,
      token::token::Token::If,
      token::token::Token::LParen,
      token::token::Token::Ident("animal".to_string()),
      token::token::Token::Equql,
      token::token::Token::String("cat".to_string()),
      token::token::Token::RParen,
      token::token::Token::LBrace,
      token::token::Token::Return,
      token::token::Token::True,
      token::token::Token::Semicolon,
      token::token::Token::RBrace,
      token::token::Token::Else,
      token::token::Token::LBrace,
      token::token::Token::Return,
      token::token::Token::False,
      token::token::Token::Semicolon,
      token::token::Token::RBrace,
      token::token::Token::RBrace,
      token::token::Token::Semicolon,
      token::token::Token::Eof,
    ]
  );
}

#[test]
fn test_call_func() {
  let input = "🎤 ( \"ass\" ) ;";
  let mut lexer = Lexer::new(input);
  let mut tokens = vec![];
  while lexer.cur_char != ZERO_CHAR {
    tokens.push(lexer.next_token());
  }
  tokens.push(token::token::Token::Eof);
  assert_eq!(
    tokens,
    vec![
      token::token::Token::Ident("🎤".to_string()),
      token::token::Token::LParen,
      token::token::Token::String("ass".to_string()),
      token::token::Token::RParen,
      token::token::Token::Semicolon,
      token::token::Token::Eof

    ]
  );
}