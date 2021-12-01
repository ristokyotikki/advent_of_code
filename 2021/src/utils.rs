use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn read_lines_from_file(path_to_file: &str) -> Vec<String> {
  let file = File::open(path_to_file).expect("no such file");
  let buf = BufReader::new(file);
  let lines: Vec<String> = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();
  return lines;
}
