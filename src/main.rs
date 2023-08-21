mod questions;
mod solutions;
mod utils;

use std::{
    error::Error,
    io::{self},
    time::SystemTime,
};

use crate::utils::files::{file_to_string_vec, get_input_file_path};
use crate::{
    questions::{day1, day2},
    solutions::Solution,
};

// use solutions::Solution;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welcome to AOC 2021");
    let solutions: Vec<Box<dyn Solution>> =
        vec![Box::new(day1::Question), Box::new(day2::Question)];
    let mut day = String::new();
    let mut part = String::new();

    println!("Enter the day you want to solve for. Eg for day, enter 1 - 25 :");
    io::stdin().read_line(&mut day)?;

    println!("Enter either 1 or 2, to solve the first or second problem :");
    io::stdin().read_line(&mut part)?;

    let start_time = SystemTime::now();

    let day = day.trim();
    let part = part.trim();

    println!("AOC:2021 Solving Day {}, part {} ....", day, part);

    let path_to_file = get_input_file_path(day, part);
    let input = file_to_string_vec(&path_to_file)?;

    let solution = &solutions[day.parse::<usize>()? - 1];

    if part == "1" {
        println!("Solution: {}", solution.solve_input_1(&input)?);
    } else {
        println!("Solution: {}", solution.solve_input_2(&input)?);
    }

    println!("Elapsed_ms: {}", start_time.elapsed()?.as_millis());
    Ok(())
}
