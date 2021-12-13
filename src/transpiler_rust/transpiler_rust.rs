use super::super::ast;

pub struct TranspilerRust {
  pub outputs: String,
}

const HEADER_COMMENTS: &str = "
// 🐶 This file is generated by the nylang_to_rust_transpiler 🐶
// -! need to add proconio crate in cargo.toml

// lib
use proconio::input;

";

impl TranspilerRust {
  pub fn new() -> TranspilerRust {
    TranspilerRust {
      outputs: String::from(HEADER_COMMENTS),
    }
  }

  pub fn transpile(&mut self, statements: ast::ast::Program) -> String {
    for statement in statements.statements.iter() {
      let evaluated_statement: String = self.statement_to_rust(statement.clone());
      // output + evaluated_statement + "\n"
      self.outputs = format!("{}{}\n", self.outputs, evaluated_statement);
    }

    self.outputs.clone()
  }

  pub fn statement_to_rust(&mut self, statement: ast::ast::Statement) -> String {
    match statement {
      ast::ast::Statement::Let { identifier, value } => {
        if let ast::ast::Expression::Ident(stringified_identifier) = identifier {
          // if let main, translate to fn main
          if stringified_identifier == "main" {
            // get call from value
            if let ast::ast::Expression::Closure {
              parameters: _,
              body,
            } = value
            {
              if let ast::ast::Statement::Block(statements) = *body {
                return format!("fn main() {{\n{}}}", self.block_transpiler(statements));
              } else {
                panic!("Expected block");
              }
            } else {
              panic!("Expected closure");
            }
          } else {
            return format!(
              "let {} = {};",
              stringified_identifier,
              self.expression_to_rust(value.clone())
            );
          }
        } else {
          panic!("Identifier is not an Identifier");
        }
      }
      ast::ast::Statement::Expression(expression) => self.expression_to_rust(expression.clone()),
      ast::ast::Statement::Block(statements) => self.block_transpiler(statements.clone()),
      _ => panic!("Statement is not a Statement"),
    }
  }

  pub fn expression_to_rust(&mut self, expression: ast::ast::Expression) -> String {
    match expression {
      ast::ast::Expression::Ident(identifier) => format!("{}", ident_to_rust(identifier)),
      ast::ast::Expression::Integer(integer) => format!("{}", integer),
      ast::ast::Expression::Double(double) => format!("{}", double),
      ast::ast::Expression::Bool(boolean) => format!("{:?}", boolean),
      ast::ast::Expression::String(string) => format!("\"{}\".to_string()", string),
      // ast::ast::Expression::Array(array) => self.array_evaluator(array),
      ast::ast::Expression::Closure { parameters, body } => {
        self.closure_transpiler(parameters, body)
      }
      ast::ast::Expression::Call { closure, arguments } => self.call_transpiler(closure, arguments),
      ast::ast::Expression::If {
        condition,
        consequence,
        alternative,
      } => self.if_transpiler(condition, consequence, alternative),
      ast::ast::Expression::Prefix { op, right } => self.prefix_transpiler(op, right),
      ast::ast::Expression::Infix { left, op, right } => self.infix_transpiler(left, op, right),
      _ => panic!("Expression is not an Expression"),
    }
  }

  pub fn call_transpiler(
    &mut self,
    closure: Box<ast::ast::Expression>,
    arguments: Vec<ast::ast::Expression>,
  ) -> String {
    // unbox closure then call
    let closure_string: String = self.expression_to_rust(*closure);
    let arguments_string: String = self.arguments_to_rust(arguments.clone());

    match self.check_builtin(&closure_string, arguments.clone()) {
      Some(_str) => _str,
      None => format!("{}({})", closure_string, arguments_string),
    }
  }

  pub fn arguments_to_rust(&mut self, arguments: Vec<ast::ast::Expression>) -> String {
    // output will be "1, 2, 3"
    let mut arguments_string = String::new();

    for (index, argument) in arguments.iter().enumerate() {
      if index != 0 {
        arguments_string = format!("{}, ", arguments_string);
      }

      arguments_string = format!(
        "{}{}",
        arguments_string,
        self.expression_to_rust(argument.clone())
      );
    }

    arguments_string
  }

  pub fn closure_transpiler(
    &mut self,
    parameters: std::vec::Vec<ast::ast::Expression>,
    body: std::boxed::Box<ast::ast::Statement>,
  ) -> String {
    format!(
      "|{}| {{\n{}\n}}",
      self.arguments_to_rust(parameters),
      self.statement_to_rust(*body)
    )
    .to_string()
  }

  pub fn block_transpiler(&mut self, statements: std::vec::Vec<ast::ast::Statement>) -> String {
    let mut block_string = String::new();

    for statement in statements.iter() {
      block_string = format!(
        "{}{};\n",
        block_string,
        self.statement_to_rust(statement.clone())
      );
    }

    block_string
  }

  pub fn if_transpiler(
    &mut self,
    condition: Box<ast::ast::Expression>,
    consequence: Box<ast::ast::Statement>,
    alternative: Option<Box<ast::ast::Statement>>,
  ) -> String {
    let condition_string: String = self.expression_to_rust(*condition);
    let consequence_string: String = self.statement_to_rust(*consequence);

    match alternative {
      Some(alternative) => format!(
        "if {} {{\n{}\n}} else {{\n{}\n}}",
        condition_string,
        consequence_string,
        self.statement_to_rust(*alternative)
      ),
      None => format!("if {} {{\n{}\n}}", condition_string, consequence_string),
    }
  }

  pub fn prefix_transpiler(
    &mut self,
    op: ast::ast::Prefix,
    right: Box<ast::ast::Expression>,
  ) -> String {
    match (op, right) {
      (ast::ast::Prefix::Minus, right) => format!("-{}", self.expression_to_rust(*right)),
      (ast::ast::Prefix::Bang, right) => format!("!{}", self.expression_to_rust(*right)),
      _ => panic!("Prefix is not a Prefix"),
    }
  }

  pub fn infix_transpiler(
    &mut self,
    left: Box<ast::ast::Expression>,
    op: ast::ast::Infix,
    right: Box<ast::ast::Expression>,
  ) -> String {
    match (op, *left.clone(), *right.clone()) {
      // infix op infix
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!(
          "{} + {}",
          self.expression_to_rust(*left),
          self.expression_to_rust(*right)
        )
      }
      (
        ast::ast::Infix::Minus,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!(
          "{} - {}",
          self.expression_to_rust(*left),
          self.expression_to_rust(*right)
        )
      }
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!(
          "{} / {}",
          self.expression_to_rust(*left),
          self.expression_to_rust(*right)
        )
      }
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!(
          "{} * {}",
          self.expression_to_rust(*left),
          self.expression_to_rust(*right)
        )
      }
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!(
          "{} == {}",
          self.expression_to_rust(*left),
          self.expression_to_rust(*right)
        )
      }
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!(
          "{} != {}",
          self.expression_to_rust(*left),
          self.expression_to_rust(*right)
        )
      }
      (
        ast::ast::Infix::LessThan,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!(
          "{} < {}",
          self.expression_to_rust(*left),
          self.expression_to_rust(*right)
        )
      }
      (
        ast::ast::Infix::GreaterThan,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!(
          "{} > {}",
          self.expression_to_rust(*left),
          self.expression_to_rust(*right)
        )
      }
      // infix op type
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Integer(right),
      ) => {
        format!("{} + {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Minus,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Integer(right),
      ) => {
        format!("{} - {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Integer(right),
      ) => {
        format!("{} / {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Integer(right),
      ) => {
        format!("{} * {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Integer(right),
      ) => {
        format!("{} == {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Integer(right),
      ) => {
        format!("{} != {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::LessThan,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Integer(right),
      ) => {
        format!("{} < {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::GreaterThan,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Integer(right),
      ) => {
        format!("{} > {}", self.expression_to_rust(*left), right)
      }
      // type op infix
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} + {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Minus,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} - {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} / {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} * {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} == {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} != {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::LessThan,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} < {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::GreaterThan,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} > {}", left, self.expression_to_rust(*right))
      }
      // infix op type
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Ident(right),
      ) => {
        format!("{} + &{}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Minus,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Ident(right),
      ) => {
        format!("{} - &{}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Ident(right),
      ) => {
        format!("{} / &{}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Ident(right),
      ) => {
        format!("{} * &{}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Ident(right),
      ) => {
        format!("{} == &{}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Ident(right),
      ) => {
        format!("{} != &{}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::LessThan,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Ident(right),
      ) => {
        format!("{} < &{}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::GreaterThan,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::Ident(right),
      ) => {
        format!("{} > &{}", self.expression_to_rust(*left), right)
      }
      // type op infix
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("&{} + {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Minus,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("&{} - {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("&{} / {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("&{} * {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("&{} == {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("&{} != {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::LessThan,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("&{} < {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::GreaterThan,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("&{} > {}", left, self.expression_to_rust(*right))
      }

      // infix op type
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::String(right),
      ) => {
        format!("{} + {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Minus,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::String(right),
      ) => {
        format!("{} - {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::String(right),
      ) => {
        format!("{} / {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::String(right),
      ) => {
        format!("{} * {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::String(right),
      ) => {
        format!("{} == {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::String(right),
      ) => {
        format!("{} != {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::LessThan,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::String(right),
      ) => {
        format!("{} < {}", self.expression_to_rust(*left), right)
      }
      (
        ast::ast::Infix::GreaterThan,
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
        ast::ast::Expression::String(right),
      ) => {
        format!("{} > {}", self.expression_to_rust(*left), right)
      }
      // type op infix
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::String(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} + {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Minus,
        ast::ast::Expression::String(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} - {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::String(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} / {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::String(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} * {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::String(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} == {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::String(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} != {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::LessThan,
        ast::ast::Expression::String(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} < {}", left, self.expression_to_rust(*right))
      }
      (
        ast::ast::Infix::GreaterThan,
        ast::ast::Expression::String(left),
        ast::ast::Expression::Infix {
          left: _,
          op: _,
          right: _,
        },
      ) => {
        format!("{} > {}", left, self.expression_to_rust(*right))
      }

      // int
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} + {}", left, right),
      (
        ast::ast::Infix::Minus,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} - {}", left, right),
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} * {}", left, right),
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} / {}", left, right),
      (
        ast::ast::Infix::Percent,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} % {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} != {}", left, right),
      (
        ast::ast::Infix::LessThan,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} < {}", left, right),
      (
        ast::ast::Infix::GreaterThan,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} > {}", left, right),
      // double
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::Double(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} + {}", left, right),
      (
        ast::ast::Infix::Minus,
        ast::ast::Expression::Double(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} - {}", left, right),
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::Double(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} * {}", left, right),
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::Double(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} / {}", left, right),
      (
        ast::ast::Infix::Percent,
        ast::ast::Expression::Double(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} % {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Double(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Double(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} != {}", left, right),
      (
        ast::ast::Infix::LessThan,
        ast::ast::Expression::Double(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} < {}", left, right),
      (
        ast::ast::Infix::GreaterThan,
        ast::ast::Expression::Double(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} > {}", left, right),
      // string
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::String(left),
        ast::ast::Expression::String(right),
      ) => format!("format!(\"{{}}{{}}\", \"{}\", \"{}\")", left, right),
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::String(left),
        ast::ast::Expression::Ident(right),
      ) => format!("format!(\"{{}}{{}}\", \"{}\", {})", left, right),
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::String(right),
      ) => format!("format!(\"{{}}{{}}\", {}, \"{}\")", left, right),
      (
        ast::ast::Infix::Plus,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Ident(right),
      ) => format!("{} + {}", left, right),
      // bool
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Bool(left),
        ast::ast::Expression::Bool(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Bool(left),
        ast::ast::Expression::Bool(right),
      ) => format!("{} != {}", left, right),
      // ident
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} != {}", left, right),
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} != {}", left, right),
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::String(right),
      ) => format!("{} != {}", left, right),
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Bool(right),
      ) => format!("{} != {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} * {}", left, right),
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} / {}", left, right),
      (
        ast::ast::Infix::Percent,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Integer(right),
      ) => format!("{} % {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::Asterisk,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} * {}", left, right),
      (
        ast::ast::Infix::Slash,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} / {}", left, right),
      (
        ast::ast::Infix::Percent,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Double(right),
      ) => format!("{} % {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::String(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Bool(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::NotEq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Ident(right),
      ) => format!("{} != {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Ident(left),
        ast::ast::Expression::Ident(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Bool(left),
        ast::ast::Expression::Ident(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Integer(left),
        ast::ast::Expression::Ident(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::Double(left),
        ast::ast::Expression::Ident(right),
      ) => format!("{} == {}", left, right),
      (
        ast::ast::Infix::Eq,
        ast::ast::Expression::String(left),
        ast::ast::Expression::Ident(right),
      ) => format!("{} == {}", left, right),
      _ => format!("[-! Infix is not an Infix]"),
    }
  }

  pub fn check_builtin(
    &mut self,
    closure_string: &String,
    arguments: Vec<ast::ast::Expression>,
  ) -> Option<String> {
    match closure_string.as_str() {
      "🎤" => Some(format!(
        "print!(\"{{}}\", {})",
        self.expression_to_rust(arguments[0].clone())
      )),
      "🎤🎶" => Some(format!(
        "println!(\"{{}}\", {})",
        self.expression_to_rust(arguments[0].clone())
      )),
      "👀" => {
        let mut var_type: String = String::new();
        if let ast::ast::Expression::String(string) = arguments[0].clone() {
          var_type = match string.as_str() {
            "number" => "i32".to_string(),
            "string" => "String".to_string(),
            _ => "String".to_string(),
          };
        }
        Some(
          format!(
            "|| -> {} {{ 
// 🐶 reading input
proconio::input!(
  input: {}
); input
 }}()",
            var_type, var_type
          )
          .to_string(),
        )
      }
      // comment out
      "🍄🍄" => {
        if let ast::ast::Expression::String(string) = arguments[0].clone() {
          Some(format!("// {}", string))
        } else {
          None
        }
      }
      _ => None,
    }
  }
}

pub fn ident_to_rust(ident: String) -> String {
  match ident.as_str() {
    _ => ident,
  }
}
