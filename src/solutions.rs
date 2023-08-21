use std::error::Error;

pub type SolutionResult = Result<String, Box<dyn Error>>;

pub trait Solution {
    fn solve_input_1(&self, input: &Vec<String>) -> SolutionResult;
    fn solve_input_2(&self, input: &Vec<String>) -> SolutionResult;
}
