use crate::solutions::{Solution, SolutionResult};

#[derive(Default)]
pub struct Question;

impl Question {
    fn solve(&self, input: &Vec<String>) -> SolutionResult {
        Ok("2".to_string())
    }
}

impl Solution for Question {
    fn solve_input_1(&self, input: &Vec<String>) -> SolutionResult {
        self.solve(input)
    }

    fn solve_input_2(&self, input: &Vec<String>) -> SolutionResult {
        self.solve(input)
    }
}
