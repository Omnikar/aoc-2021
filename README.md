# Advent of Code 2021
My solutions to [Advent of Code 2021](https://adventofcode.com/2021).

## Usage
To build: `cargo build --release`  
To run a solution: `aoc-2021 <day> <part>` e.g. `aoc-2021 day1 part1`

Puzzle input is accepted through stdin, and result is written to stdout.

Add `--time` as the first argument to have the calculation time outputted in μs.

### Currently completed puzzles
A star (*) next to a day means that there is a revised solution present, which was not
what I used to first solve the puzzle, but what I deem a better solution/implementation
which I came up with after initially solving the puzzle. Revised solutions are accessible
by appending `_revised` to the day, e.g., `aoc-2021 day1_revised part1`. All non-revised
solutions are the (almost) exact original code (some adjustments may have been made to the
organization of the parts, which consequently minorly affected the code for the parts
themselves), including `cargo clippy` violations, which will not be fixed outside of
revised versions; as a result, there are `#[allow(…)]` attributes applied to some of the
original solutions to disable the violated lints.

* [x] [Day 1](src/days/day1.rs)[*](src/days/day1_revised.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 2](src/days/day2.rs)[*](src/days/day2_revised.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 3](src/days/day3.rs)[*](src/days/day3_revised.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 4](src/days/day4.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 5](src/days/day5.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 6](src/days/day6.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 7](src/days/day7.rs)[*](src/days/day7_revised.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 8](src/days/day8.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 9](src/days/day9.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 10](src/days/day10.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 11](src/days/day11.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 12](src/days/day12.rs)[*](src/days/day12_revised.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 13](src/days/day13.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 14](src/days/day14.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 15](src/days/day15.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 16](src/days/day16.rs)
  * [x] Part 1
  * [x] Part 2
* [ ] Day 17
  * [ ] Part 1
  * [ ] Part 2
* [x] [Day 18](src/days/day18.rs)
  * [x] Part 1
  * [x] Part 2
* [ ] Day 19
  * [ ] Part 1
  * [ ] Part 2
* [x] [Day 20](src/days/day20.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 21](src/days/day21.rs)
  * [x] Part 1
  * [x] Part 2
* [x] [Day 22](src/days/day22.rs)
  * [x] Part 1
  * [x] Part 2
* [ ] Day 23
  * [ ] Part 1
  * [ ] Part 2
* [ ] Day 24
  * [ ] Part 1
  * [ ] Part 2
* [ ] [Day 25](src/days/day25.rs)
  * [x] Part 1
  * [ ] Part 2
