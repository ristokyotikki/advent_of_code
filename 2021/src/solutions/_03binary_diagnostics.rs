use crate::solutions::utils::*;

pub fn run() {
  println!("\nExcuting Day 3 Binary Diagnostics - excercise 1");
  excercise_one();

  // println!("\nExcuting Day 1 Sonar Sweep - excercise 2");
  // excercise_two(&files);
}

fn excercise_one() {
  let file = file_as_string("src/inputs/binary_diagnostics");
  let line_len = file[..file.find("\n").unwrap()].len();
  let mut gamma = String::new();
  let mut epsilon = String::new();

  for n in 0..line_len {
    let mut ones = 0;
    let bits: String = file
      .trim()
      .split("\n")
      .map(|s| {
        let char = s.chars().nth(n).unwrap();
        if char == '1' {
          ones += 1
        }
        return char;
      })
      .into_iter()
      .collect();

    let more_ones = ones > bits.len() / 2;
    if more_ones {
      gamma += "1";
      epsilon += "0"
    } else {
      gamma += "0";
      epsilon += "1"
    }
  }

  let result: isize = isize::from_str_radix(&gamma, 2).unwrap() * isize::from_str_radix(&epsilon, 2).unwrap();
  println!("Power consumption is: {}", result);
}
