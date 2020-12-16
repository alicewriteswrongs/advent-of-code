const fs = require("fs")

const getData = () =>
  String(fs.readFileSync("./data/boarding_passes.txt"))
    .trim()
    .split("\n")

const binSearch = (left, right, lower, upper) => seat => {
  const [low, high] = seat.split("").reduce(
    ([low, high], letter) => {
      if (letter === left) {
        return [low, low + Math.floor((high - low) / 2)]
      }
      if (letter === right) {
        return [low + Math.ceil((high - low) / 2), high]
      }
    },
    [lower, upper]
  )
  return low
}

const seatToRow = binSearch("F", "B", 0, 127)

const seatToCol = binSearch("L", "R", 0, 7)

const seatID = seatNumber =>
  seatToRow(seatNumber.slice(0, 7)) * 8 + seatToCol(seatNumber.slice(7))

const solution = () =>
  getData().reduce((bestSoFar, seatNumber) =>
    seatID(seatNumber) > seatID(bestSoFar) ? seatNumber : bestSoFar
  )

module.exports = { solution, seatToRow, seatToCol, seatID }
