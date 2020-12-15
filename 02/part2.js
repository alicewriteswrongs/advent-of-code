const fs = require("fs")

const getData = () =>
  String(fs.readFileSync("./data/passwords.txt"))
    .trim()
    .split("\n")
    .map(str => str.split(" "))
    .map(([range, letter, password]) => [
      range.split("-").map(Number),
      letter.replace(":", ""),
      password
    ])

const isValidPassword = password => {
  const [range, letter, pw] = password
  const [lower, upper] = range

  return Boolean((pw[lower - 1] === letter) ^ (pw[upper - 1] === letter))
}

const solution = () => {
  const passwords = getData()
  return passwords.filter(isValidPassword).length
}

module.exports = { solution }
