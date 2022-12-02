use anyhow::Result;
use std::cmp;
use std::fs;

fn get_file_lines() -> Result<Vec<String>> {
    let contents = fs::read_to_string("data/day_1.txt")?;
    let lines: Vec<String> = contents.split('\n').map(String::from).collect();
    Ok(lines)
}

// enum to hold the parser state for the little state machine parser
enum Elf {
    NotStarted,
    Parsing(i32),
    Finished(i32),
}

fn parse_file(lines: &Vec<String>) -> Result<Vec<i32>> {
    let mut elves: Vec<i32> = vec![];

    let mut elf = Elf::NotStarted;

    for line in lines {
        let next_elf = parse_line(line, elf)?;

        if let Elf::Finished(num) = next_elf {
            elves.push(num);
        }
        elf = next_elf;
    }

    Ok(elves)
}

fn parse_line(line: &str, elf: Elf) -> Result<Elf> {
    // first handle the blank line case
    if line.is_empty() {
        if let Elf::Parsing(num) = elf {
            return Ok(Elf::Finished(num));
        } else {
            return Err(anyhow!(
                "I should have an elf-in-progress when I reach a blank line!"
            ));
        }
    }

    let num = str::parse::<i32>(line)?;

    match elf {
        Elf::NotStarted => Ok(Elf::Parsing(num)),
        Elf::Parsing(prev) => Ok(Elf::Parsing(prev + num)),
        Elf::Finished(_) => Ok(Elf::Parsing(num)),
    }
}

pub fn solution() -> Result<()> {
    let lines = get_file_lines()?;

    let mut elves = parse_file(&lines)?;

    // day 1, part 1
    let most_caloric_elf = elves.iter().reduce(cmp::max);
    if let Some(calories) = most_caloric_elf {
        println!(
            "Day 1, part 1: the most calorie-laden elf has {} calories",
            calories
        );
    } else {
        return Err(anyhow!("Something went wrong :/"));
    }

    // day 1, part 2
    elves.sort();

    let top_three_elves_calories: i32 = elves.iter().rev().take(3).sum();

    println!(
        "Day 1, part 2: the top three elves have {} calories",
        top_three_elves_calories
    );

    Ok(())
}
