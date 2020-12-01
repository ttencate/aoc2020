Advent of Code 2018
===================

This repository contains my solutions for the
[Advent of Code 2020](https://adventofcode.com/2020).

There's some simple code to fetch today's input and place it in the right
directory. It requires a file called `.session_cookie` containing the value of
the `session` cookie from a valid AoC login session. If input files are not
found, they will be fetched automatically.

The solution to each day's puzzle can be run with:

    cargo run --bin XX

where `XX` is the zero-padded day number.
