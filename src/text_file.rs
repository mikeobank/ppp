
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn find_in(filename: String, pattern: &str) -> bool {

  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(s) = line {
        if s == pattern {
          return true
        }
      }
    }
  }

  return false
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

