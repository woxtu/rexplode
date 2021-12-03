use rexplode::explode;

#[test]
fn test_empty() {
  assert_eq!(explode(""), Ok(vec![]));
}
