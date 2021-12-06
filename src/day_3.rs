use std::fs;

fn get_data() -> Vec<Move> {
    fs::read_to_string("src/data/day_3.txt")
        .expect("couldn't read the file")
        .lines()
        .collect()
}
