def get_navigation(txt: str) -> tuple[str, dict]:
    instructions: str = ""
    navigation: dict = {}
    starting_points = []
    with open(txt, "r") as file:
        for i, line in enumerate(file.readlines()):
            if not i:
                instructions = line.strip()
            elif line.strip():
                # should really learn regex....
                node = line.strip().split("=")[0].strip()
                left = line.strip().split("(")[1].split(",")[0].strip()
                right = line.strip().split(",")[1].strip().split(")")[0]
                if node[-1] == "A":
                    starting_points.append(node)
                navigation[node] = [left, right]

        return instructions, navigation, starting_points


def get_steps_1(instructions: str, navigation: dict) -> int:
    steps = 0
    index = 0
    node = "AAA"
    while True:
        if index >= len(instructions):
            index = 0

        if node == "ZZZ":
            break

        direction = instructions[index]
        if direction == "R":
            node = navigation[node][1]
            steps += 1
            index += 1

        if direction == "L":
            node = navigation[node][0]
            steps += 1
            index += 1

    return steps


def get_steps_2(instructions: str, navigation: dict, starting: list) -> int:
    """every last Z letter is coefficient for every starting point"""
    steps = 0
    index = 0
    coefficients = []
    for start in starting:
        node = start
        # print(node)
        while True:
            # print(node, navigation[node])
            if index >= len(instructions):
                index = 0

            direction = instructions[index]
            if direction == "R":
                node = navigation[node][1]
                steps += 1
                index += 1

            if direction == "L":
                node = navigation[node][0]
                steps += 1
                index += 1

            if node[-1] == "Z":
                coefficients.append((start, steps))
                steps = 0
                break

    multiplier = 2
    lcms = []
    total = 1
    for coefficient in coefficients:
        number = coefficient[1]
        lcm = []
        while True:
            m = number % multiplier
            if not m:
                lcm.append(multiplier)
                number = number / multiplier
                multiplier = 2
                if number == 1:
                    break
            else:
                multiplier += 1
        for i in lcm:
            if i not in lcms:
                lcms.append(i)
                total *= i
            else:
                if lcm.count(i) > lcms.count(i):
                    lcms.append(i)
                    total *= i

    return total


if __name__ == "__main__":
    i, n, s = get_navigation("day_8_input.txt")
    steps = get_steps_1(i, n)
    print("part 1:", steps)
    steps = get_steps_2(i, n, s)
    print("part 2:", steps)
