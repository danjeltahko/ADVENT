def get_report(txt: str):
    report = []
    with open(txt, "r") as file:
        for line in file.readlines():
            report.append(line.strip().split(" "))
    return report


def extrapolate(sequences: list) -> int:
    diff = 0
    for i in range(1, len(sequences) + 1):
        if i == 1:
            continue
        else:
            new_num = int(sequences[-i][-1]) + diff
            diff = new_num
    return diff


def oasis1(report: list):
    next_value = 0
    for history in report:
        sequences: list = [history]
        root = history
        while True:
            sequence = [
                int(root[i + 1]) - int(j)
                for i, j in enumerate(root)
                if i < len(root) - 1
            ]
            root = sequence
            sequences.append(root)
            if all(i == 0 for i in root):
                break

        next_value += extrapolate(sequences)
    print("total: ", next_value)


def oasis2(report: list):
    next_value = 0
    for history in report:
        sequences: list = [history]
        root = history
        while True:
            sequence = [
                int(root[i + 1]) - int(j)
                for i, j in enumerate(root)
                if i < len(root) - 1
            ]
            root = sequence
            sequences.append(root)
            if all(i == 0 for i in root):
                break
        diff = 0
        for i in range(1, len(sequences) + 1):
            if i == 1:
                continue
            else:
                new_num = int(sequences[-i][0]) - diff
                # print(f"{sequences[-i]} - {new_num}")
                diff = new_num
        # print("--------------------")
        next_value += diff
    print("total: ", next_value)


if __name__ == "__main__":
    # report = get_report("example1.txt")
    report = get_report("day_9_input.txt")
    oasis1(report)
    oasis2(report)
