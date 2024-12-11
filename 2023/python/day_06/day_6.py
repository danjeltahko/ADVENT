def get_first_race(txt: str) -> list:
    with open(txt, "r") as file:
        races = []
        for i in file.readlines():
            racing = []
            line = i.strip().split(":")[1]
            for number in line.strip().split(" "):
                if number.isdigit():
                    racing.append(int(number))
            races.append(racing)
        return races


def get_second_race(txt: str) -> list:
    with open(txt, "r") as file:
        races = []
        for i in file.readlines():
            line = i.strip().split(":")[1]
            num = ""
            for number in line.strip().split(" "):
                if number.isdigit():
                    num += number
            races.append(int(num))
        return races


def calculate_first_race():
    races = get_first_race("day_6_input.txt")
    winnings = []
    for i in range(0, len(races[0])):
        milliseconds = races[0][i]
        record = races[1][i]
        race_win = []
        for seconds in range(milliseconds + 1):
            race = (milliseconds - seconds) * seconds
            if race > record:
                race_win.append(seconds)
        winnings.append(race_win)

    total_wins = 1
    for wins in winnings:
        total_wins *= len(wins)

    print("total wins first: ", total_wins)


def calculate_second_race():
    races = get_second_race("day_6_input.txt")
    winnings = []
    print(races)
    # print(races[0][i], races[1][i])
    milliseconds = races[0]
    record = races[1]
    race_win = []
    for seconds in range(milliseconds + 1):
        race = (milliseconds - seconds) * seconds
        if race > record:
            race_win.append(seconds)
    winnings.append(race_win)

    total_wins = 1
    for wins in winnings:
        total_wins *= len(wins)

    print(total_wins)


if __name__ == "__main__":
    calculate_first_race()
    calculate_second_race()
