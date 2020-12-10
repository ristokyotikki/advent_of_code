use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
  let file = File::open("inputs").expect("no such file");
  let buf = BufReader::new(file);
  let lines: Vec<String> = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();

  let mut num_yes = 0;
  let mut num_group_yes = 0;
  let mut group = HashSet::<char>::new();
  let mut group_2 = HashSet::<char>::new();
  let mut new_group = true;
  for (i, line) in lines.iter().enumerate() {
    let mut passenger = HashSet::<char>::new();
    for answer in line.chars() {
      passenger.insert(answer);
      if new_group {
        group_2.insert(answer);
      }
    }
    new_group = false;
    group.extend(passenger.iter());
    if line.len() == 0 || i == lines.len() - 1 {
      num_yes += group.len();
      num_group_yes += group_2.len();
      group = HashSet::<char>::new();
      group_2 = HashSet::<char>::new();
      new_group = true;
    } else {
      group_2.retain(|x| passenger.contains(x));
    }
  }

  println!("Total amount of yes answers in all groups is: {}", num_yes);
  println!("Total amount of a groups yes answers is: {}", num_group_yes);
}
