use std::fs;

struct Position {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Position {
    fn init() -> Position {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn make_move(self, next_move: Move) -> Position {
        match next_move {
            Move::Forward(magnitude) => Position {
                horizontal: self.horizontal + magnitude,
                ..self
            },
            Move::Down(magnitude) => Position {
                depth: self.depth + magnitude,
                ..self
            },
            Move::Up(magnitude) => Position {
                depth: self.depth - magnitude,
                ..self
            },
        }
    }

    fn make_pt2_move(self, next_move: Move) -> Position {
        match next_move {
            Move::Forward(magnitude) => Position {
                horizontal: self.horizontal + magnitude,
                depth: self.depth + self.aim * magnitude,
                ..self
            },
            Move::Down(magnitude) => Position {
                aim: self.aim + magnitude,
                ..self
            },
            Move::Up(magnitude) => Position {
                aim: self.aim - magnitude,
                ..self
            },
        }
    }
}

enum Move {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Move {
    fn new(action: &str, magnitude: u32) -> Option<Move> {
        match action {
            "forward" => Some(Move::Forward(magnitude)),
            "down" => Some(Move::Down(magnitude)),
            "up" => Some(Move::Up(magnitude)),
            &_ => None,
        }
    }
}

fn get_data() -> Vec<Move> {
    fs::read_to_string("src/data/day_2.txt")
        .expect("couldn't read the file")
        .lines()
        .map(|line| {
            let (action, magnitude) = line.split_once(" ").expect("should split once on a space");
            let magnitude: u32 = magnitude.parse().expect("numbers should parse");
            Move::new(action, magnitude).expect("lines should map cleanly to a Move")
        })
        .collect()
}

pub fn part_one() -> u32 {
    let course = get_data();

    let final_position = course
        .into_iter()
        .fold(Position::init(), |current_position, next_move| {
            current_position.make_move(next_move)
        });

    return final_position.horizontal * final_position.depth;
}

pub fn part_two() -> u32 {
    let course = get_data();

    let final_position = course
        .into_iter()
        .fold(Position::init(), |current_position, next_move| {
            current_position.make_pt2_move(next_move)
        });

    return final_position.horizontal * final_position.depth;
}
