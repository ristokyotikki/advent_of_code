use std::fs::{read_to_string, File};
use std::io::{prelude::*, BufReader};

pub fn file_as_string(path_to_file: &str) -> String {
  return read_to_string(path_to_file).expect("unable to read file");
}

fn read_file(path_to_file: &str) -> BufReader<File> {
  let file = File::open(path_to_file).expect("no such file");
  return BufReader::new(file);
}

pub fn read_lines_from_file(path_to_file: &str) -> Vec<String> {
  let reader = read_file(path_to_file);
  let lines: Vec<String> = reader
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();
  return lines;
}
