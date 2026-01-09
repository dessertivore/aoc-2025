# Advent of Code 2025

Welcome to my solutions for [Advent of Code 2025](https://adventofcode.com/2025)! 🎄

## About

This repository contains my solutions to the 2025 edition of Advent of Code. Each day features two parts, and I'll aim to complete them using clean, efficient, and well-documented code. This is my biggest foray into Rust so far, so not sure how it'll go!!

## Structure

The repository is organized as follows:

```
aoc-2025/
├── src/
│   ├── bin/
│   │   ├── day_1.rs
│   │   ├── day_2.rs
│   └── utils/
│       ├─── mod.rs
└── README.md
```

- Each `day_XX.rs` file contains solutions for that day's puzzles, and maybe notes about it.
- I will be writing unit tests for each day too. I tend to prefer completing AOC using semi-TDD.

## Running the Code

First, compile the code:
```bash
cargo build
```

Next, solve the day you want, using the `make` command `solve_day` with your day of 
choice. E.g.:
```bash
DAY=2 make solve_day
```

Note: requires `cookie.txt` file in this directory, which contains your AoC user cookie
in order to get your inputs.

To build the docs, run:
```bash
cargo doc && cargo doc --open
```

<!-- AOC TILES BEGIN -->
<h1 align="center">
  2025 - 16 ⭐ - Rust
</h1>
<a href="src/bin/day_1.rs">
  <img src=".aoc_tiles/tiles/2025/01.png" width="203px">
</a>
<a href="src/bin/day_2.rs">
  <img src=".aoc_tiles/tiles/2025/02.png" width="203px">
</a>
<a href="src/bin/day_3.rs">
  <img src=".aoc_tiles/tiles/2025/03.png" width="203px">
</a>
<a href="src/bin/day_4.rs">
  <img src=".aoc_tiles/tiles/2025/04.png" width="203px">
</a>
<a href="src/bin/day_5.rs">
  <img src=".aoc_tiles/tiles/2025/05.png" width="203px">
</a>
<a href="src/bin/day_6.rs">
  <img src=".aoc_tiles/tiles/2025/06.png" width="203px">
</a>
<a href="src/bin/day_7.rs">
  <img src=".aoc_tiles/tiles/2025/07.png" width="203px">
</a>
<a href="src/bin/day_8.rs">
  <img src=".aoc_tiles/tiles/2025/08.png" width="203px">
</a>
<a href="src/bin/day_9.rs">
  <img src=".aoc_tiles/tiles/2025/09.png" width="203px">
</a>
<a href="src/bin/day_10.rs">
  <img src=".aoc_tiles/tiles/2025/10.png" width="203px">
</a>
<a href="src/bin/day_11.rs">
  <img src=".aoc_tiles/tiles/2025/11.png" width="203px">
</a>
<a href="src/bin/day_12.rs">
  <img src=".aoc_tiles/tiles/2025/12.png" width="203px">
</a>
<!-- AOC TILES END -->