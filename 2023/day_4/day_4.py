def scratchcards():
    with open("day_4_input.txt", "r") as file:
        # should i maybe learn regex.....
        splitted = [
            [
                line.strip().split(":")[0].strip().split(" ")[-1],
                [
                    winning.strip()
                    for winning in line.strip()
                    .split(":")[1]
                    .strip()
                    .split("|")[0]
                    .strip()
                    .split(" ")
                    if winning.isdigit()
                ],
                [
                    scratched.strip()
                    for scratched in line.strip()
                    .split(":")[1]
                    .strip()
                    .split("|")[1]
                    .strip()
                    .split(" ")
                    if scratched.isdigit()
                ],
            ]
            for line in file.readlines()
        ]
        sum = 0
        instances = {}
        for card in splitted:
            points = 0
            total_wins = 0
            for winning in card[1]:
                if winning in card[2]:
                    # add all the points and also wins
                    points = points * 2 if points else 1
                    total_wins += 1
            for i in range(0, total_wins + 1):
                index = int(card[0]) + i
                try:
                    if index == int(card[0]):
                        instances[str(index)] += 1
                    else:
                        try:
                            instances[str(index)] += instances[card[0]]
                        except Exception:
                            instances[str(index)] = instances[card[0]]
                except Exception:
                    instances[str(index)] = 1
            sum += points

        print("------")
        total_scratched = 0
        for value in instances.values():
            total_scratched += value
        print(sum, total_scratched)


if __name__ == "__main__":
    scratchcards()
