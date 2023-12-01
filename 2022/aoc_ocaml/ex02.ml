open Core

let get_data =
  In_channel.read_lines "./data/day_2.txt" |> List.map ~f:String.strip

type outcome = Win | Loss | Draw

type move = Rock | Paper | Scissors

type game = move * move 

let game_outcome game = match game with
    | (Rock, Paper) -> Win
    | (Rock, Scissors) -> Loss
    | (Rock, Rock) -> Draw
    | (Paper, Scissors) -> Win
    | (Paper, Rock) -> Loss
    | (Paper, Paper) -> Draw
    | (Scissors, Rock) -> Win
    | (Scissors, Paper) -> Loss
    | (Scissors, Scissors) -> Draw

let outcome_score game = match game_outcome game with
    | Win -> 6
    | Loss -> 0
    | Draw -> 3

let move_score move = match move with
    | Rock -> 1
    | Paper -> 2
    | Scissors -> 3

let game_score game = move_score game + outcome_score game

let parse_game_part_one line =
    let chars = String.split ~on:' ' line in
    let enemy_move move = match move with
    | "A" -> Rock
    | "B" -> Paper
    | "C" -> Scissors 
    | "" -> throw in
    let my_move move = match move with
    let  = match List.hd chars with
    | Some "A" ->
