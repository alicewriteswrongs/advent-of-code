open Core

let get_data =
  In_channel.read_lines "./data/day_1.txt" |> List.map ~f:String.strip

type parser_state = Init | ParsingElf of int

type elf = Elf of int

let parse (lines : string list) =
  let parse_line (state, elves) line =
    let empty = String.is_empty line in
    match state with
    | Init ->
        if empty then (state, elves)
        else (ParsingElf (Int.of_string line), elves)
    | ParsingElf sum ->
        if empty then (Init, elves @ [Elf sum])
        else (ParsingElf (Int.of_string line + sum), elves)
  in
  let parse_lines = List.fold ~f:parse_line ~init:(Init, []) in
  let get_elves (_, elves) = elves in
  lines |> parse_lines |> get_elves

let elf_to_string (Elf calories) = Int.to_string calories

let main () =
  let _result =
    get_data |> parse |> List.map ~f:elf_to_string |> List.map ~f:print_endline
  in
  ()

let () = main ()
