Advent of Code 2020
===================

This repository contains my solutions for the
[Advent of Code 2020](https://adventofcode.com/2020).

There's some code to fetch today's input and place it in the right directory.
The function `aoc::input()` makes it available as a `String`.

Similarly, example inputs are parsed from the problem description and made
available as `aoc::example(0)`, `aoc::example(1)` and so on.

These mechanisms requires a file called `.session_cookie` containing the value
of the `session` cookie from a valid AoC login session; if not found, the
cookie value will be prompted for. If input files are not found, they will be
fetched automatically.

The solution to each day's puzzle can be run with:

    cargo run --bin XX

where `XX` is the zero-padded day number.
