const fs = require("fs")

const getData = () =>
  String(fs.readFileSync("./data/group_questions.txt"))
    .trim()
    .split("\n\n")

const intersection = (set1, set2) => new Set([...set1].filter(x => set2.has(x)))

const questionsAnsweredByAll = group =>
  group
    .split("\n")
    .map(individual => new Set(individual.trim().split("")))
    .reduce(intersection).size

const solution = () =>
  getData()
    .map(questionsAnsweredByAll)
    .reduce((acc, x) => acc + x)

module.exports = { solution, questionsAnsweredByAll }
