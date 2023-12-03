use std::collections::HashMap;
use crate::solutions::{Solution, SolutionResult};

#[derive(Default)]
pub struct Question;

impl Question {
    fn has_valid_cube_count(&self, color_parts: &str) -> bool {
        // 3 blue, 4 red, 1 red, 2 green, 6 blue, 2 green
        let color_map: HashMap<&str, i32> = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]);
        for color_part in color_parts.split(",") {
            let mut color_part = color_part.trim().split(" ");
            let color_count = color_part.next().unwrap().trim().parse::<i32>().unwrap();
            let color_name = color_part.next().unwrap().trim();
            if &color_count > color_map.get(color_name).unwrap() {
                return false;
            }
        }
        true
    }

    fn solve_1(&self, input: &Vec<String>) -> SolutionResult {
        let game_sum = input.iter().fold(0, |acc, line| {
            let mut parts = line.split(":");
            let game_part = parts.next().unwrap();
            let color_parts = parts.next().unwrap();

            let mut game_part = game_part.split("Game ");
            // skip the first part ""
            game_part.next();
            let game_number = game_part.next().unwrap().parse::<i32>().unwrap();

            let color_parts = color_parts.replace(";", ",");
            if self.has_valid_cube_count(&color_parts) {
                println!("Game {} is valid", game_number);
                acc + game_number
            } else {
                acc
            }
        });
        Ok(game_sum.to_string())
    }
}

impl Solution for Question {
    fn solve_input_1(&self, input: &Vec<String>) -> SolutionResult {
        self.solve_1(input)
    }

    fn solve_input_2(&self, input: &Vec<String>) -> SolutionResult {
        self.solve_1(input)
    }
}
