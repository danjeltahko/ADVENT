symbols = []

with open("day_3_input.txt", "r") as file:
    for line in file.readlines():
        for char in line:
            if char != ".":
                if char not in symbols:
                    symbols.append(char)

print(symbols)
