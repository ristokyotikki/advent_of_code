use crate::solutions::utils::*;

pub fn run() {
  let lines = read_lines_from_file("src/inputs/sonar_sweep");
  println!("\nExcuting Day 1 Sonar Sweep - excercise 1");
  excercise_one(&lines);

  println!("\nExcuting Day 1 Sonar Sweep - excercise 2");
  excercise_two(&lines);
}

fn excercise_one(lines: &Vec<String>) {
  let mut prev = String::new();
  let mut count = 0;

  for curr in lines {
    let curr_usize = curr.parse::<usize>().unwrap();
    match curr_usize {
      _ if prev.is_empty() => (),
      c if c > prev.parse::<usize>().unwrap() => count += 1,
      _ => (),
    }

    prev = curr.to_string();
  }

  println!("Found {} times depth measurement increased", count);
}

fn excercise_two(files: &Vec<String>) {
  let mut count: usize = 0;
  let mut a: usize = 0;
  let mut b: usize = 0;
  let mut prev: usize = 0;

  for (index, curr) in files.iter().enumerate() {
    let curr_usize = curr.parse::<usize>().unwrap();

    if (index + 1) % 2 == 0 && index != 2 {
      if index > 0 {
        b += curr_usize;
      }

      if index > 2 && b > a {
        count += 1
      }

      a = prev + curr_usize;
    } else {
      a += curr_usize;

      if index > 3 && a > b {
        count += 1;
      }

      b = prev + curr_usize;
    }
    prev = curr_usize;
  }
  println!("Found {} times depth measurement increased", count);
}
