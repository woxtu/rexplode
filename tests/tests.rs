use rexplode::explode;

#[test]
fn test_empty() {
  assert_eq!(explode(""), Ok(vec![]));
}

#[test]
fn test_flags() {
  assert_eq!(explode("(?is)"), Ok(vec!["(?is)".to_string()]));
}

#[test]
fn test_literals() {
  assert_eq!(explode("a"), Ok(vec!["a".to_string()]));
  assert_eq!(explode(r"\*"), Ok(vec!["*".to_string()]));
  assert_eq!(explode(r"\x61"), Ok(vec!["a".to_string()]));
  assert_eq!(explode(r"\u0061"), Ok(vec!["a".to_string()]));
  assert_eq!(explode(r"\u{0061}"), Ok(vec!["a".to_string()]));
}

#[test]
fn test_dot() {
  assert_eq!(explode("."), Ok(vec![".".to_string()]));
}

#[test]
fn test_assertions() {
  assert_eq!(explode("^"), Ok(vec!["^".to_string()]));
  assert_eq!(explode(r"\A"), Ok(vec![r"\A".to_string()]));
}

#[test]
fn test_classes() {
  assert_eq!(explode(r"\pL"), Ok(vec![r"\pL".to_string()]));
  assert_eq!(explode(r"\PL"), Ok(vec![r"\PL".to_string()]));
  assert_eq!(explode(r"\p{Greek}"), Ok(vec![r"\p{Greek}".to_string()]));
  assert_eq!(explode(r"\P{Greek}"), Ok(vec![r"\P{Greek}".to_string()]));
  assert_eq!(explode(r"\p{foo=bar}"), Ok(vec![r"\p{foo=bar}".to_string()]));
  assert_eq!(explode(r"\P{foo=bar}"), Ok(vec![r"\P{foo=bar}".to_string()]));
  assert_eq!(explode(r"\d"), Ok(vec![r"\d".to_string()]));
  assert_eq!(explode(r"\D"), Ok(vec![r"\D".to_string()]));
  assert_eq!(explode("[a]"), Ok(vec!["a".to_string()]));
  assert_eq!(
    explode("[a-c]"),
    Ok(vec!["a".to_string(), "b".to_string(), "c".to_string()])
  );
  assert_eq!(explode("[[:alnum:]]"), Ok(vec!["[:alnum:]".to_string()]));
  assert_eq!(explode("[[:^alnum:]]"), Ok(vec!["[:^alnum:]".to_string()]));
  assert_eq!(explode(r"[\pL]"), Ok(vec![r"\pL".to_string()]));
  assert_eq!(explode(r"[\p{Greek}]"), Ok(vec![r"\p{Greek}".to_string()]));
  assert_eq!(explode(r"[\p{foo=bar}]"), Ok(vec![r"\p{foo=bar}".to_string()]));
  assert_eq!(explode(r"[\d]"), Ok(vec![r"\d".to_string()]));
  assert_eq!(explode("[[a]]"), Ok(vec!["a".to_string()]));
  assert_eq!(
    explode("[a-cd]"),
    Ok(vec!["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()])
  );
  assert_eq!(explode("[^a]"), Ok(vec!["[^a]".to_string()]));
  assert_eq!(explode("[^a-c]"), Ok(vec!["[^a-c]".to_string()]));
  assert_eq!(explode("[^[:alnum:]]"), Ok(vec!["[^[:alnum:]]".to_string()]));
  assert_eq!(explode(r"[^\pL]"), Ok(vec![r"[^\pL]".to_string()]));
  assert_eq!(explode(r"[^\p{Greek}]"), Ok(vec![r"[^\p{Greek}]".to_string()]));
  assert_eq!(explode(r"[^\d]"), Ok(vec![r"[^\d]".to_string()]));
  assert_eq!(explode("[^[a]]"), Ok(vec!["[^[a]]".to_string()]));
  assert_eq!(explode("[^a-cd]"), Ok(vec!["[^a-cd]".to_string()]));
}

#[test]
fn test_class_operators() {
  assert_eq!(explode("[a-c&&b-d]"), Ok(vec!["b".to_string(), "c".to_string()]));
  assert_eq!(explode("[a-c--b-d]"), Ok(vec!["a".to_string()]));
  assert_eq!(explode("[a-c~~b-d]"), Ok(vec!["a".to_string(), "d".to_string()]));
  assert_eq!(explode("[^a-c&&b-d]"), Ok(vec!["[^a-c&&b-d]".to_string()]));
  assert_eq!(explode("[^a-c--b-d]"), Ok(vec!["[^a-c--b-d]".to_string()]));
  assert_eq!(explode("[^a-c~~b-d]"), Ok(vec!["[^a-c~~b-d]".to_string()]));
}

#[test]
fn test_repetitions() {
  assert_eq!(explode("a?"), Ok(vec!["".to_string(), "a".to_string()]));
  assert_eq!(explode("a*"), Ok(vec!["a*".to_string()]));
  assert_eq!(explode("a+"), Ok(vec!["a+".to_string()]));
  assert_eq!(explode("a{2}"), Ok(vec!["aa".to_string()]));
  assert_eq!(explode("a{2,}"), Ok(vec!["a{2,}".to_string()]));
  assert_eq!(
    explode("a{0,2}"),
    Ok(vec!["".to_string(), "a".to_string(), "aa".to_string()])
  );
}

#[test]
fn test_groups() {
  assert_eq!(explode("(a)"), Ok(vec!["a".to_string()]));
  assert_eq!(explode("(?P<name>a)"), Ok(vec!["a".to_string()]));
  assert_eq!(explode("(?:a)"), Ok(vec!["a".to_string()]));
  assert_eq!(explode("(?i:a)"), Ok(vec!["a".to_string()]));
}

#[test]
fn test_alternations() {
  assert_eq!(explode("a|b"), Ok(vec!["a".to_string(), "b".to_string()]));
  assert_eq!(
    explode("a|b|c"),
    Ok(vec!["a".to_string(), "b".to_string(), "c".to_string()])
  );
}
