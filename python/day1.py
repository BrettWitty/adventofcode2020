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

def part1_best():

    numbers = set()

    with open("../input/2020/day1.txt", 'r') as f:
        for line in f:
            numbers.add( int(line.strip()) )

    for v in numbers:
        if 2020-v in numbers:
            print(f"Day 1, Part 1 (Best): {v*(2020-v)}")
            return

def part2_best():

    odds = set()
    evens = set()

    with open("../input/2020/day1.txt", 'r') as f:
        for line in f:
            n = int(line)
            if n % 2 == 0:
                evens.add(n)
            else:
                odds.add(n)

    for a in evens:
        for b in odds:
            if 2020-(a+b) in odds:
                print(f"Day 1, Part 2 (Best): {a*b*(2020-(a+b))}")
                return



def part1():
    part(2)

def part2():
    part(3)

        
def main():

    part1()
    part2()
    part1_best()
    part2_best()

if __name__ == "__main__":
    main()
