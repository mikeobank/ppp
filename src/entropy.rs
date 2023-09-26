
use regex::Regex;
use num_bigint::BigUint;
use num_traits::{Zero, One};
use num::pow::pow;

use crate::text_file;

pub fn calculate(strings: &Vec<String>) -> (BigUint, Vec<BigUint>) {

  let mut entropies: Vec<BigUint> = Vec::new();
  let mut total: BigUint = One::one();
  for string in strings.iter() {
    let entropy = calculate_for_string(string);
    entropies.push(entropy.clone());
    total = total * entropy;
  }
  return (total, entropies);
}

fn calculate_for_string(string: &str) -> BigUint {

  let num_chars = string.chars().count();

  if num_chars == 0 {
    return One::one();
  }

  if in_dictionary(string) {
    return One::one();
  }

  let base: u32;

  let digits_regex = Regex::new(r"^\d+$").unwrap();
  let hex_regex = Regex::new(r"^[a-fA-F0-9]+$").unwrap();
  let alphanumeric_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
  let ascii_regex = Regex::new(r"^[ -~]+$").unwrap();

  if digits_regex.is_match(string) {
    base = 10;
  } else if hex_regex.is_match(string) {
    base = 16;
  } else if alphanumeric_regex.is_match(string) {
    base = 42;
  } else if ascii_regex.is_match(string) {
    base = 126 - 32;
  } else {
    // total number of Unicode chars (https://unicode.org/faq/basic_q.html)
    base = 149186;
  }

  return pow(BigUint::from(base), num_chars);
}

pub fn decimal_to_bits(n: &BigUint) -> u32 {
  let mut b: u32 = 0;
  let mut nn: BigUint = n.clone();
  let zero = Zero::zero();
  while nn > zero {
    b = b + 1;
    nn = nn >> 1;
  }
  return b;
}

fn in_dictionary(string: &str) -> bool {
  return text_file::find_in(String::from("./dict/1000000-password-seclists.txt"), string)
}