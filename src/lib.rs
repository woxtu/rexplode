use regex_syntax::ast::*;

pub fn explode(pattern: &str) -> Result<Vec<String>, Error> {
  let ast = parse::Parser::new().parse(pattern)?;
  Ok(convert(&ast))
}

fn convert(ast: &Ast) -> Vec<String> {
  use Ast::*;

  match ast {
    Empty(_) => vec![],
    Flags(_) | Dot(_) | Assertion(_) => vec![ast.to_string()],
    Literal(literal) => vec![format_literal(literal)],
    _ => unimplemented!(),
  }
}

fn format_literal(Literal { c, .. }: &Literal) -> String {
  c.to_string()
}
