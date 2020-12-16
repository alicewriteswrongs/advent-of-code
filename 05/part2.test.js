const { solution } = require("./part2")

const test = require("ava")

test("full solution", t => {
  const mySeat = solution()
  t.truthy(mySeat)
  // console.log(mySeat)
})
