use anyhow::Result;
use std::fs;

fn get_file_lines() -> Result<Vec<String>> {
    let contents = fs::read_to_string("data/day1.txt")?;
    let lines: Vec<String> = contents.split('\n').map(String::from).collect();
    Ok(lines)
}

mod part_1 {
    use super::get_file_lines;
    use anyhow::Result;

    fn parse_line(line: &str) -> i32 {
        let nums: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();

        // just panic if it doesn't work
        let first = nums.first().expect("this should be present");
        let last = nums.last().expect("this should also be present!");
        format!("{}{}", first, last).parse().expect("should parse")
    }

    pub fn solution() -> Result<()> {
        let lines = get_file_lines()?;

        let mut sum = 0;

        for line in lines {
            if !line.is_empty() {
                let num = parse_line(&line);
                sum += num;
            }
        }

        println!(
            "Day 1, part 1: the sum of the calibration values is {}",
            sum
        );

        Ok(())
    }
}

mod part_2 {
    use super::get_file_lines;
    use anyhow::Result;

    fn parse_line(line: &str) -> i32 {
        let nums: Vec<char> = line.chars().filter(|c| c.is_ascii_digit()).collect();

        // just panic if it doesn't work
        let first = nums.first().expect("this should be present");
        let last = nums.last().expect("this should also be present!");
        format!("{}{}", first, last).parse().expect("should parse")
    }

    pub fn solution() -> Result<()> {
        let lines = get_file_lines()?;

        let mut sum = 0;

        for line in lines {
            if !line.is_empty() {
                let num = parse_line(&line);
                sum += num;
            }
        }

        println!(
            "Day 1, part 2: the sum of the calibration values is {}",
            sum
        );

        Ok(())
    }
}


pub fn solution() -> Result<()> {
    part_1::solution()?;
    part_2::solution()?;

    Ok(())
}
