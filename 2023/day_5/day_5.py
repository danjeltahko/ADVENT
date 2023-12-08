map_keys = [
    "seeds",
    "seed-to-soil map:",
    "soil-to-fertilizer map:",
    "fertilizer-to-water map:",
    "water-to-light map:",
    "light-to-temperature map:",
    "temperature-to-humidity map:",
    "humidity-to-location map:",
]


def create_map1():
    map = {
        "seeds": [],
        "seed-to-soil map:": [],
        "soil-to-fertilizer map:": [],
        "fertilizer-to-water map:": [],
        "water-to-light map:": [],
        "light-to-temperature map:": [],
        "temperature-to-humidity map:": [],
        "humidity-to-location map:": [],
    }
    with open("day_5_input.txt", "r") as file:
        index = 0
        for line in file.readlines():
            line = line.strip()
            if not line:
                index += 1 if index != 7 else 0
                continue
            if not index:
                map[map_keys[index]] = line.split(":")[1].strip().split(" ")
            else:
                if line not in map_keys:
                    map[map_keys[index]].append(line.strip().split(" "))
    return map


def create_map2():
    map = {
        "seeds": [],
        "seed-to-soil map:": [],
        "soil-to-fertilizer map:": [],
        "fertilizer-to-water map:": [],
        "water-to-light map:": [],
        "light-to-temperature map:": [],
        "temperature-to-humidity map:": [],
        "humidity-to-location map:": [],
    }
    with open("day_5_input.txt", "r") as file:
        index = 0
        for line in file.readlines():
            line = line.strip()
            if not line:
                index += 1 if index != 7 else 0
                continue
            if not index:
                seeds = line.split(":")[1].strip().split(" ")
                seed_list = [
                    (int(seeds[i]), int(seeds[i]) + int(seeds[i + 1]))
                    for i in range(0, len(seeds), 2)
                ]
                map[map_keys[index]] = seed_list

            else:
                if line not in map_keys:
                    map[map_keys[index]].append(line.strip().split(" "))
    return map


def gardener1(map: dict):
    seeds = {}
    # Iterate through the seeds
    for seed in map["seeds"]:
        seeds[seed] = [int(seed)]
        # Iterate through all the categories
        for i, category in enumerate(map):
            if category == "seeds":
                continue
            range_found = 0
            # Iterate through all the destination lists
            for lists in map[category]:
                destin = range(int(lists[0]), int(lists[0]) + int(lists[2]))
                source = range(int(lists[1]), int(lists[1]) + int(lists[2]))

                if seeds[seed][i - 1] in source:
                    index = source.index(seeds[seed][i - 1])
                    range_found = destin[index]
                    seeds[seed].append(range_found)

            if not range_found:
                seeds[seed].append(seeds[seed][i - 1])

    lowest = None
    for i in seeds:
        if not lowest or seeds[i][-1] < lowest:
            lowest = seeds[i][-1]
    print("lowest:", lowest)


def convert(x, y, cat):
    """convert source to destination"""
    seed_range = y - x
    destin = range(cat[0], cat[0] + cat[2])
    source = range(cat[1], cat[1] + cat[2])
    x = destin[source.index(x)]
    y = x + seed_range
    return x, y


def refactor(root: list):
    cleaned: list = []
    root.sort()
    for r in root:
        if not cleaned:
            cleaned.append(r)

        else:
            if cleaned[-1][1] >= r[1]:
                continue
            elif cleaned[-1][1] >= r[0] and cleaned[-1][1] <= r[1]:
                new_range = (cleaned[-1][0], r[1])
                cleaned[-1] = new_range

            else:
                cleaned.append(r)
    cleaned.sort()
    return cleaned


def remove_already_used_spill_overs(spil_overs, already_checked):
    foo = []
    splitted = False
    for spill in spil_overs:
        exists = 0
        for checked in already_checked:
            if spill[0] < checked[1] and spill[1] > checked[0]:
                exists = 1
                if spill[0] < checked[0] and spill[1] > checked[1]:
                    foo.append((spill[0], checked[0] - 1))
                    foo.append((checked[1] + 1, spill[1]))
                    splitted = True
                elif spill[0] < checked[0]:
                    foo.append((spill[0], checked[0] - 1))
                    splitted = True

                elif spill[1] > checked[1]:
                    foo.append((checked[1] + 1, spill[1]))
                    splitted = True

        if not exists:
            foo.append(spill)
        exists = 0

    if splitted:
        foo = remove_already_used_spill_overs(foo, already_checked)
    return foo


def split_range(category_list, root, category):
    destination = []
    spil_overs = []
    already_checked = []
    range_found = 0
    for ranged_root in root:
        for ranged_cat in category_list:
            # to see if root/seed is in source span
            steps = ranged_cat[2]
            x = ranged_cat[1]
            y = ranged_cat[1] + steps
            if x < ranged_root[1] and y > ranged_root[0]:
                found_x = x if x > ranged_root[0] else ranged_root[0]
                found_y = y if y < ranged_root[1] else ranged_root[1]
                if ranged_root[0] < found_x and ranged_root[1] > found_y:
                    spil_overs.append((ranged_root[0], found_x - 1))
                    already_checked.append((found_x, found_y))
                    dest_x, dest_y = convert(found_x, found_y, ranged_cat)
                    destination.append((dest_x, dest_y))
                    spil_overs.append((found_y + 1, ranged_root[1]))
                    range_found = 1
                elif ranged_root[0] < found_x:
                    spil_overs.append((ranged_root[0], found_x - 1))
                    already_checked.append((found_x, found_y))
                    dest_x, dest_y = convert(found_x, found_y, ranged_cat)
                    destination.append((dest_x, dest_y))
                    range_found = 1

                elif ranged_root[1] > found_y:
                    dest_x, dest_y = convert(found_x, found_y, ranged_cat)
                    already_checked.append((found_x, found_y))
                    destination.append((dest_x, dest_y))
                    spil_overs.append((found_y + 1, ranged_root[1]))
                    range_found = 1

                else:
                    dest_x, dest_y = convert(found_x, found_y, ranged_cat)
                    already_checked.append((found_x, found_y))
                    destination.append((dest_x, dest_y))
                    range_found = 1

        if not range_found and category != "humidity-to-location map:":
            destination.append((ranged_root[0], ranged_root[1]))

        range_found = 0

    destination.sort()
    spil_overs = remove_already_used_spill_overs(spil_overs, already_checked)
    for spill in spil_overs:
        destination.append(spill)
    destination = refactor(destination)

    return destination


def gardener2(map: dict):
    root = map["seeds"]
    root.sort()

    # iterate through all the categories
    for category in map:
        if category == "seeds":
            continue
        # create a list with (destination, source, range) for each
        category_list: list = [
            (int(lists[0]), int(lists[1]), int(lists[2])) for lists in map[category]
        ]

        # Compare all the lists with source to get a new list
        category_list.sort()
        temp: list = split_range(category_list, root, category)
        temp.sort()
        root = temp

    print(f"lowest : {root[0][0]}")


def main():
    print("=== PART 1 ===")
    map = create_map1()
    gardener1(map)
    print("=== PART 2 ===")
    map = create_map2()
    gardener2(map)
    # correct answer : 56931769


if __name__ == "__main__":
    main()
