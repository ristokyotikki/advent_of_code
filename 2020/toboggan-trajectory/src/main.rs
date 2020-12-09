use linecount::count_lines;
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
  let mut grid = Vec::new();
  let file = File::open("inputs").expect("no such file");
  let grid_heigth = count_lines(&file).unwrap();
  let grid_width = right * grid_heigth;
  let file = File::open("inputs").expect("no such file");
  let buf = BufReader::new(file);
  let lines: Vec<String> = buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect();

  for line in lines.iter() {
    let mut row = Vec::new();
    let grid_repeat = (grid_width as f64 / line.len() as f64).ceil() as usize;
    
    for _n in 0..grid_repeat {
      let mut input_row: Vec<char> = line.chars().collect();
      row.append(&mut input_row);
    }

    grid.push(row);
  }

  let mut trees = 0;
  let mut step = right;
  let mut row = down;

  while row < grid_heigth {
    if step < grid_width {
      match grid[row][step] {
        '#' => trees += 1,
        '.' => {},
        _ => println!("Unexpected encounter! {}", grid[row][step]),
      }
    } else {
      println!("step out of bounds: {}. Grid width is: {}", step, grid_width);
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