use crate::solutions::utils::*;

#[derive(Clone)]
struct Column {
  value: String,
  most_common_char: char,
}

pub fn run() {
  println!("\nExcuting Day 3 Binary Diagnostics - excercise 1");
  excercise_one();

  println!("\nExcuting Day 1 Sonar Sweep - excercise 2");
  excercise_two();
}

fn create_column(file: String, n: usize) -> Column {
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

  let mut most_common_char = '0';
  if ones >= bits.len() / 2 {
    most_common_char = '1';
  }

  return Column {
    value: bits,
    most_common_char: most_common_char,
  };
}

fn excercise_one() {
  let file = file_as_string("src/inputs/binary_diagnostics");
  let line_len = file[..file.find("\n").unwrap()].len();
  let mut gamma = String::new();
  let mut epsilon = String::new();

  for n in 0..line_len {
    let column: Column = create_column(file.clone(), n);

    match column.most_common_char == '1' {
      true => {
        gamma += "1";
        epsilon += "0"
      }
      false => {
        gamma += "0";
        epsilon += "1"
      }
    }
  }

  let gamma_int = isize::from_str_radix(&gamma, 2).unwrap();
  let epsilon_int = isize::from_str_radix(&epsilon, 2).unwrap();
  println!("Power consumption is: {}", gamma_int * epsilon_int);
}

fn get_rating(rating_type: &str) -> String {
  let report = read_lines_from_file("src/inputs/binary_diagnostics");
  let mut lines = report.clone();
  let mut bit_pos = 0;
  while lines.len() != 1 {
    let mut ones = 0;
    let mut zeroes = 0;
    lines.iter().for_each(|c| {
      let char = c.chars().nth(bit_pos).unwrap();
      if char == '1' {
        ones += 1;
      } else {
        zeroes += 1;
      }
    });

    
    let choose_one = ones >= zeroes;
    if rating_type == "oxygen" {
      lines.retain(|c| (c.chars().nth(bit_pos).unwrap() == '1') == choose_one);
    }
    if rating_type == "co2" {
      lines.retain(|c| (c.chars().nth(bit_pos).unwrap() == '1') == !choose_one);
    }

    bit_pos += 1;
  }

  return lines[0].to_string();
}

fn excercise_two() {
  let oxygen_generator_rating = get_rating("oxygen");
  let co2_scrbber_rating = get_rating("co2");

  let oxygen_generator_rating_int = isize::from_str_radix(&oxygen_generator_rating, 2).unwrap();
  let co2_scrbber_rating_int = isize::from_str_radix(&co2_scrbber_rating, 2).unwrap();

  let life_support_rating = oxygen_generator_rating_int * co2_scrbber_rating_int;
  println!("Life support rating is {}", life_support_rating);
}
