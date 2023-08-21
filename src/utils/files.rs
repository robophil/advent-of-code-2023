use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_input_file_path(day: &str, part: &str) -> String {
    format!("src/inputs/day{}-{}.txt", day, part)
}

pub fn file_to_string_vec(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}
