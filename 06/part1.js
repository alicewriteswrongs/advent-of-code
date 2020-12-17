const fs = require("fs")

const getData = () =>
  String(fs.readFileSync("./data/group_questions.txt")).trim()

const countQuestions = group => new Set(group.replace(/\W/g, "").split("")).size

const countQuestionsInGroups = groups =>
  groups
    .split("\n\n")
    .map(countQuestions)
    .reduce((acc, x) => acc + x)

const solution = () => countQuestionsInGroups(getData())

module.exports = { solution, countQuestions, countQuestionsInGroups }
