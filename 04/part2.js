const fs = require("fs")

const getData = () =>
  String(fs.readFileSync("./data/passports.txt"))
    .trim()
    .split("\n\n")
    .map(passport => passport.split("\n").join(" "))
    .map(passport => passport.split(" ").map(entry => entry.split(":")))
    .map(Object.fromEntries)

// FIELDS
const BIRTH_YEAR = "byr"
const ISSUE_YEAR = "iyr"
const EXP_YEAR = "eyr"
const HEIGHT = "hgt"
const HAIR_COLOR = "hcl"
const EYE_COLOR = "ecl"
const PASSPORT_ID = "pid"
const COUNTRY_ID = "cid"

const VALID_EYE_COLORS = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]

const numberBetween = (low, high) => n => low <= Number(n) && Number(n) <= high

const heightValidation = height => {
  if (height.endsWith("cm")) {
    return numberBetween(150, 193)(Number(height.replace("cm", "")))
  }
  if (height.endsWith("in")) {
    return numberBetween(59, 76)(Number(height.replace("in", "")))
  }
  return false
}

const VALIDATION = [
  [BIRTH_YEAR, numberBetween(1920, 2002)],
  [ISSUE_YEAR, numberBetween(2010, 2020)],
  [EXP_YEAR, numberBetween(2020, 2030)],
  [HEIGHT, heightValidation],
  [HAIR_COLOR, hcl => /^#[0-9a-f]{6}$/.test(hcl)],
  [EYE_COLOR, ecl => VALID_EYE_COLORS.includes(ecl)],
  [PASSPORT_ID, pid => /^[0-9]{9}$/.test(pid)]
]

const isValid = passport =>
  VALIDATION.every(([field, validator]) => {
    const value = passport[field] ?? false
    return value && validator(value)
  })

const solution = passports => passports.filter(isValid).length

module.exports = { solution, getData }
