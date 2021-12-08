use std::fs;

#[derive(Debug)]
struct Board {
    board: [u32; 25],
}

#[derive(Debug)]
struct Game {
    moves: Vec<u32>,
    boards: Vec<Board>,
}

fn get_data() -> Game {
    let raw_string =
        fs::read_to_string("src/data/day_4.txt").expect("some problem reading the file! :O");

    let (raw_moves, raw_boards) = raw_string
        .split_once("\n\n")
        .expect("this should split just fine");

    Game {
        moves: raw_moves
            .split(",")
            .map(|numeral| numeral.parse().unwrap())
            .collect(),
        boards: raw_boards
            .split("\n\n")
            .map(|raw_board| Board {
                board: raw_board
                    .split("\n")
                    .flat_map(|row| row.split(" "))
                    .filter(|&str| str != "")
                    .map(|numeral| numeral.parse().unwrap())
                    .collect::<Vec<u32>>()
                    .try_into()
                    .expect("conversion to array panicked :("),
            })
            .collect(),
    }
}

pub fn part_one() -> u32 {
    let game = get_data();

    println!("{:?}", game.moves);
    println!("{:?}", game.boards);

    32
}
