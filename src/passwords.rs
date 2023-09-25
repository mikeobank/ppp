pub fn prepare(strings: Vec<String>) -> Vec<String> {
  return strings.into_iter().map(|s| s.trim().into()).collect();
}