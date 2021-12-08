use super::super::ast;

pub struct Transpiler_rust {
  pub ast: ast::ast::Program,
  pub outputs: String
}

impl Transpiler_rust {
  pub fn new(ast: ast::ast::Program) -> Transpiler_rust {
    Transpiler_rust {
      ast: ast,
      outputs: String::new()
    }
  }

  pub fn transpile(&mut self) {

    for statement in self.ast.statements.iter() {
      statement.transpile(self);
    }

  }

}