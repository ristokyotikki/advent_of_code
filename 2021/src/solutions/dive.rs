pub fn run(files: Vec<String>) {
  println!("\nExcuting Day 2 Dive! - excercise 1");
  excercise_one(&files);

  println!("\nExcuting Day 2 Dive! - excercise 2");
  excercise_two(&files);
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

fn excercise_two(files: &Vec<String>) {
  let mut depth: i32 = 0;
  let mut position: i32 = 0;
  let mut aim: i32 = 0;

  for file in files {
    let instruction: Vec<&str> = file.split_whitespace().collect();

    match instruction[0] {
      "up" => aim -= instruction[1].parse::<i32>().unwrap(),
      "down" => aim += instruction[1].parse::<i32>().unwrap(),
      "forward" => {
        position += instruction[1].parse::<i32>().unwrap();
        depth += instruction[1].parse::<i32>().unwrap() * aim;
      },
      _ => panic!("unexpected instruction! {}", instruction[0])
    }
  }

  println!("depth is: {}", depth);
  println!("position is: {}", position);
  println!("total is: {}", depth * position);
}