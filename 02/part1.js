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

  const letterCount = [...pw.matchAll(new RegExp(letter, "g"))].length

  return range[0] <= letterCount && letterCount <= range[1]
}

const solution = () => {
  const passwords = getData()
  return passwords.filter(isValidPassword).length
}

module.exports = { solution }
