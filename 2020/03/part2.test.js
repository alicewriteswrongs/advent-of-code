const { solution, getData } = require("./part2")

const test = require("ava")

test("example data", t => {
  const treeMap = [
    "..##.......",
    "#...#...#..",
    ".#....#..#.",
    "..#.#...#.#",
    ".#...##..#.",
    "..#.##.....",
    ".#.#.#....#",
    ".#........#",
    "#.##...#...",
    "#...##....#",
    ".#..#...#.#"
  ]

  const treesEncounteredMultiplied = solution(treeMap)
  t.is(treesEncounteredMultiplied, 336)
})

test("full solution", t => {
  const treeMap = getData()
  const treesEncountered = solution(treeMap)
  t.truthy(treesEncountered)
  // console.log(treesEncountered)
})
