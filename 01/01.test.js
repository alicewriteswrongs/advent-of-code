const { solution } = require("./01")

const test = require("ava")

test("solution", t => {
  const [firstNumber, secondNumber] = solution()
  t.is(firstNumber + secondNumber, 2020)
})
