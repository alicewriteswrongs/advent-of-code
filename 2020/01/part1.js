const fs = require("fs")

const getData = () =>
  String(fs.readFileSync("./data/expense_report.txt"))
    .split("\n")
    .map(Number)

const solution = () => {
  const report = getData()
  const inverse = report.map(x => 2020 - x)
  return report.filter(x => inverse.includes(x))
}

module.exports = { solution }
