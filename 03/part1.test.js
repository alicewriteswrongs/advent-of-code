const { solution, getData } = require("./part1")

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

  const treesEncountered = solution(treeMap)
  t.is(treesEncountered, 7)
})

test("full solution", t => {
  const treeMap = getData()
  const treesEncountered = solution(treeMap)
  t.truthy(treesEncountered)
  // console.log(treesEncountered)
})
