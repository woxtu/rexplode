mod ast;

use ast::ToString;
use itertools::Itertools;
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
    Literal(literal) => vec![literal.to_string()],
    Class(class) => convert_class(class),
    Repetition(repetition) => convert_repetition(repetition),
    Group(group) => convert_group(group),
    Alternation(alternation) => convert_alternation(alternation),
    Concat(concat) => convert_concat(concat),
  }
}

fn convert_class(class: &Class) -> Vec<String> {
  use Class::*;

  match class {
    Unicode(unicode) => vec![unicode.to_string()],
    Perl(perl) => vec![perl.to_string()],
    Bracketed(bracketed) => convert_class_bracketed(bracketed),
  }
}

fn convert_class_bracketed(bracketed @ ClassBracketed { negated, kind, .. }: &ClassBracketed) -> Vec<String> {
  if *negated {
    vec![bracketed.to_string()]
  } else {
    convert_class_set(kind)
  }
}

fn convert_class_set(set: &ClassSet) -> Vec<String> {
  use ClassSet::*;

  match set {
    Item(item) => convert_class_set_item(item),
    BinaryOp(op) => convert_class_set_binary_op(op),
  }
}

fn convert_class_set_item(item: &ClassSetItem) -> Vec<String> {
  use ClassSetItem::*;

  match item {
    Empty(_) | Literal(_) | Ascii(_) | Unicode(_) | Perl(_) => vec![item.to_string()],
    Range(range) => convert_class_set_range(range),
    Bracketed(bracketed) => convert_class_bracketed(bracketed),
    Union(union) => convert_class_set_union(union),
  }
}

fn convert_class_set_range(ClassSetRange { start, end, .. }: &ClassSetRange) -> Vec<String> {
  (start.c..=end.c).map(|c| c.to_string()).collect()
}

fn convert_class_set_union(ClassSetUnion { items, .. }: &ClassSetUnion) -> Vec<String> {
  items.iter().flat_map(convert_class_set_item).collect()
}

fn convert_class_set_binary_op(ClassSetBinaryOp { kind, lhs, rhs, .. }: &ClassSetBinaryOp) -> Vec<String> {
  use ClassSetBinaryOpKind::*;

  let lhs = convert_class_set(lhs);
  let rhs = convert_class_set(rhs);
  match kind {
    Intersection => rhs.into_iter().filter(|x| lhs.contains(x)).collect(),
    Difference => lhs.into_iter().filter(|x| !rhs.contains(x)).collect(),
    SymmetricDifference => Iterator::chain(
      lhs.iter().filter(|x| !rhs.contains(x)),
      rhs.iter().filter(|x| !lhs.contains(x)),
    )
    .cloned()
    .collect(),
  }
}

fn convert_repetition(Repetition { op, ast, .. }: &Repetition) -> Vec<String> {
  use RepetitionKind::*;
  use RepetitionRange::*;

  match op.kind {
    ZeroOrOne => convert_repetition_range(ast, 0, 1),
    ZeroOrMore => convert(ast).into_iter().map(|x| format!("{}*", x)).collect(),
    OneOrMore => convert(ast).into_iter().map(|x| format!("{}+", x)).collect(),
    Range(Exactly(m)) => convert_repetition_range(ast, m, m),
    Range(AtLeast(m)) => convert(ast).into_iter().map(|x| format!("{}{{{},}}", x, m)).collect(),
    Range(Bounded(m, n)) => convert_repetition_range(ast, m, n),
  }
}

fn convert_repetition_range(ast: &Ast, m: u32, n: u32) -> Vec<String> {
  let v = convert(ast);
  (m..=n)
    .flat_map(|i| match i {
      0 => vec!["".to_string()],
      1 => v.clone(),
      _ => vec![v.clone(); i as _]
        .into_iter()
        .multi_cartesian_product()
        .map(|x| x.join(""))
        .collect(),
    })
    .collect()
}

fn convert_group(Group { ast, .. }: &Group) -> Vec<String> {
  convert(ast)
}

fn convert_alternation(Alternation { asts, .. }: &Alternation) -> Vec<String> {
  asts.iter().flat_map(convert).collect()
}

fn convert_concat(Concat { asts, .. }: &Concat) -> Vec<String> {
  asts
    .iter()
    .map(convert)
    .multi_cartesian_product()
    .map(|x| x.join(""))
    .collect()
}
