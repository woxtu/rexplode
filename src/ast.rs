use regex_syntax::ast::*;

pub trait ToString {
  fn to_string(&self) -> String;
}

impl ToString for Literal {
  fn to_string(&self) -> String {
    self.c.to_string()
  }
}

impl ToString for ClassAscii {
  fn to_string(&self) -> String {
    use ClassAsciiKind::*;

    format!(
      "[:{}{}:]",
      if self.negated { "^" } else { "" },
      match &self.kind {
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
}

impl ToString for ClassUnicode {
  fn to_string(&self) -> String {
    use ClassUnicodeKind::*;
    use ClassUnicodeOpKind::*;

    format!(
      r"\{}{}",
      if self.negated { 'P' } else { 'p' },
      match &self.kind {
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
}

impl ToString for ClassPerl {
  fn to_string(&self) -> String {
    use ClassPerlKind::*;

    format!(
      r"\{}",
      match &self.kind {
        Digit if self.negated => 'D',
        Digit => 'd',
        Space if self.negated => 'S',
        Space => 's',
        Word if self.negated => 'W',
        Word => 'w',
      }
    )
  }
}

impl ToString for ClassBracketed {
  fn to_string(&self) -> String {
    format!("[{}{}]", if self.negated { "^" } else { "" }, self.kind.to_string(),)
  }
}

impl ToString for ClassSet {
  fn to_string(&self) -> String {
    use ClassSet::*;

    match &self {
      Item(item) => item.to_string(),
      BinaryOp(op) => op.to_string(),
    }
  }
}

impl ToString for ClassSetItem {
  fn to_string(&self) -> String {
    use ClassSetItem::*;

    match self {
      Empty(_) => "".to_string(),
      Literal(literal) => literal.to_string(),
      Range(range) => range.to_string(),
      Ascii(ascii) => ascii.to_string(),
      Unicode(unicode) => unicode.to_string(),
      Perl(perl) => perl.to_string(),
      Bracketed(bracketed) => bracketed.to_string(),
      Union(union) => union.to_string(),
    }
  }
}

impl ToString for ClassSetRange {
  fn to_string(&self) -> String {
    format!("{}-{}", self.start.to_string(), self.end.to_string())
  }
}

impl ToString for ClassSetUnion {
  fn to_string(&self) -> String {
    self.items.iter().map(ToString::to_string).collect()
  }
}

impl ToString for ClassSetBinaryOp {
  fn to_string(&self) -> String {
    use ClassSetBinaryOpKind::*;

    match &self.kind {
      Intersection => format!("{}&&{}", self.lhs.to_string(), self.rhs.to_string()),
      Difference => format!("{}--{}", self.lhs.to_string(), self.rhs.to_string()),
      SymmetricDifference => format!("{}~~{}", self.lhs.to_string(), self.rhs.to_string()),
    }
  }
}
