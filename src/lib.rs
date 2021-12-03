use regex_syntax::ast::*;

pub fn explode(pattern: &str) -> Result<Vec<String>, Error> {
  let ast = parse::Parser::new().parse(pattern)?;
  Ok(convert(&ast))
}

fn convert(ast: &Ast) -> Vec<String> {
  unimplemented!()
}
