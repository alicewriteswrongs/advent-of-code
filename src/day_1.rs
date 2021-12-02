use std::fs;

fn get_data() -> Vec<i32> {
    fs::read_to_string("src/data/day_1.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|intstring| intstring.parse::<i32>().unwrap())
        .collect()
}

fn count_increases(measurements: Vec<i32>) -> i32 {
    measurements.iter().enumerate()
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

    let mut idx: usize = 0;
    let mut window_sums: Vec<i32> = vec![];

    while idx + 2 < measurements.len() {
        window_sums.push(
            measurements[idx] + measurements[idx + 1] + measurements[idx + 2]
        );
        idx += 1;
    };

    count_increases(window_sums)
}
