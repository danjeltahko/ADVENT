"""
Actually I couldn't solve this, so the code is copied from 
https://cutonbuminband.github.io/AOC/qmd/2023.html
"""

import functools


def get_map(txt):
    with open(txt, "r") as file:
        map = []
        for line in file.readlines():
            spring = line.strip().split(" ")[0]
            block = line.strip().split(" ")[1].strip()
            group = tuple(int(b) for b in block.split(","))
            lookup = {"#": 2, "?": 1, ".": 0}
            row = tuple(lookup[char] for char in spring)
            map.append((row, group))

        return map


def match_beginning(data, length):
    return all(x > 0 for x in data[:length]) and (
        (len(data) == length) or data[length] < 2
    )


@functools.cache
def brap(row, block):
    total_blocks = sum(block)
    min = sum(x == 2 for x in row)
    max = sum(x > 0 for x in row)

    if min > total_blocks or max < total_blocks:
        return 0
    if total_blocks == 0:
        return 1
    if row[0] == 0:
        return brap(row[1:], block)
    if row[0] == 2:
        l = block[0]
        if match_beginning(row, l):
            if l == len(row):
                return 1
            return brap(row[l + 1 :], block[1:])
        return 0
    return brap(row[1:], block) + brap((2,) + row[1:], block)


def broken_springs():
    map = get_map("day_12_input.txt")
    sum = 0
    for row, block in map:
        sum += brap(((row + (1,)) * 5)[:-1], block * 5)

    print("totti ", sum)


if __name__ == "__main__":
    broken_springs()
