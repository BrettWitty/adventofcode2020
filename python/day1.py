#!/usr/bin/python3
import itertools
from math import prod

def part(N):

    numbers = []

    with open("../input/2020/day1.txt", 'r') as f:
        for line in f:
            numbers.append( int(line.strip()) )

    for v in itertools.combinations(numbers,N):
        if sum(v) == 2020:
            print(f"Day 1, Part 1: {prod(v)}")
            return

def part1():
    part(2)

def part2():
    part(3)
        
def main():

    part1()
    part2()

if __name__ == "__main__":
    main()
