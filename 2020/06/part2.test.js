const { solution, questionsAnsweredByAll } = require("./part2")

const test = require("ava")

test("questionsAnsweredByAll", t => {
  t.is(questionsAnsweredByAll("abc"), 3)
  t.is(questionsAnsweredByAll("a\nb\nc"), 0)
  t.is(questionsAnsweredByAll("ab\nac"), 1)
  t.is(questionsAnsweredByAll("a\na\na\na"), 1)
  t.is(questionsAnsweredByAll("b"), 1)
})

test("full solution", t => {
  const count = solution()
  t.truthy(count)
  // console.log(count)
})
