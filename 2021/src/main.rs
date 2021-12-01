mod utils;

use utils::read_lines_from_file;
use::std::fs;
use std::io;

#[path = "solutions/01_sonar_sweep.rs"] mod sonar_sweep;

static SOLUTIONS_PATH: &str ="./src/solutions";

fn main() {
    let paths_count = fs::read_dir(SOLUTIONS_PATH).unwrap().count();
    let paths = fs::read_dir(SOLUTIONS_PATH).unwrap();
    
    println!("\n\n");
    println!("Which solution would you like to run?");
    for (i, path) in paths.enumerate() {
        println!("    {}: {:?}", i + 1, path.unwrap().file_name())
    }

    let mut input = 999;
    while input > paths_count {
        let prompt = format!("Run solution: (Select by inputting number between: 1 and {})", paths_count);
        input = prompt_int_input(prompt);
    }

    match input {
        1 => sonar_sweep::run(read_lines_from_file("src/inputs/sonar_sweep_1")),
        _ => panic!("input {}, not mapped to a solution", input)
    }
    
}

fn prompt_int_input(prompt: String) -> usize {
    let mut input = String::new();

    println!("\n{}", prompt);
    io::stdin()
    .read_line(&mut input)
    .expect("failed to read from stdin");
    
    let trimmed = input.trim();
    match trimmed.parse::<usize>() {
        Ok(integer) => return integer,
        Err(error) => panic!("Failed to parse input to integer. Error: {}", error)
    }
}