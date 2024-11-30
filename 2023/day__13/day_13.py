import numpy


def get_map(txt: str):
    map = []
    with open(txt, "r") as file:
        paths = []
        for line in file.readlines():
            if not line.strip():
                map.append(paths)
                paths = []
            else:
                paths.append([char for char in line.strip()])
        map.append(paths)
    return map


def get_steps(reflect, match, pattern):
    steps = 0
    start, end = match
    while True:
        if reflect == "vertical":
            try:
                if list(pattern[:, start - steps]) == list(pattern[:, end + steps]):
                    steps += 1
                else:
                    break
            except IndexError:
                break
        else:
            try:
                if list(pattern[start - steps]) == list(pattern[end + steps]):
                    steps += 1
                else:
                    break
            except IndexError:
                break
    print("found ", reflect, steps)
    return steps


def vertical(pattern):
    length = len(pattern[0])
    step = None
    for i in range(length - 1):
        if list(pattern[:, i]) == list(pattern[:, i + 1]):
            # if i != (length - 2) and list(pattern[:, i - 1]) == list(pattern[:, i + 2]):
            #     return i + 1

            if i >= int(length / 2):
                steps = (length - 1) - i
                begin = i - (steps - 1)
                # print("Vertical")
                # print(
                #     "vertical (i, steps, begin, length): ",
                #     i,
                #     steps,
                #     begin,
                #     length,
                # )
                # print("vertical")
                step = get_steps(reflect="hori", match=(i, i + 1), pattern=pattern)
                print("step: ", step)
                # if list(pattern[:, begin]) == list(pattern[:, -1]):
                #     print(
                #         "vertical (i, steps, begin, length): ",
                #         i,
                #         steps,
                #         begin,
                #         length,
                #     )
                #     # return i + 1
                #     if not found:
                #         # found = (i + 1, steps)
                #         found = steps + 1
                #     elif steps > found:
                #         # found = (i + 1, steps)
                #         found = steps + 1
                # else:
                #     return None
            else:
                steps = i + 1
                end = i + steps
                # print("vertical")
                # print(
                #     "vertical (i, steps, end, length): ",
                #     i,
                #     steps,
                #     end,
                #     length,
                # )
                step = get_steps(reflect="hori", match=(i, i + 1), pattern=pattern)
                print("step: ", step)
                # if list(pattern[:, 0]) == list(pattern[:, end]):
                #     print(
                #         "vertical (i, steps, end, length): ",
                #         i,
                #         steps,
                #         end,
                #         length,
                #     )
                #     # return i + 1
                #     if not found:
                #         # found = (i + 1, steps)
                #         found = steps + 1
                #     elif steps > found:
                #         # found = (i + 1, steps)
                #         found = steps + 1
                # else:
                #     return None
    if step:
        print("found: ", step)
        return step
    else:
        return None


def horizontal(pattern):
    length = len(list(pattern[:, 0]))
    step = None
    for i in range(length - 1):
        if list(pattern[i]) == list(pattern[i + 1]):
            # if i != (length - 2) and list(pattern[i - 1]) == list(pattern[i + 2]):
            #     return i + 1
            if i >= int(length / 2):
                steps = (length - 1) - i
                begin = i - (steps - 1)
                step = get_steps(reflect="hori", match=(i, i + 1), pattern=pattern)
                print("step: ", step)
                # if list(pattern[begin]) == list(pattern[-1]):
                #     print(
                #         "horizontal (i, steps, begin, length): ",
                #         i,
                #         steps,
                #         begin,
                #         length,
                #     )
                #     # return i + 1
                #     if not found:
                #         # found = (i + 1, steps)
                #         found = steps + 1
                #     elif steps > found:
                #         # found = (i + 1, steps)
                #         found = steps + 1
                # else:
                #     return None
            else:
                steps = i + 1
                end = i + steps
                step = get_steps(reflect="hori", match=(i, i + 1), pattern=pattern)
                print("step: ", step)
                # if list(pattern[0]) == list(pattern[end]):
                #     print(
                #         "horizontal (i, steps, end, length): ",
                #         i,
                #         steps,
                #         end,
                #         length,
                #     )
                #     # return i + 1
                #     if not found:
                #         # found = (i + 1, steps)
                #         found = steps + 1
                #     elif steps > found:
                #         # found = (i + 1, steps)
                #         found = steps + 1
                # else:
                #     return None
    if step:
        print("found: ", step)
        return step
    else:
        return None


def mirrors():
    # map = get_map("example.txt")
    map = get_map("day_13_input.txt")
    rows = 0
    cols = 0
    summarize = 0
    for patterns in map:
        pattern = numpy.array(patterns)
        print(pattern)
        # print("\n")
        vert = vertical(pattern)
        if vert:
            rows += vert
            # print("is vertical, ", vert, " cols to left")
        else:
            # print("not vertical")
            hor = horizontal(pattern)
            # print("is horizontal, ", hor, " rows up")
            if not hor:
                raise ValueError
            else:
                cols += hor
        print("\n")

    summarize = rows + (cols * 100)

    print("summarize: ", summarize)
    # print("len ", len(pattern[0]) - 1)
    # print(pattern[:, 2])
    # print(pattern)
    # print(pattern)
    # print("\n")
    # a, b += test()
    # break

    # print(a, b)


if __name__ == "__main__":
    mirrors()
    # not 30589
    # not 33219
    # not 15975
    # not 20827
