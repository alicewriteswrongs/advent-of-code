const fs = require("fs")

const getData = () =>
  String(fs.readFileSync("./data/passports.txt"))
    .trim()
    .split("\n\n")
    .map(passport => passport.split("\n").join(" "))

// FIELDS
const BIRTH_YEAR = "byr"
const ISSUE_YEAR = "iyr"
const EXP_YEAR = "eyr"
const HEIGHT = "hgt"
const HAIR_COLOR = "hcl"
const EYE_COLOR = "ecl"
const PASSPORT_ID = "pid"
const COUNTRY_ID = "cid"

const isValid = passport =>
  passport.includes(BIRTH_YEAR) &&
  passport.includes(ISSUE_YEAR) &&
  passport.includes(EXP_YEAR) &&
  passport.includes(HEIGHT) &&
  passport.includes(HAIR_COLOR) &&
  passport.includes(EYE_COLOR) &&
  passport.includes(PASSPORT_ID)

const solution = passports => passports.filter(isValid).length

module.exports = { solution, getData }
