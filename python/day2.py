#!/usr/bin/python3
import itertools
from pyparsing import Word, Char, nums, alphas, Suppress, Literal
from collections import Counter

def fn(s, loc, toks):
    return [int(toks[0])]

def count(s, loc, toks):
    return [Counter(toks[0])]

def is_valid(data):
    low, hi, char, counts = data
    c = counts[char]
    
    return (low <= c <= hi)

def is_valid_v2(data):
    pos1, pos2, char, string = data

    x = (string[pos1-1] == char)
    y = (string[pos2-1] == char)

    return x ^ y

def part1():

    policy = Word(nums).setParseAction(fn) + Suppress(Literal('-')) + Word(nums).setParseAction(fn) + Word(alphas) + Suppress(Literal(':')) + Word(alphas).setParseAction(count)

    valid = 0

    with open("../input/2020/day2.txt", 'r') as f:
        for line in f:
            if is_valid( policy.parseString(line) ):
                valid += 1

    print(f"Day 2 Part 1: {valid}")

def part2():

    policy = Word(nums).setParseAction(fn) + Suppress(Literal('-')) + Word(nums).setParseAction(fn) + Word(alphas) + Suppress(Literal(':')) + Word(alphas)

    valid = 0

    with open("../input/2020/day2.txt", 'r') as f:
        for line in f:
            if is_valid_v2( policy.parseString(line) ):
                valid += 1

    print(f"Day 2 Part 2: {valid}")



def main():
    part1()
    part2()
    
if __name__ == "__main__":
    main()
