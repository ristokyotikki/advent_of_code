pub fn run(files: Vec<String>) {
  println!("\nExcuting Day 1 Dive! - excercise 1");
  excercise_one(&files);
}

fn excercise_one(files: &Vec<String>) {
  let mut depth: i32 = 0;
  let mut position: i32 = 0;

  for file in files {
    let instruction: Vec<&str> = file.split_whitespace().collect();

    match instruction[0] {
      "up" => depth -= instruction[1].parse::<i32>().unwrap(),
      "down" => depth += instruction[1].parse::<i32>().unwrap(),
      "forward" => position += instruction[1].parse::<i32>().unwrap(),
      _ => panic!("unexpected instruction! {}", instruction[0])
    }
  }

  println!("depth is: {}", depth);
  println!("position is: {}", position);
  println!("total is: {}", depth * position);
}