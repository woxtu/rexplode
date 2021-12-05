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
  use ClassSet::*;

  match kind {
    Item(_) if *negated => vec![format_class_bracketed(bracketed)],
    Item(item) => convert_class_set_item(item),
    _ => unimplemented!(),
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
  use ClassSet::*;

  format!(
    "[{}{}]",
    if *negated { "^" } else { "" },
    match kind {
      Item(item) => format_class_set_item(item),
      _ => unimplemented!(),
    }
  )
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
