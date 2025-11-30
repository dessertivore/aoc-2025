# Advent of Code 2025

Welcome to my solutions for [Advent of Code 2025](https://adventofcode.com/2025)! 🎄

## About

This repository contains my solutions to the 2025 edition of Advent of Code. Each day features two parts, and I'll aim to complete them using clean, efficient, and well-documented code. This is my biggest foray into Rust so far, so not sure how it'll go!!

## Structure

The repository is organized as follows:

```
aoc-2025/
├── src/
│   ├── solutions/
│   │   ├── day_1.rs
│   │   ├── day_2.rs
│   ├── utils/
│   │   ├── mod.rs
│   └── main.rs
└── README.md
```

- Each `day_XX.rs` file contains solutions for that day's puzzles, and maybe notes about it.
- I will be writing unit tests for each day too. I tend to prefer completing AOC using semi-TDD.

## Running the Code

The following code will print all the solutions I have so far to the console.

Note: requires `cookie.txt` file in this directory, which contains your AoC user cookie
in order to get your inputs.

```bash
cargo build
cargo run
```

To build the docs, run:
```bash
cargo doc && cargo doc --open
```