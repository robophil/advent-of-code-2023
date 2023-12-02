use crate::solutions::{Solution, SolutionResult};

#[derive(Default)]
pub struct Question;

impl Question {
    fn solve_1(&self, input: &Vec<String>) -> SolutionResult {
        let sum = input.iter().fold(0, |acc, line| {
            let mut first = 0;
            let mut second = 0;
            let mut first_number_found = false;

            for c in line.chars() {
                if c.is_numeric() {
                    if !first_number_found {
                        first_number_found = true;
                        first = c.to_digit(10).unwrap() * 10;
                        second = c.to_digit(10).unwrap();
                    } else {
                        second = c.to_digit(10).unwrap();
                    }
                }
            }
            acc + first + second
        });
        Ok(sum.to_string())
    }

    fn solve_2(&self, input: &Vec<String>) -> SolutionResult {
        Ok("".to_string())
    }
}

impl Solution for Question {
    fn solve_input_1(&self, input: &Vec<String>) -> SolutionResult {
        self.solve_1(input)
    }

    fn solve_input_2(&self, input: &Vec<String>) -> SolutionResult {
        self.solve_2(input)
    }
}
