use super::super::ast;

pub struct TranspilerRust {
  pub outputs: String,
}

impl TranspilerRust {
  pub fn new() -> TranspilerRust {
    TranspilerRust {
      outputs: String::new(),
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
            if let ast::ast::Expression::Closure { parameters: _, body } = value {
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
      ast::ast::Expression::Bool(boolean) => format!("{}", boolean),
      ast::ast::Expression::String(string) => format!("\"{}\"", string),
      // ast::ast::Expression::Array(array) => self.array_evaluator(array),
      ast::ast::Expression::Closure { parameters, body } => {
        self.closure_transpiler(parameters, body)
      }
      ast::ast::Expression::Call { closure, arguments } => self.call_transpiler(closure, arguments),
      _ => panic!("Expression is not an Expression"),
    }
  }

  pub fn call_transpiler(
    &mut self,
    closure: Box<ast::ast::Expression>,
    arguments: Vec<ast::ast::Expression>,
  ) -> String {
    // unbox closure then call
    let closure_string = self.expression_to_rust(*closure);
    let arguments_string = self.arguments_to_rust(arguments);

    format!("{}({})", closure_string, arguments_string)
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
}

pub fn ident_to_rust(ident: String) -> String {
  match ident.as_str() {
    "🎤" => "print!".to_string(),
    "🎤🎶" => "println!".to_string(),
    _ => ident,
  }
}
