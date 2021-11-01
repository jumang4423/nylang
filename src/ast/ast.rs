// ast objects

#[derive(Clone, Debug)]
pub struct Program {
  pub statements: Vec<Statement>,
}

impl Program {
  // no program statements constructor
  pub fn new() -> Self {
    Self {
      statements: Vec::new(),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Statement {
  // let i = v
  Let {
    identifier: Expression,
    value: Expression,
  },
  // return e
  Return(Expression),
  // e
  Expression(Expression),
  // { []s }
  Block(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub enum Expression {
  Ident(String),
  String(String),
  Integer(i32),
  Bool(bool),
  If {
    condition: Box<Expression>,
    consequence: Box<Statement>,
    alternative: Option<Box<Statement>>,
  },
  Array {
    elements: Vec<Expression>,
  },
  ArrayIndex {
    left_ident: Box<Expression>,
    index: Box<Expression>,
  },
  Closure {
    parameters: Vec<Expression>,
    body: Box<Statement>,
  },
  Call {
    closure: Box<Expression>,
    arguments: Vec<Expression>,
  },
  Prefix {
    op: Prefix,
    right: Box<Expression>,
  },
  Infix {
    op: Infix,
    left: Box<Expression>,
    right: Box<Expression>,
  },
}

// operators

#[derive(Copy, Debug, Clone)]
pub enum Prefix {
  Bang,
  Minus,
  Typeof
}

#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub enum Infix {
  Plus,
  Minus,
  Asterisk,
  Slash,
  And,
  Or,
  Percent,
  Eq,
  NotEq,
  LessThan,
  GreaterThan,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub enum WhichTheBest {
  Lowest,
  AndOr,
  Equals,
  LessGreater,
  Sum,
  Product,
  Prefix,
  Call,
}
