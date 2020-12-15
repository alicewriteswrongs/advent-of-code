const fs = require("fs")

const get_data = () =>
  String(fs.readFileSync("./data/expense_report.txt"))
    .trim()
    .split("\n")
    .map(Number)

const solution = () => {
  const report = get_data()

  for (let x of report) {
    for (let y of report) {
      if (x + y < 2020) {
        for (let z of report) {
          if (x + y + z === 2020) {
            return [x, y, z]
          }
        }
      }
    }
  }
  return [1, 1, 1]
}

module.exports = { solution }
