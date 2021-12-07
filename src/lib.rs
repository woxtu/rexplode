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
    Literal(literal) => vec![format_literal(literal)],
    Class(class) => convert_class(class),
    Repetition(repetition) => convert_repetition(repetition),
    Group(group) => convert_group(group),
    Alternation(alternation) => convert_alternation(alternation),
    _ => unimplemented!(),
  }
}

fn convert_class(class: &Class) -> Vec<String> {
  use Class::*;

  match class {
    Unicode(unicode) => vec![format_class_unicode(unicode)],
    Perl(perl) => vec![format_class_perl(perl)],
    Bracketed(bracketed) => convert_class_bracketed(bracketed),
  }
}

fn convert_class_bracketed(bracketed @ ClassBracketed { negated, kind, .. }: &ClassBracketed) -> Vec<String> {
  if *negated {
    vec![format_class_bracketed(bracketed)]
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
    Empty(_) | Literal(_) | Ascii(_) | Unicode(_) | Perl(_) => vec![format_class_set_item(item)],
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

fn format_literal(Literal { c, .. }: &Literal) -> String {
  c.to_string()
}

fn format_class_ascii(ClassAscii { kind, negated, .. }: &ClassAscii) -> String {
  use ClassAsciiKind::*;

  format!(
    "[:{}{}:]",
    if *negated { "^" } else { "" },
    match kind {
      Alnum => "alnum",
      Alpha => "alpha",
      Ascii => "ascii",
      Blank => "blank",
      Cntrl => "cntrl",
      Digit => "digit",
      Graph => "graph",
      Lower => "lower",
      Print => "print",
      Punct => "punct",
      Space => "space",
      Upper => "upper",
      Word => "word",
      Xdigit => "xdigit",
    }
  )
}

fn format_class_unicode(ClassUnicode { negated, kind, .. }: &ClassUnicode) -> String {
  use ClassUnicodeKind::*;
  use ClassUnicodeOpKind::*;

  format!(
    r"\{}{}",
    if *negated { 'P' } else { 'p' },
    match kind {
      OneLetter(letter) => letter.to_string(),
      Named(name) => format!("{{{}}}", name),
      NamedValue { op: Equal, name, value } => format!("{{{}={}}}", name, value),
      NamedValue { op: Colon, name, value } => format!("{{{}:{}}}", name, value),
      NamedValue {
        op: NotEqual,
        name,
        value,
      } => format!("{{{}!={}}}", name, value),
    }
  )
}

fn format_class_perl(ClassPerl { kind, negated, .. }: &ClassPerl) -> String {
  use ClassPerlKind::*;

  format!(
    r"\{}",
    match kind {
      Digit if *negated => 'D',
      Digit => 'd',
      Space if *negated => 'S',
      Space => 's',
      Word if *negated => 'W',
      Word => 'w',
    }
  )
}

fn format_class_bracketed(ClassBracketed { negated, kind, .. }: &ClassBracketed) -> String {
  format!("[{}{}]", if *negated { "^" } else { "" }, format_class_set(kind),)
}

fn format_class_set(set: &ClassSet) -> String {
  use ClassSet::*;

  match set {
    Item(item) => format_class_set_item(item),
    BinaryOp(op) => format_class_set_binary_op(op),
  }
}

fn format_class_set_item(item: &ClassSetItem) -> String {
  use ClassSetItem::*;

  match item {
    Empty(_) => "".to_string(),
    Literal(literal) => format_literal(literal),
    Range(range) => format_class_set_range(range),
    Ascii(ascii) => format_class_ascii(ascii),
    Unicode(unicode) => format_class_unicode(unicode),
    Perl(perl) => format_class_perl(perl),
    Bracketed(bracketed) => format_class_bracketed(bracketed),
    Union(union) => format_class_set_union(union),
  }
}

fn format_class_set_range(ClassSetRange { start, end, .. }: &ClassSetRange) -> String {
  format!("{}-{}", format_literal(start), format_literal(end))
}

fn format_class_set_union(ClassSetUnion { items, .. }: &ClassSetUnion) -> String {
  items.iter().map(format_class_set_item).collect()
}

fn format_class_set_binary_op(ClassSetBinaryOp { kind, lhs, rhs, .. }: &ClassSetBinaryOp) -> String {
  use ClassSetBinaryOpKind::*;

  match kind {
    Intersection => format!("{}&&{}", format_class_set(lhs), format_class_set(rhs)),
    Difference => format!("{}--{}", format_class_set(lhs), format_class_set(rhs)),
    SymmetricDifference => format!("{}~~{}", format_class_set(lhs), format_class_set(rhs)),
  }
}
