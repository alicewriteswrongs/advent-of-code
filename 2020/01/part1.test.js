const { solution } = require("./part1")

const test = require("ava")

test("solution", t => {
  const [firstNumber, secondNumber] = solution()
  t.is(firstNumber + secondNumber, 2020)
})
