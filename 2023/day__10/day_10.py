def get_map(txt: str):
    with open(txt, "r") as file:
        map = []
        for line in file.readlines():
            row = [i for i in line if i.strip()]
            map.append(row)
        return map


def create_map():
    map = get_map("day_10_input.txt")
    starting_point: tuple
    for x, row in enumerate(map):
        for y, symbol in enumerate(row):
            if symbol == "S":
                starting_point = (x, y)

    return starting_point, map


def next(map: list, prev: tuple, atm: tuple):
    prev_x, prev_y = prev[0], prev[1]
    atm_x, atm_y = atm[0], atm[1]
    symbol = map[atm_x][atm_y]

    if symbol == "|":
        if prev_x < atm_x:
            return (atm_x, atm_y), (atm_x + 1, atm_y)
        else:
            return (atm_x, atm_y), (atm_x - 1, atm_y)

    elif symbol == "-":
        if prev_y < atm_y:
            return (atm_x, atm_y), (atm_x, atm_y + 1)
        else:
            return (atm_x, atm_y), (atm_x, atm_y - 1)

    elif symbol == "L":
        if prev_x < atm_x:
            return (atm_x, atm_y), (atm_x, atm_y + 1)
        else:
            return (atm_x, atm_y), (atm_x - 1, atm_y)

    elif symbol == "J":
        if prev_x < atm_x:
            return (atm_x, atm_y), (atm_x, atm_y - 1)
        else:
            return (atm_x, atm_y), (atm_x - 1, atm_y)

    elif symbol == "7":
        if prev_x > atm_x:
            return (atm_x, atm_y), (atm_x, atm_y - 1)
        else:
            return (atm_x, atm_y), (atm_x + 1, atm_y)

    elif symbol == "F":
        if prev_x > atm_x:
            return (atm_x, atm_y), (atm_x, atm_y + 1)
        else:
            return (atm_x, atm_y), (atm_x + 1, atm_y)


def get_start(map, starting_point):
    x, y = starting_point[0], starting_point[1]
    # if matching pipe exist north
    if x and map[x - 1][y] in ["|", "7", "F"]:
        return (x - 1, y)
    # if matching pipe exists east
    elif (y <= len(map[0]) - 1) and map[x][y + 1] in ["-", "J", "7"]:
        return (x, y + 1)
    # if matching pipe exists south
    elif (x <= len(map) - 1) and map[x + 1][y] in ["|", "L", "J"]:
        return (x + 1, y)
    # if matching pipe exists west
    elif y and map[x][y - 1] in ["-", "L", "F"]:
        return (x, y - 1)


def calculate_map(starting_point, map):
    start = starting_point
    end = get_start(map, starting_point)
    steps = 0
    pixels = [start, end]
    while True:
        values = next(map, start, end)
        start, end = values[0], values[1]
        steps += 1
        if map[end[0]][end[1]] == "S":
            break
        pixels.append(end)
    print("-------------------")
    print("steps : ", steps)
    farthest = int(steps / 2) + (steps % 2)
    print("farthest: ", farthest)

    not_boundary = [
        (x, y)
        for x, row in enumerate(map)
        for y in range(len(row))
        if (x, y) not in pixels
    ]

    from matplotlib import path

    p = path.Path(pixels)
    inside = 0
    for x_y in not_boundary:
        if p.contains_points([x_y]):
            inside += 1
    print("-------------------")
    print("inside boundary: ", inside)


def pipes():
    s, map = create_map()
    calculate_map(s, map)


if __name__ == "__main__":
    pipes()
