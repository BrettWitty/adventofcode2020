#!/usr/bin/python3


def part1():

    with open("../input/2020/day3.txt", 'r') as f:

        j = 0
        dj = 3
        trees = 0

        for i, l in enumerate(f):

            width = len(l.strip())
            if l[j % width] == '#':
                trees += 1

            j += dj


    print(f"Day 3, Part 1: {trees}")

def part2():

    with open("../input/2020/day3.txt", 'r') as f:

        lines = f.readlines()

    total = 1

    for (di,dj) in [ (1,1), (1,3), (1,5), (1,7), (2,1)]:

        j = 0
        trees = 0

        width = len(lines[0].strip())

        for i in range(0,len(lines),di):

            l = lines[i]

            if l[j % width] == '#':
                trees += 1

            j += dj

        total *= trees


    print(f"Day 3, Part 1: {total}")

def main():
    part1()
    part2()

if __name__ == "__main__":
    main()
