example = [
    "467..114..",
    "...*......",
    "..35..633.",
    "......#...",
    "617*......",
    ".....+.58.",
    "..592.....",
    "......755.",
    "...$.*....",
    ".664.598..",
]
example2 = [
    "467..114..",
    "...*......",
    "..35..633.",
    "......#...",
    "617*......",
    ".....+.58.",
    "..592.....",
    "......755.",
    "...$.*....",
    ".664.598..",
]


def is_valid(x, y, matrix):
    if x < 0 or y < 0:
        return False
    try:
        matrix[x][y]
        return True
    except IndexError:
        return False


def is_part(matrix, x, index):
    symbols = ["+", "*", "=", "-", "&", "#", "/", "%", "$", "@"]
    is_symbol = False

    for i in range(x - 1, x + 2):
        for y in range(index[0] - 1, index[-1] + 2):
            if not is_valid(i, y, matrix):
                continue
            if matrix[i][y] in symbols:
                is_symbol = True
            if matrix[i][y] == '*' and i >= x:
                


    return is_symbol


def calculate_schematics(matrix: list[list[str]]):
    numbers = []
    break_point = [".", "+", "*", "=", "-", "&", "#", "/", "%", "$", "@"]
    digit = ""
    index = []
    for x, row in enumerate(matrix):
        for y, char in enumerate(row):
            if char.isdigit():
                digit += char
                index.append(y)

            if char in break_point and len(digit) > 0:
                if is_part(matrix, x, index):
                    numbers.append(digit)
                digit = ""
                index = []
        # fucking hell this fucking fuck shit took me fucking hours fucking fuck
        if len(digit) > 0 and is_part(matrix, x, index):
            numbers.append(digit)
        digit = ""
        index = []
    return numbers


def engine_schematic() -> None:
    # with open("day_3_input.txt", "r") as file:
    matrix = []
    for line in example2:
        # matrix = [list(line.strip()) for line in file.readlines()]
        matrix.append(line)
    value = calculate_schematics(matrix)
    # print(value)
    # print(len(value))
    sum = 0
    for i in value:
        # print(i)
        sum += int(i)
    print(sum)


if __name__ == "__main__":
    engine_schematic()
