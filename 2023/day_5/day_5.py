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
                    # print(line)
                    map[map_keys[index]].append(line.strip().split(" "))
    print("Created Map")
    return map


def gardener1(map: dict):
    seeds = {}
    # Iterate through the seeds
    for seed in map["seeds"]:
        # print(seed)
        seeds[seed] = [int(seed)]
        # Iterate through all the categories
        for i, category in enumerate(map):
            if category == "seeds":
                continue
            # print(category, map[category])
            range_found = 0
            # Iterate through all the destination lists
            for lists in map[category]:
                # print(lists)
                destin = range(int(lists[0]), int(lists[0]) + int(lists[2]))
                source = range(int(lists[1]), int(lists[1]) + int(lists[2]))

                if seeds[seed][i - 1] in source:
                    index = source.index(seeds[seed][i - 1])
                    # print(destin[index])
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
    seed_range = y - x
    destin = range(cat[0], cat[0] + cat[2])
    source = range(cat[1], cat[1] + cat[2])
    x = destin[source.index(x)]
    y = x + seed_range
    return x, y


def refactor(root, spill_overs):
    cleaned: list = []
    for spill in spill_overs:
        root.append(spill)
    root.sort()
    print(f"before clean : {root}")
    # print(root)
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
    print(f"after clean : {cleaned}")
    return cleaned


def split_range(temp_list, root, category):
    print("===== REFACTOR ====")
    destination = []
    spil_overs = []
    range_found = 0
    # print(f"root = {root}")
    for ranged_root in root:
        for ranged_cat in temp_list:
            # to see if root/seed is in source span
            steps = ranged_cat[2]
            x = ranged_cat[1]
            y = ranged_cat[1] + steps
            # print(f"found ({ranged_root[0]}, {ranged_root[1]}) in ({x}, {y})")
            if x < ranged_root[1] and y > ranged_root[0]:
                found_x = x if x > ranged_root[0] else ranged_root[0]
                found_y = y if y < ranged_root[1] else ranged_root[1]
                print(f"found ({ranged_root[0]}, {ranged_root[1]}) in ({x}, {y})")
                print(f"matching x,y - ({found_x}, {found_y})")

                # if [range of seeds] x is less than source x
                # and y is greater that source y, then
                # split the range to
                if ranged_root[0] < found_x and ranged_root[1] > found_y:
                    # print(f"found x, y = {found_x},{found_y}")
                    spil_overs.append((ranged_root[0], found_x - 1))
                    dest_x, dest_y = convert(found_x, found_y, ranged_cat)
                    print(f"became : ({dest_x},{dest_y})")
                    destination.append((dest_x, dest_y))
                    spil_overs.append((found_y + 1, ranged_root[1]))
                    range_found = 1
                    # print(f"range found in x & y = {range_found}")
                elif ranged_root[0] < found_x:
                    spil_overs.append((ranged_root[0], found_x - 1))
                    dest_x, dest_y = convert(found_x, found_y, ranged_cat)
                    print(f"became : ({dest_x},{dest_y})")
                    destination.append((dest_x, dest_y))
                    range_found = 1
                    # print(f"range found in x = {range_found}")

                elif ranged_root[1] > found_y:
                    dest_x, dest_y = convert(found_x, found_y, ranged_cat)
                    destination.append((dest_x, dest_y))
                    print(f"became : ({dest_x},{dest_y})")
                    spil_overs.append((found_y + 1, ranged_root[1]))
                    range_found = 1

                else:
                    dest_x, dest_y = convert(found_x, found_y, ranged_cat)
                    print(f"became : ({dest_x},{dest_y})")
                    destination.append((dest_x, dest_y))
                    range_found = 1

        if not range_found and category != "humidity-to-location map:":
            # print(f"not found : {(ranged_root[0], ranged_root[1])}")
            destination.append((ranged_root[0], ranged_root[1]))

        range_found = 0
    # print(category)
    # if category == "humidity-to-location map:":
    #     print(destination)
    #     return destination

    destination.sort()
    # destination = refactor(destination, spil_overs)

    print("\n")
    # print(destination)

    return destination


def gardener2(map: dict):
    root = map["seeds"]
    root.sort()

    print(f"seeds : {root}")
    for i, category in enumerate(map):
        if category == "seeds":
            continue

        category_list: list = [
            (int(lists[0]), int(lists[1]), int(lists[2])) for lists in map[category]
        ]

        category_list.sort()
        # for j in category_list:
        #     print(j)
        # root.map = split_range(category_list, root.map)
        print("\n")
        # print(category_list)
        print(category)
        temp: list = split_range(category_list, root, category)
        temp.sort()
        root = temp
        # print(f"{category} : {root}")
        # for r in root:
        #     print(r)
        # root = refactor(root)
        # if i == 1:
        #     break

    return root


def main():
    # I just tried running this *without* adding the unmatching source
    # as the source number, So I only add the matching numbers..
    print("=== PART 1 ===")
    map1 = create_map1()
    gardener1(map1)
    print("=== PART 2 ===")
    map2 = create_map2()
    root = gardener2(map2)
    print(root[0][0])
    # not right : 167312469
    # not right : 0
    # correct answer : 56931769


if __name__ == "__main__":
    main()
