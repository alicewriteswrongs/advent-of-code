use anyhow::Result;
use std::fs;

pub fn get_file_lines(path: &str) -> Result<Vec<String>> {
    let contents = fs::read_to_string(path)?;
    let lines: Vec<String> = contents.split('\n').map(String::from).collect();
    Ok(lines)
}
