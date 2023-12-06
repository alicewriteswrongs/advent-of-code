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
    use std::collections::HashMap;

    // digits can be 1-9 _or_
    // 'one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', and 'nine'
    fn get_first_number(name_to_number: &HashMap<&str, char>, line: &str) -> Option<char> {
        let mut line_copy = line.to_string();

        while !line_copy.is_empty() {
            // first check for ascii digit
            let c = line_copy
                .chars()
                .next()
                .expect("should be able to get leading character");
            if c.is_ascii_digit() {
                return Some(c);
            }

            for (name, num) in name_to_number {
                if line_copy.starts_with(name) {
                    return Some(*num);
                }
            }
            line_copy = line_copy
                .get(1..)
                .expect("should have at least one character left")
                .to_string();
        }
        None
    }

    fn parse_line(line: &str) -> i32 {
        let name_to_number = HashMap::from([
            ("one", '1'),
            ("two", '2'),
            ("three", '3'),
            ("four", '4'),
            ("five", '5'),
            ("six", '6'),
            ("seven", '7'),
            ("eight", '8'),
            ("nine", '9'),
        ]);

        let first = get_first_number(&name_to_number, line).expect("should get a first num");
        let reversed = line.chars().rev().collect::<String>();

        let name_to_number = HashMap::from([
            ("eno", '1'),
            ("owt", '2'),
            ("eerht", '3'),
            ("ruof", '4'),
            ("evif", '5'),
            ("xis", '6'),
            ("neves", '7'),
            ("thgie", '8'),
            ("enin", '9'),
        ]);
        let last = get_first_number(&name_to_number, &reversed).expect("should get a second num");
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
