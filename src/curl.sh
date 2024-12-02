#!/bin/bash

function run {
  curl "https://adventofcode.com/2024/day/$1/input" -H "Cookie: session=$AOC_SESSION_TOKEN" > "src/days/day$1/input$1.txt"
}

run $1
