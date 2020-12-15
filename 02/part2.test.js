const { solution } = require("./part2")

const test = require("ava")

test("solution", t => {
  const numberValid = solution()
  t.is(numberValid, 489)
})
