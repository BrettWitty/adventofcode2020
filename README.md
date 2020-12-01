# Advent of Code 2020

These are my attempts at the [Advent of Code 2020](https://adventofcode.com/2020). I'm doing it primarily in Rust to get my skills up.

I'm using:
- [cargo-aoc](https://github.com/gobanos/cargo-aoc) to automate all the boring parts.


## Python version

Under `python/` I'm attempting the same thing (without automation) in Python because I'm super-comfortable there.


## Day 1 solution

Hitting this problem directly isn't too hard. Find all combinations. Filter on `a+b==2020` and early-return with `a*b`.

You can however spend a little time in the file-reading to partition the numbers into odds and evens. Then you can search a smaller space because 2020 is even.

2020 (Even) = Even + Even = Odd + Odd

Also:
Even = Even + Even + Even = Even + Odd + Odd

So you can iterate on triples of evens, or pairs of odds with an even.
