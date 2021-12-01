pub fn run(files: Vec<String>) {
  println!("\nExcuting Day 1 excercise Sonar Sweep");

  let mut prev = String::new();
  let mut count = 0;
  for curr in files {
    match &curr {
      _ if prev.is_empty() => (),
      c if c.parse::<usize>().unwrap() > prev.parse::<usize>().unwrap() => count += 1,
      _ => ()
    }

    prev = curr;
  }

  println!("Found {} times depth measurement increased", count);
}