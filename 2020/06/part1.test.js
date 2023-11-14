const { solution, countQuestions, countQuestionsInGroups } = require("./part1")

const test = require("ava")

test("example group", t => {
  const group = "abcx\nabcy\nabcz"
  t.is(countQuestions(group), 6)
})

test("example groups", t => {
  const groups = `abc

a
b
c

ab
ac

a
a
a
a

b`
  t.is(countQuestionsInGroups(groups), 11)
})

test("full solution", t => {
  const best = solution()
  t.truthy(best)
  console.log(best)
})
