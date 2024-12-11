def get_map(txt: str) -> list:
    """split input & create 2d matrix"""
    with open(txt, "r") as file:
        i = 1
        map: list = []
        for line in file.readlines():
            row = []
            for planet in line.strip():
                if planet != "#":
                    row.append(planet)
                else:
                    row.append(str(i))
                    i += 1
            map.append(row)
        return map


def get_empty_rows(map: list) -> tuple:
    """return two lists containing the empty rows & columns"""
    empty_row: list = []
    empty_col: list = []
    # look at all the rows if they are empty
    for x, row in enumerate(map):
        if all(i == "." for i in row):
            empty_row.append(x)
    # create a map with columns as rows
    y_based_map = []
    for y in range(len(map)):
        column = []
        for row in map:
            column.append(row[y])
        y_based_map.append(column)
    # look at all the column-rows if they are empty
    for y, column in enumerate(y_based_map):
        if all(i == "." for i in column):
            empty_col.append(y)
    return empty_row, empty_col


def path_of_galaxies(txt: str, expand_universe: int):
    expand_universe -= 1
    map = get_map(txt)
    rows, cols = get_empty_rows(map)
    galaxies = [
        (map[i][j], (i, j))
        for i in range(len(map))
        for j in range(len(map[0]))
        if map[i][j] != "."
    ]
    total_distance = 0
    for index in range(len(galaxies)):
        for galaxy in galaxies[index + 1 :]:
            start = galaxies[index][1]
            desti = galaxy[1]
            steps = abs(start[0] - desti[0]) + abs(start[1] - desti[1])
            if start[0] <= desti[0]:
                x1 = start[0]
                x2 = desti[0]
            else:
                x1 = desti[0]
                x2 = start[0]

            if start[1] <= desti[1]:
                y1 = start[1]
                y2 = desti[1]
            else:
                y1 = desti[1]
                y2 = start[1]

            for i in range(x1, x2):
                if i in rows:
                    steps += expand_universe

            for j in range(y1, y2):
                if j in cols:
                    steps += expand_universe

            total_distance += steps

    print("total distance: ", total_distance)


if __name__ == "__main__":
    # path_of_galaxies("example1.txt", expand_universe=2)
    path_of_galaxies("day_11_input.txt", expand_universe=2)
    path_of_galaxies("day_11_input.txt", expand_universe=1000000)
