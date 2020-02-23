# Advent of Code 2019
Here is yet another repository with solutions for the [Advent of Code 2019](https://adventofcode.com/2019) problems. I did them in Rust, a language I started learning only recently.

The solutions were designed to be self-contained (using _zero_ external crates), reasonably general (working on every valid input), compact and performant, without going overboard with error handling, custom types or traits.

## Highlights
 - A pretty generic approach for graph-related problems, reusing the same BFS/shortest-path code for day 15 (part 2), day 18, day 20, and reusing the same DFS/backtracking code for day 15 (part 1), day 17, day 25.
 - A clean implementation of the FFT of day 16, which gives the exact solution in O(n log(n)) time for every offset.
 - A generic solver for the text adventure of day 25.

## Usage

The inputs are expected in the [input](./input/) folder.

Run any of the solutions with `cargo run --release --bin DAY_NUMBER`, or run all with `./run_all.sh`.

Run the tests with `cargo test`.
