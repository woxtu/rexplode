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
