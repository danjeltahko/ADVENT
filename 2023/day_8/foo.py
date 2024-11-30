numbers = [11567, 21251, 12643, 16409, 19099, 14257]

multiplier = 2
bras = []
for num in numbers:
    number = num
    brap = []
    while True:
        m = number % multiplier
        # print(m, number, multiplier)
        if not m:
            # print(multiplier)
            brap.append(multiplier)
            number = number / multiplier
            # print("number:", number)
            multiplier = 2
            if number == 1:
                break
        else:
            multiplier += 1
    bras.append(brap)
# [[43, 269], [79, 269], [47, 269], [61, 269], [71, 269], [53, 269]]
# right answer = 9858474970153
print(bras)
