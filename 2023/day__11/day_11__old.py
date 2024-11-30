def create_map(txt: str) -> list:
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
    empty_row = []
    empty_col = []
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


def expand_map(rows: list, cols: list, map: list) -> tuple[list, list]:
    """adds the empty rows and columns"""
    expanded_map = []
    galaxies = []
    # iterate through all the rows
    for x, row in enumerate(map):
        expanded_row = []
        # iterate through all the columns
        for y, col in enumerate(row):
            expanded_row.append(col)
            # if this column is in empty list
            # then add one additional empty symbol
            if y in cols:
                expanded_row.append(".")
            # if col != ".":
            #     galaxies.append((col, (x, y)))
        # after iterating all columns append the new row
        expanded_map.append(expanded_row)
        # if this row is empty, add an additional empty row
        if x in rows:
            expanded_map.append(expanded_row)

    galaxies = []
    for i in range(len(expanded_map)):
        for j in range(len(expanded_map[0])):
            if expanded_map[i][j] != ".":
                galaxies.append((expanded_map[i][j], (i, j)))

    return expanded_map, galaxies


def get_map(txt: str) -> tuple[list, list]:
    map = create_map(txt)
    rows, columns = get_empty_rows(map)
    map, galaxies = expand_map(rows, columns, map)
    return map, galaxies


def calculate_pairs(galaxies: list) -> list:
    pairs: list = []
    print(len(galaxies))
    # count each pair once
    for i in range(len(galaxies)):
        for j in range(1, len(galaxies)):
            if ((galaxies[i], galaxies[j])) in pairs:
                continue
            if ((galaxies[j], galaxies[i])) in pairs:
                continue
            if galaxies[i] == galaxies[j]:
                continue
            else:
                pairs.append((galaxies[i], galaxies[j]))
                print(galaxies[i], galaxies[j])
    return pairs


def get_next_nodes(pos, goal):
    # if goal is in right botom corner
    if goal[0] > pos[0] and goal[1] > pos[1]:
        return [(pos[0], pos[1] + 1), (pos[0] + 1, pos[1])]
    # if goal is in left botom corner
    elif goal[0] > pos[0] and goal[1] < pos[1]:
        return [(pos[0] + 1, pos[1]), (pos[0], pos[1] - 1)]
    # if goal is in right top corner
    elif goal[0] < pos[0] and goal[1] > pos[1]:
        return [(pos[0], pos[1] + 1), (pos[0] - 1, pos[1])]
    # if goal is in left top corner
    elif goal[0] < pos[0] and goal[1] < pos[1]:
        return [(pos[0] - 1, pos[1]), (pos[0], pos[1] - 1)]
    # if goal is straight east
    elif goal[1] > pos[1]:
        return [(pos[0], pos[1] + 1)]
    # if goal is straight west
    elif goal[1] < pos[1]:
        return [(pos[0], pos[1] - 1)]
    # if goal is straight south
    elif goal[0] > pos[0]:
        return [(pos[0] + 1, pos[1])]
    # if goal is straight north
    elif goal[0] < pos[0]:
        return [(pos[0] - 1, pos[1])]


def calculate_distance(pair: tuple):
    from collections import deque

    end = pair[1][1]
    steps = 0
    search_queue: list[tuple] = deque()
    search_queue += [[(pair[0][1][0], pair[0][1][1])]]
    checked = []
    found = False
    while not found:
        next_nodes = []
        positions = search_queue.popleft()
        # print("positions : ", positions)
        for position in positions:
            if position in checked:
                continue
            if position == end:
                # print("found in ", steps, " steps")
                found = True
                break
            checked.append(position)

            next_nodes += get_next_nodes(position, end)
            # print(next_nodes)
        if not found:
            search_queue += [next_nodes]
            steps += 1
    return steps


def cosmic_expansion(map: list, galaxies: list):
    # print(galaxies)
    for i in map:
        print(i)
    print("-------------------")
    print("Calculate Pairs")
    pairs = calculate_pairs(galaxies)
    print("pairs: ", len(pairs))
    print("-------------------")
    total = 0
    for pair in pairs:
        steps = calculate_distance(pair)
        print(pair, " - steps : ", steps)
        total += steps
    print("-------------------")
    print("total steps: ", total)


if __name__ == "__main__":
    map, galaxies = get_map("example1.txt")
    # map, galaxies = get_map("example1.txt")
    cosmic_expansion(map, galaxies)
