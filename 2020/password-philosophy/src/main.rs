use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
  println!("\n------------------------------------");
  println!("Task 1: How many passwords are valid?\n");
  task_one();
  println!("\n------------------------------------");
  println!("Task 2: How many passwords are valid?\n");
  task_two();
  println!("");
}

fn task_one() {
  let file = File::open("inputs").expect("no such file");
  let buf = BufReader::new(file);
  let mut lines: Vec<String> = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();
  lines.retain(|line| {
    let mut retain = true;
    let min_max_seperator = line.find("-").unwrap();
    let letter_separator = line.find(" ").unwrap();
    let password_separator = line.find(":").unwrap();
    let min = &line[..min_max_seperator].parse::<usize>().unwrap();
    let max = &line[min_max_seperator + 1..letter_separator]
      .parse::<usize>()
      .unwrap();
    let letter = &line[letter_separator + 1..password_separator];
    let count = &line[password_separator..].matches(letter).count();
    if count > max || count < min {
      retain = false;
    }
    retain
  });
  println!("Number of correct passwords is: {}", lines.len());
}

fn task_two() {
  let file = File::open("inputs").expect("no such file");
  let buf = BufReader::new(file);
  let mut lines: Vec<String> = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();
  lines.retain(|line| {
    let mut retain = false;
    let min_max_seperator = line.find("-").unwrap();
    let letter_separator = line.find(" ").unwrap();
    let password_separator = line.find(":").unwrap();
    let pos_1 = line[..min_max_seperator].parse::<usize>().unwrap() + 1;
    let pos_2 = line[min_max_seperator + 1..letter_separator].parse::<usize>().unwrap() + 1;
    let letter = &line[letter_separator + 1..password_separator];
    let password = &line[password_separator..];
    let a = password.chars().nth(pos_1);
    let b = password.chars().nth(pos_2);
    let c = letter.chars().nth(0);
    if (a == c || b == c) && a != b {
      retain = true;
    }
    retain
  });
  println!("Number of correct passwords is: {}", lines.len());
}
