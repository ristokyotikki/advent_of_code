use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
  let file = File::open("inputs").expect("no such file");
  let buf = BufReader::new(file);
  let mut lines: Vec<String> = buf.lines().map(|l| l.expect("Could not parse line")).collect();
  lines.retain(|line| {
    let mut retain = true;
    let min_max_seperator = line.find("-").unwrap();
    let letter_seperator = line.find(" ").unwrap();
    let password_seperator = line.find(":").unwrap();
    let min = &line[..min_max_seperator].parse::<usize>().unwrap();
    let max = &line[min_max_seperator + 1..letter_seperator].parse::<usize>().unwrap();
    let letter = &line[letter_seperator + 1..password_seperator];
    let count = &line[password_seperator..].matches(letter).count();
    if count > max || count < min {
      retain = false;
    }
    retain
  });
  println!("Number of correct passwords is: {}", lines.len());
}