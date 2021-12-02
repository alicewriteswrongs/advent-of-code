use std::fs;

pub fn solution() -> i32 {
    let contents: Vec<i32> = fs::read_to_string("src/data/day_1.txt")
        .unwrap()
        .trim()
        .split("\n")
        .map(|intstring| intstring.parse::<i32>().unwrap())
        .collect();

    contents
        .iter()
        .enumerate()
        .fold(0, |acc, (index, depth)| match index {
            0 => 0,
            x => match contents[x - 1] < *depth {
                true => acc + 1,
                false => acc,
            },
        })
}
