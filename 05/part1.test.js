const { solution, seatToRow, seatToCol, seatID } = require("./part1")

const test = require("ava")

test("example seat number", t => {
  const exampleSeatNumber = "FBFBBFFRLR"
  t.is(seatToRow(exampleSeatNumber.slice(0, 7)), 44)
  t.is(seatToCol(exampleSeatNumber.slice(7)), 5)
  t.is(seatID(exampleSeatNumber), 357)
})

test("full solution", t => {
  const best = solution()
  t.truthy(best)
  // console.log(seatID(best))
})
