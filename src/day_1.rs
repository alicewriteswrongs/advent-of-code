use std::fs;

fn get_data() -> Vec<i32> {
    fs::read_to_string("src/data/day_1.txt")
        .expect("some issue reading the file :/")
        .lines()
        .map(|intstring| intstring.parse().unwrap())
        .collect()
}

fn count_increases(measurements: Vec<i32>) -> i32 {
    measurements
        .iter()
        .enumerate()
        .fold(0, |acc, (index, depth)| match index {
            0 => 0,
            x => match measurements[x - 1] < *depth {
                true => acc + 1,
                false => acc,
            },
        })
}

pub fn part_one() -> i32 {
    let measurements: Vec<i32> = get_data();
    count_increases(measurements)
}

pub fn part_two() -> i32 {
    let measurements: Vec<i32> = get_data();
    let window_sums = measurements
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();
    count_increases(window_sums)
}
