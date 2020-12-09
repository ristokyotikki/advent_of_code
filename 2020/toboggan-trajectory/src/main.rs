use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
  println!("\n------------------------------------");
  println!("Task 1: Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?\n");
  task_one(1, 3);
  println!("\n------------------------------------");
  println!("Task 2: What do you get if you multiply together the number of trees encountered on each of the listed slopes?\n");
  task_two();
  println!("");
}

fn task_one(down: usize, right: usize) -> usize {
  let file = File::open("inputs").expect("no such file");
  let buf = BufReader::new(file);
  let lines: Vec<String> = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();


  let mut trees = 0;
  let mut step = right;
  let mut row = down;

  for (index, line) in lines.iter().enumerate() {
    if index < row {
      continue;
    }

    let path: Vec<char> = line.chars().collect();
    let mut x = step;
    while x > line.len() - 1 {
      x = x - line.len();
    };

    match path[x] {
      '#' => trees += 1,
      '.' => {},
      _ => println!("Unexpected encounter! {}", path[step]),
    }

    step += right;
    row += down;
  }

  println!("Encountered {} trees.", trees);
  trees
}

fn task_two() {
  println!("Answer is: {:?}", task_one(1, 1) * task_one(1, 3) * task_one(1, 5) * task_one(1, 7) * task_one(2, 1));
}