mod utils;
mod solutions;

use solutions::*;
use utils::read_lines_from_file;
use::std::fs;
use std::io;

static SOLUTIONS_PATH: &str ="./src/solutions";

fn main() {
    let paths_count = fs::read_dir(SOLUTIONS_PATH).unwrap().count() - 1;
    let paths = fs::read_dir(SOLUTIONS_PATH).unwrap();
    
    println!("\n\n");
    println!("Which solution would you like to run?");
    for (i, path) in paths.enumerate() {
        let file_name = path.unwrap().file_name();
        if file_name != "mod.rs" {
            println!("    {}: {:?}", i + 1, file_name)
        }
    }

    let mut input = 999;
    while input > paths_count {
        let prompt = format!("Run solution: (Select by inputting number between: 1 and {})", paths_count);
        input = prompt_int_input(prompt);
    }

    match input {
        1 => sonar_sweep::run(read_lines_from_file("src/inputs/sonar_sweep")),
        2 => dive::run(read_lines_from_file("src/inputs/dive")),
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