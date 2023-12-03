use std::collections::HashMap;

use crate::solutions::{Solution, SolutionResult};

#[derive(Default)]
pub struct Question;

impl Question {
    fn get_valid_number(&self, line: &String) -> u32 {
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
        first + second
    }

    fn number_to_string(&self, input: &str) -> String {
        let mut all_string_numbers = input.to_string();
        let number_map = HashMap::from([
            ("1", ",one,"),
            ("2", ",two,"),
            ("3", ",three,"),
            ("4", ",four,"),
            ("5", ",five,"),
            ("6", ",six,"),
            ("7", ",seven,"),
            ("8", ",eight,"),
            ("9", ",nine,"),
        ]);
        for num in 1..10 {
            let num_str = num.to_string();
            let num_str_value = number_map.get(&num_str.as_str()).unwrap();
            all_string_numbers = all_string_numbers.replace(&num_str, &&num_str_value.to_string());
        }
        all_string_numbers
    }

    fn solve_1(&self, input: &Vec<String>) -> SolutionResult {
        let sum = input.iter().fold(0, |acc: u32, line: &String| {
            acc + self.get_valid_number(line)
        });
        Ok(sum.to_string())
    }

    fn solve_2(&self, input: &Vec<String>) -> SolutionResult {
        let string_numbers = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let sum = input.iter().fold(0, |acc: u32, line: &String| {
            let valid_line = String::from(self.number_to_string(line));
            let mut output = String::from("");
            let mut start = 0;
            let mut end = valid_line.len();

            // last character
            while end > start {
                let is_match = string_numbers.iter().find(|string_number: &&&str| {
                    let string_number = *string_number;
                    valid_line[..end].ends_with(string_number)
                });
                if is_match.is_some() {
                    let string_number = is_match.unwrap();
                    let string_number_value = 1 + string_numbers.iter().position(|&r| r == *string_number).unwrap();
                    output.push_str(&valid_line[start..(end - string_number.len())]);
                    output.push_str((string_number_value).to_string().as_str());
                    output.push_str(&valid_line[end..]);
                    end -= string_number.len();
                    break;
                } else {
                    end -= 1;
                }
            }

            // first character
            while start < end {
                let is_match = string_numbers.iter().find(|string_number: &&&str| {
                    let string_number = *string_number;
                    valid_line[start..].starts_with(string_number)
                });
                if is_match.is_some() {
                    let string_number = is_match.unwrap();
                    let string_number_value = 1 + string_numbers.iter().position(|&r| r == *string_number).unwrap();
                    output = output.replacen(string_number, &string_number_value.to_string(), 1);
                    break;
                } else {
                    start += 1;
                }
            }

            if output.len() == 0 {
                output = valid_line.clone();
            }
            
            acc + self.get_valid_number(&output)
        });
        Ok(sum.to_string())
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
