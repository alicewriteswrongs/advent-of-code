use anyhow::Result;
use std::fs;

fn get_file_lines() -> Result<Vec<String>> {
    let contents = fs::read_to_string("data/day_2.txt")?;
    let lines: Vec<String> = contents.split('\n').map(String::from).collect();
    Ok(lines)
}

enum Move {
    Rock,
    Paper,
    Scissors,
}

fn score_for_move(chosen_move: &Move) -> i32 {
    match chosen_move {
        Move::Rock => 1,
        Move::Paper => 2,
        Move::Scissors => 3,
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

type Game = (Move, Move);

fn outcome_for_game(game: &Game) -> Outcome {
    match game {
        // draws
        (Move::Rock, Move::Rock) => Outcome::Draw,
        (Move::Paper, Move::Paper) => Outcome::Draw,
        (Move::Scissors, Move::Scissors) => Outcome::Draw,
        // opponent plays rock
        (Move::Rock, Move::Paper) => Outcome::Win,
        (Move::Rock, Move::Scissors) => Outcome::Lose,
        // opponent plays scissors
        (Move::Scissors, Move::Rock) => Outcome::Win,
        (Move::Scissors, Move::Paper) => Outcome::Lose,
        // opponent plays paper
        (Move::Paper, Move::Scissors) => Outcome::Win,
        (Move::Paper, Move::Rock) => Outcome::Lose,
    }
}

fn score_for_game(game: &Game) -> i32 {
    let (_, player_move) = game;

    let game_score = match outcome_for_game(game) {
        Outcome::Win => 6,
        Outcome::Draw => 3,
        Outcome::Lose => 0,
    };

    game_score + score_for_move(player_move)
}

type StrategyGuide = Vec<Game>;

fn parse_strategy_guide(lines: &Vec<String>) -> StrategyGuide {
    let mut strategy_guide: StrategyGuide = vec![];

    for line in lines {
        if !line.is_empty() {
            let game = parse_game(line);
            strategy_guide.push(game);
        }
    }
    strategy_guide
}

fn parse_game(line: &str) -> Game {
    let letters: Vec<char> = line.chars().collect();
    (enemy_move(letters[0]), player_move(letters[2]))
}

fn enemy_move(letter: char) -> Move {
    match letter {
        'A' => Move::Rock,
        'B' => Move::Paper,
        'C' => Move::Scissors,
        _ => todo!(),
    }
}

fn player_move(letter: char) -> Move {
    match letter {
        'X' => Move::Rock,
        'Y' => Move::Paper,
        'Z' => Move::Scissors,
        _ => todo!(),
    }
}

pub fn solution() -> Result<()> {
    let lines = get_file_lines()?;
    let strategy_guide = parse_strategy_guide(&lines);
    let score: i32 = strategy_guide.iter().map(score_for_game).sum();
    println!(
        "Day 2, part 1: score following the strategy guide is {}",
        score
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_example() {
        let lines = vec![
            String::from("A Y"),
            String::from("B X"),
            String::from("C Z"),
        ];

        let games = parse_strategy_guide(&lines);
        let sum: i32 = games.iter().map(score_for_game).sum();
        assert!(sum == 15);
    }
}
