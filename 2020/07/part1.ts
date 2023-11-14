import {readFileSync} from "fs"

const getData = () =>
  String(readFileSync("./data/bag_rules.txt")).trim()
