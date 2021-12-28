use super::super::ast;
use super::super::lexer;
use super::super::token;

#[derive(Debug)]
pub struct Parser<'a> {
  l: lexer::lexer::Lexer<'a>,
  cur_token: token::token::Token,
  peek_token: token::token::Token,
}

impl<'a> Parser<'a> {
  ////////////////////////////////////////////////////////////////////////////////
  ///
  /// // next_token
  ///
  ////////////////////////////////////////////////////////////////////////////////
  fn next_token(&mut self) {
    self.cur_token = self.peek_token.clone();
    self.peek_token = self.l.next_token();
  }

  // seek the lexer then make ast for that
  pub fn new(l: lexer::lexer::Lexer<'a>) -> Self {
    let mut p = Parser {
      l,
      cur_token: token::token::Token::Illegal,
      peek_token: token::token::Token::Illegal,
    };
    p.next_token();
    p.next_token();
    p
  }

  ////////////////////////////////////////////////////////////////////////////////
  ///
  /// // actual parser
  ///
  ////////////////////////////////////////////////////////////////////////////////

  pub fn program_parser(&mut self) -> ast::ast::Program {
    let mut program = ast::ast::Program::new();

    loop {
      if self.cur_token.clone() == token::token::Token::Eof {
        break;
      } else {
        program.statements.push(self.statement_parser());
        self.next_token();
      }
    }

    program
  }

  fn statement_parser(&mut self) -> ast::ast::Statement {
    // check just a ident or assign
    match self.check_just_a_ident_or_assign() {
      Some(some_stat) => some_stat,
      None => match self.cur_token.clone() {
        token::token::Token::Let => self.let_statement_parser(),
        token::token::Token::Return => self.return_statement_parser(),
        _ => self.expression_statement_parser(),
      },
    }
  }

  fn check_just_a_ident_or_assign(&mut self) -> Option<ast::ast::Statement> {
    if let token::token::Token::Ident(ident) = self.cur_token.clone() {
      if self.peek_token.clone() == token::token::Token::Assign {
        self.next_token();
        self.next_token();
        let value = self.expression_parser(ast::ast::WhichTheBest::Lowest);

        if self.peek_token == token::token::Token::Semicolon {
          self.next_token();
        }

        Some(ast::ast::Statement::Assign {
          identifier: ident,
          value,
        })
      } else {
        None
      }
    } else {
      None
    }
  }

  fn let_statement_parser(&mut self) -> ast::ast::Statement {
    self.next_token();
    let identifier = if let token::token::Token::Ident(identifier) = self.cur_token.clone() {
      ast::ast::Expression::Ident(identifier)
    } else {
      panic!("Expected identifier")
    };
    self.next_token(); // =
    self.next_token(); // value

    let value = self.expression_parser(ast::ast::WhichTheBest::Lowest);

    if self.peek_token == token::token::Token::Semicolon {
      self.next_token();
    }

    ast::ast::Statement::Let { identifier, value }
  }

  fn return_statement_parser(&mut self) -> ast::ast::Statement {
    self.next_token();
    let value = self.expression_parser(ast::ast::WhichTheBest::Lowest);

    if self.peek_token == token::token::Token::Semicolon {
      self.next_token();
    }

    ast::ast::Statement::Return(value)
  }

  fn expression_statement_parser(&mut self) -> ast::ast::Statement {
    let expression = self.expression_parser(ast::ast::WhichTheBest::Lowest);

    if self.peek_token == token::token::Token::Semicolon {
      self.next_token();
    }

    ast::ast::Statement::Expression(expression)
  }

  #[allow(mutable_borrow_reservation_conflict)]
  fn expression_parser(&mut self, precedence: ast::ast::WhichTheBest) -> ast::ast::Expression {
    let mut left_exp = match &self.cur_token {
      token::token::Token::Ident(ident) => {
        if self.peek_token == token::token::Token::LBRacket {
          self.index_parser(ident.clone())
        } else {
          ast::ast::Expression::Ident(ident.clone())
        }
      }
      token::token::Token::String(s) => ast::ast::Expression::String(s.clone()),
      token::token::Token::Integer(value) => ast::ast::Expression::Integer(*value),
      token::token::Token::Double(value) => ast::ast::Expression::Double(*value),
      token::token::Token::LBRacket => self.array_parser(),
      token::token::Token::True => ast::ast::Expression::Bool(true),
      token::token::Token::False => ast::ast::Expression::Bool(false),
      token::token::Token::Bang => self.prefix_parser(),
      token::token::Token::Minus => self.prefix_parser(),
      token::token::Token::Typeof => self.prefix_parser(),
      token::token::Token::LParen => self.g_exp_parser(),
      token::token::Token::If => self.expression_if_parser(),
      token::token::Token::Closure => self.closure_parser(),
      token => {
        panic!("Unexpected left token {:?}", token)
      }
    };

    while self.peek_token != token::token::Token::Semicolon
      && precedence < self.peek_token.which_the_best()
    {
      match self.peek_token {
        token::token::Token::Plus => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        token::token::Token::Minus => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        token::token::Token::Slash => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        token::token::Token::Asterisk => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        token::token::Token::Percent => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        //
        token::token::Token::And => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        token::token::Token::Or => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        //
        token::token::Token::LParen => {
          self.next_token();
          left_exp = self.call_parser(left_exp);
        }
        token::token::Token::Equql => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        token::token::Token::NotEquql => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        //
        token::token::Token::LessThan => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        token::token::Token::GreaterThan => {
          self.next_token();
          left_exp = self.infix_parser(left_exp);
        }
        _ => {
          return left_exp;
        }
      }
    }
    left_exp
  }

  fn g_exp_parser(&mut self) -> ast::ast::Expression {
    self.next_token();
    self.expression_parser(ast::ast::WhichTheBest::Lowest)
  }

  fn expression_if_parser(&mut self) -> ast::ast::Expression {
    self.next_token();
    // if (
    let bools = Box::new(self.expression_parser(ast::ast::WhichTheBest::Lowest));

    self.next_token();
    self.next_token();

    // ) {
    let if_true = Box::new(self.block_parser());

    // } else {
    let else_if = if self.peek_token == token::token::Token::Else {
      self.next_token();
      self.next_token();
      let alt = Box::new(self.block_parser());
      Some(alt)
    } else {
      None
    };
    // }

    ast::ast::Expression::If {
      condition: bools,
      consequence: if_true,
      alternative: else_if,
    }
  }

  fn block_parser(&mut self) -> ast::ast::Statement {
    self.next_token();
    let mut blocks = Vec::new();

    loop {
      if self.cur_token != token::token::Token::RBrace && self.cur_token != token::token::Token::Eof
      {
        blocks.push(self.statement_parser());

        self.next_token();
      } else {
        break;
      }
    }

    ast::ast::Statement::Block(blocks)
  }

  fn array_parser(&mut self) -> ast::ast::Expression {
    let mut elements: Vec<ast::ast::Expression> = Vec::new();

    if self.peek_token != token::token::Token::RBRacket {
      self.next_token();

      loop {
        elements.push(self.expression_parser(ast::ast::WhichTheBest::Lowest));
        if self.peek_token == token::token::Token::Comma {
          self.next_token();
          self.next_token();
        } else {
          break;
        }
      }
    }

    // self.next_token();
    self.next_token();

    ast::ast::Expression::Array { elements }
  }

  fn index_parser(&mut self, ident: String) -> ast::ast::Expression {
    self.next_token();
    self.next_token();

    let index = self.expression_parser(ast::ast::WhichTheBest::Lowest);

    self.next_token();

    ast::ast::Expression::ArrayIndex {
      left_ident: Box::new(ast::ast::Expression::Ident(ident)),
      index: Box::new(index),
    }
  }

  fn assign_parser(&mut self) -> ast::ast::Statement {
    let identifier: String;

    if let token::token::Token::String(_str) = &self.cur_token {
      identifier = _str.clone()
    } else {
      panic!("Expected identifier");
    }
    self.next_token(); // =
    let value = self.expression_parser(ast::ast::WhichTheBest::Lowest);

    ast::ast::Statement::Assign { identifier, value }
  }

  fn closure_parser(&mut self) -> ast::ast::Expression {
    // (
    self.next_token();

    let kakkono_naka = self.func_kakkono_naka_parser();

    self.next_token();
    self.next_token();

    // )
    // {
    let blocks = Box::new(self.block_parser());
    // }
    ast::ast::Expression::Closure {
      parameters: kakkono_naka,
      body: blocks,
    }
  }

  fn func_kakkono_naka_parser(&mut self) -> Vec<ast::ast::Expression> {
    let mut indentifier_vecs = Vec::new();

    if self.peek_token == token::token::Token::RParen {
      indentifier_vecs
    } else {
      self.next_token();
      loop {
        if let token::token::Token::Ident(indentifier) = &self.cur_token {
          indentifier_vecs.push(ast::ast::Expression::Ident(indentifier.clone()));
        } else {
          panic!("Expected identifier");
        }

        if self.peek_token == token::token::Token::Comma {
          self.next_token();
          self.next_token();
        } else {
          break;
        }
      }

      indentifier_vecs
    }
  }

  // (u , n, k, o)

  fn call_parser(&mut self, closure: ast::ast::Expression) -> ast::ast::Expression {
    let mut args = Vec::new();

    if self.peek_token != token::token::Token::RParen {
      self.next_token();
      while {
        args.push(self.expression_parser(ast::ast::WhichTheBest::Lowest));
        self.peek_token == token::token::Token::Comma
      } {
        self.next_token();
        self.next_token();
      }
    }

    self.next_token(); // )
    assert!(
      !(self.cur_token != token::token::Token::RParen),
      "Expected RParen in call expression"
    );
    if self.peek_token == token::token::Token::Semicolon {
      self.next_token();
    }

    ast::ast::Expression::Call {
      closure: Box::new(closure),
      arguments: args,
    }
  }

  // A ?
  fn prefix_parser(&mut self) -> ast::ast::Expression {
    let operator = match self.cur_token {
      token::token::Token::Bang => ast::ast::Prefix::Bang,
      token::token::Token::Minus => ast::ast::Prefix::Minus,
      token::token::Token::Typeof => ast::ast::Prefix::Typeof,
      _ => {
        panic!("Unexpected token {:?}", self.cur_token)
      }
    };

    self.next_token();

    let right = Box::new(self.expression_parser(ast::ast::WhichTheBest::Prefix));

    ast::ast::Expression::Prefix {
      op: operator,
      right,
    }
  }

  // A ? B
  fn infix_parser(&mut self, left: ast::ast::Expression) -> ast::ast::Expression {
    let op = match self.cur_token {
      token::token::Token::Plus => ast::ast::Infix::Plus,
      token::token::Token::Minus => ast::ast::Infix::Minus,
      token::token::Token::Asterisk => ast::ast::Infix::Asterisk,
      token::token::Token::Slash => ast::ast::Infix::Slash,
      token::token::Token::And => ast::ast::Infix::And,
      token::token::Token::Or => ast::ast::Infix::Or,
      token::token::Token::Percent => ast::ast::Infix::Percent,
      token::token::Token::Equql => ast::ast::Infix::Eq,
      token::token::Token::NotEquql => ast::ast::Infix::NotEq,
      token::token::Token::LessThan => ast::ast::Infix::LessThan,
      token::token::Token::GreaterThan => ast::ast::Infix::GreaterThan,
      _ => {
        panic!("Unexpected token {:?}", self.cur_token)
      }
    };

    let _best = self.cur_token.which_the_best();
    self.next_token();
    let right = Box::new(self.expression_parser(_best));

    ast::ast::Expression::Infix {
      op,
      left: Box::new(left),
      right,
    }
  }
}

#[test]
fn test_parse() {
  let _lines = "🍙 x = 5;
  🍄🍄 ( \"welcome to nylang!\" ) ;
  ";
  let l = lexer::lexer::Lexer::new(_lines);
  let mut p = Parser::new(l);
  p.program_parser();
}
