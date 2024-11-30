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


def gear_ratio(matrix, numbers, gears):
    sum = 0
    gear_number = []
    for x, y in gears:
        for i in range(x - 1, x + 2):
            for j in range(y - 1, y + 2):
                if not is_valid(i, j, matrix):
                    continue
                if matrix[i][j].isdigit():
                    for number in numbers:
                        if i == int(number[1]) and j in number[2]:
                            if number not in gear_number:
                                gear_number.append(number)
        if len(gear_number) == 2:
            sum += int(gear_number[0][0]) * int(gear_number[1][0])
        gear_number = []

    print(sum)


def is_part(matrix, x, index):
    symbols = ["+", "*", "=", "-", "&", "#", "/", "%", "$", "@"]
    is_symbol = False

    for i in range(x - 1, x + 2):
        for y in range(index[0] - 1, index[-1] + 2):
            if not is_valid(i, y, matrix):
                continue
            if matrix[i][y] in symbols:
                is_symbol = True

    return is_symbol


def calculate_schematics(matrix: list[list[str]]):
    numbers = []
    break_point = [".", "+", "*", "=", "-", "&", "#", "/", "%", "$", "@"]
    digit = ""
    index = []
    gear = []
    for x, row in enumerate(matrix):
        for y, char in enumerate(row):
            if char.isdigit():
                digit += char
                index.append(y)

            if char in break_point and len(digit) > 0:
                if is_part(matrix, x, index):
                    numbers.append([digit, x, index])
                digit = ""
                index = []

            if char == "*":
                gear.append((x, y))

        # fucking hell this fucking fuck shit took me fucking hours fucking fuck
        if len(digit) > 0 and is_part(matrix, x, index):
            numbers.append([digit, x, index])

        digit = ""
        index = []
    return numbers, gear


def engine_schematic() -> None:
    with open("day_3_input.txt", "r") as file:
        matrix = [list(line.strip()) for line in file.readlines()]
        value, gear = calculate_schematics(matrix)
    sum = 0
    for i in value:
        sum += int(i[0])
    print(sum)

    gear_ratio(matrix, value, gear)


if __name__ == "__main__":
    engine_schematic()
