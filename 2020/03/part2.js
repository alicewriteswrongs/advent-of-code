const fs = require("fs")

const getData = () =>
  String(fs.readFileSync("./data/tree_map.txt"))
    .trim()
    .split("\n")

const TREE = "#"

// COORDS
//
//   0 1 2 3 - x coord
// 0 . # . #
// 1 . . # .
// 2 . # . .
// 3 # # . .
// -
// y coord
//
// and so on

const move = (currentLocation, direction) => {
  const { x, y } = currentLocation
  const { down, right } = direction

  return {
    x: x + right,
    y: y + down
  }
}

const isTree = (currentLocation, treeMap) => {
  const { x, y } = currentLocation
  const value = treeMap[y][x % treeMap[y].length]
  return value === TREE
}

const checkForMove = (treeMap, moveToCheck) => {
  let currentLocation = { x: 0, y: 0 }
  let treesEncountered = 0

  while (currentLocation.y < treeMap.length) {
    if (isTree(currentLocation, treeMap)) {
      treesEncountered++
    }
    currentLocation = move(currentLocation, moveToCheck)
  }
  return treesEncountered
}

const solution = treeMap => {
  const results = [
    checkForMove(treeMap, { right: 1, down: 1 }),
    checkForMove(treeMap, { right: 3, down: 1 }),
    checkForMove(treeMap, { right: 5, down: 1 }),
    checkForMove(treeMap, { right: 7, down: 1 }),
    checkForMove(treeMap, { right: 1, down: 2 })
  ]
  return results.reduce((acc, x) => acc * x, 1)
}

module.exports = { solution, getData }
