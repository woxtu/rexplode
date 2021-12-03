use rexplode::explode;

#[test]
fn test_empty() {
  assert_eq!(explode(""), Ok(vec![]));
}

#[test]
fn test_flags() {
  assert_eq!(explode("(?is)"), Ok(vec!["(?is)".to_string()]));
}
