const { solution } = require("./part1")

const test = require("ava")

test("solution", t => {
  const numberValid = solution()
  t.truthy(numberValid)
})
