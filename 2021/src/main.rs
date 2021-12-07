mod solutions;

use solutions::*;
use std::io;

fn main() {
    let prompt = format!(
        "Input number of the solution you would like to execute:
        1: Sonar Sweep
        2: Dive!
        3: Binary Diagnostics
        4: Giant Squid
        5: Hydrothermal Venture"
    );
    let input = prompt_int_input(prompt);

    match input {
        1 => _01sonar_sweep::run(),
        2 => _02dive::run(),
        3 => _03binary_diagnostics::run(),
        4 => _04giant_squid::run(),
        5 => _05hydrothermal_venture::run(),
        _ => panic!("Day {} not solved yet unfortunately", input),
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
        Err(error) => panic!("Failed to parse input to integer. Error: {}", error),
    }
}
