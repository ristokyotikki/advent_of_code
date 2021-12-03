mod solutions;

use solutions::*;
use::std::fs;
use std::io;

static SOLUTIONS_PATH: &str ="./src/solutions";

fn main() {
    let paths_count = fs::read_dir(SOLUTIONS_PATH).unwrap().count() - 2;
    let paths = fs::read_dir(SOLUTIONS_PATH).unwrap();
    let mut choice_adjustment: isize = 1;

    println!("\n\n");
    println!("Which solution would you like to run?");
    for (i, path) in paths.enumerate() {
        let file_name = path.unwrap().file_name();
        if file_name == "mod.rs" || file_name == "utils.rs" {
            choice_adjustment -= 1;
            continue;
        }

        let choice = i as isize + choice_adjustment;
        println!("    {}: {:?}", choice, file_name)
    }

    let mut input = 999;
    while input > paths_count {
        let prompt = format!("Run solution: (Select by inputting number between: 1 and {})", paths_count);
        input = prompt_int_input(prompt);
    }

    match input {
        1 => _01sonar_sweep::run(),
        2 => _02dive::run(),
        3 => _03binary_diagnostics::run(),
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