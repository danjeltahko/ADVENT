example = [
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
]


def strip_game(game: str):
    splitted = game.split(":")
    game_id = splitted[0].split(" ")[1]  # get last char = id
    game_sets = splitted[1].split(";")

    # print(game_id)
    red = 0
    green = 0
    blue = 0
    # games
    for game_set in game_sets:
        for cubes in game_set.split(","):
            cube = cubes.strip().split(" ")
            # print(cube)
            if "red" in cube[1] and red < int(cube[0]):
                red = int(cube[0])
            elif "green" in cube[1] and green < int(cube[0]):
                green = int(cube[0])
            elif "blue" in cube[1] and blue < int(cube[0]):
                blue = int(cube[0])
        # cubes = splitted[1].split(",")
    power = red * green * blue
    return {
        "game_id": game_id,
        "cubes": {"red": red, "green": green, "blue": blue},
        "total": power,
    }


def play_game():
    red = 12
    green = 13
    blue = 14
    game_ids = 0
    sum_of_power = 0
    with open("day_2_input.txt", "r") as file:
        for game in file.readlines():
            game = strip_game(game)
            # print(game)
            if (
                game["cubes"]["red"] <= red
                and game["cubes"]["green"] <= green
                and game["cubes"]["blue"] <= blue
            ):
                game_ids += int(game["game_id"])

            sum_of_power += game["total"]

        print(game_ids)
        print(f"sum of power : {sum_of_power}")


if __name__ == "__main__":
    play_game()
