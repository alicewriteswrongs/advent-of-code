const { solution } = require("./part2")

const test = require("ava")

test("solution", t => {
  const [firstNumber, secondNumber, thirdNumber] = solution()
  t.is(firstNumber + secondNumber + thirdNumber, 2020)
})
