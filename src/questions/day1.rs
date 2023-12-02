use crate::solutions::{Solution, SolutionResult};

#[derive(Default)]
pub struct Question;

struct Count {
    last_value: i32,
    count: i32,
    first_run: bool,
}
impl Question {
    fn solve(&self, input: &Vec<String>) -> SolutionResult {
        let result: Count = input.iter().fold(
            Count {
                last_value: 0,
                count: 0,
                first_run: true,
            },
            |acc: Count, f: &String| {
                let num: i32 = f.parse::<i32>().unwrap();
                if acc.first_run {
                    Count {
                        last_value: num,
                        count: acc.count,
                        first_run: false,
                    }
                } else if num > acc.last_value {
                    Count {
                        last_value: num,
                        count: acc.count + 1,
                        first_run: false,
                    }
                } else {
                    Count {
                        last_value: num,
                        count: acc.count,
                        first_run: false,
                    }
                }
            },
        );
        Ok(result.count.to_string())
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
