from os import pipe
from os.path import split
import re


def get_spring_rows(txt: str) -> list:
    with open(txt, "r") as file:
        springs = [line.strip().split(" ") for line in file.readlines()]
    return springs


def get_broken_indices(spring, record):
    # for i in spring:
    #     print(i)
    # n_record = [1, 3, 1, 6]
    # n_sspring = "?#?#?#?#?#?#?#?"
    print("spring: ", spring)
    test = [s for s in spring.split(".") if s]
    test_hypo = [len(s) * "#" for s in spring.split(".") if s]
    broken_index = [b for b in range(len(spring)) if spring[b] == "?"]
    # record = [j for j in record.split(",")]
    print("test: ", test)
    print("hyph: ", test_hypo)
    print("reco: ", record)
    print("brok: ", broken_index)
    new_hypo = test_hypo.copy()
    # grupperna plussa på, += 1 för punkterna
    if test_hypo == record:
        print("SAME ONE")
        return []
    else:
        if len(test_hypo) == len(record):
            print("SAME LEN")
            for index in range(len(test_hypo)):
                if len(test_hypo[index]) == len(record[index]):
                    print("INDEX IS SAME", test_hypo[index])
                    test[index] = record[index]

    print("to search: ", test)
    indexes = "".join(test)
    new_broken_index = []
    i = 0
    for s in indexes:
        if s == "?":
            new_broken_index.append(broken_index[i])
            i += 1

    # print(new_broken_index)
    return new_broken_index
    # while True:
    #     if len(new_hyp) == len(record):
    #         for i in len(new_hyp):

    # print("not same length")
    # print(new_hyp, record)
    # for i, same in enumerate(record):
    #     # if len(same) != len(new_hyp[i]):
    #     ...

    # break
    # for i in test:
    #     ...
    # for s in spring.split("."):
    # print(s.strip())


def get_different_arrangements(spring, record):
    # print(spring, record)
    # arrangements = []
    # record = record.split(",")
    # goal = []
    # for i in record:
    #     goal.append(int(i) * "#")
    # print(record)
    # record = [int(i) * "#" for i in record.split(",")]
    # print("goal", goal)
    #
    # while True:
    #     print(spring, record)
    #     test = spring
    #
    #     break
    # print(spring, record)
    record = [int(i) * "#" for i in record.split(",")]
    broken_index = get_broken_indices(spring, record)
    if not broken_index:
        return 1
    # record = [int(i) * "#" for i in record.split(",")]
    # broken_index = [b for b in range(len(spring)) if spring[b] == "?"]
    # hypo_string will have all broken == #
    hypo_spring = ["#" if s == "?" else s for s in spring]
    # print("hypo: ", hypo_spring)
    # print("----------------------")
    # spring = "123.###"
    # generate(k=len(broken_index), b=broken_index, c=0, A=hypo_spring)
    A = [j for j in hypo_spring]
    arrangements = 0
    # replace every '#' that are broken to "."
    temp_array = A
    # print(record)
    print("index to search below: ", broken_index)
    print(A)
    print(record)
    for i in broken_index:
        # print(i, A)
        # temp_array = A
        # print(A)
        A[i] = "."
        # print(i, A)
        # print(record)
        # print("\n")
        # print(len(broken_index), A, record, arrangements)
        # print(broken_index)
        # print(i, A)

        modified = A.copy()
        # everything here looks good!!
        print("broken index: ", broken_index)
        print("modified: ", modified)
        print("record: ", record)
        print("arrangements: ", arrangements)
        print("==== index ", i, " ====")
        arrangements = test(broken_index, modified, record, arrangements)
        # print(i, arrangements)
        # print("atm arr: ", arrangements)
        # A = "".join(A)
        # print(A)
        # print(i, A)
        # A = temp_array
        # print("==========")
    # break
    print("found ", arrangements, " times")
    return arrangements


# EXAMPLE
# row = ???.### 1,1,3
# real = ["?", "?", "?", ".", "#", "#", "#"]
# BROKEN = [0, 1, 2] - length : 3
# hypo = ["#", "#", "#", ".", "#", "#", "#"]
# goal = ["#", "#", "###"] = 1, 1, 3


def test(k, a, goal, count):
    for i in range(len(k)):
        # print(i)
        # swap first broken index with last broken index
        # a[k[0]], a[k[-1]] = a[k[-1]], a[k[0]]
        cleaned = [foo for foo in "".join(a).split(".") if foo]
        # cleaned = "".join(a).split(".")
        # print("\n", cleaned, a, "\n")
        print(cleaned, goal)
        if cleaned == goal:
            # print("FOUND")
            count += 1
            # print("count.. ", count)
        a[k[i]], a[k[-1]] = a[k[-1]], a[k[i]]
        # print(a, i, k)
        # print(k)
        # print(a)

        # cleaned = [foo for foo in "".join(a).split(".") if foo]
        # if cleaned == goal:
        #     # print("FOUND")
        #     count += 1
        # print("count.. ", count)

    return count


# def test(k, a, goal, count):
#     if k == 1:
#         print("".join(a).split("."), goal)
#         if "".join(a).split(".") == goal:
#             print("FOUND")
#             return 1
#         return 0
#     for i in range(k):
#         # print("i: ", i, " | k: ", k)
#         count += test(k - 1, a, goal, count)
#         # print(count)
#         if k % 2 == 0:
#             a[i], a[k - 1] = a[k - 1], a[i]
#             # print("reverse even")
#         else:
#             a[0], a[k - 1] = a[k - 1], a[0]
#             # print("reverse odd")
#     return count
#


def heapPermutation(a, size):
    # if size becomes 1 then prints the obtained
    # permutation
    if size == 1:
        print(a)
        return

    for i in range(size):
        heapPermutation(a, size - 1)

        # if size is odd, swap 0th i.e (first)
        # and (size-1)th i.e (last) element
        # else If size is even, swap ith
        # and (size-1)th i.e (last) element
        if size & 1:
            a[0], a[size - 1] = a[size - 1], a[0]
        else:
            a[i], a[size - 1] = a[size - 1], a[i]


"""
procedure generate(k : integer, A : array of any):
    if k = 1 then
        output(A)
    else
        // Generate permutations with k-th unaltered
        // Initially k = length(A)
        generate(k - 1, A)

        // Generate permutations for k-th swapped with each k-1 initial
        for i := 0; i < k-1; i += 1 do
            // Swap choice dependent on parity of k (even or odd)
            if k is even then
                swap(A[i], A[k-1]) // zero-indexed, the k-th is at k-1
            else
                swap(A[0], A[k-1])
            end if
            generate(k - 1, A)
        end for
    end if

"""


def generate(k: int, b: list, c: int, A: list):
    # print(k, A.split(""))
    # print(A[0])
    print(k, b, c, A)
    if k == 1:
        return A
    else:
        # A = generate(k - 1, A)
        # ["?", "?", "?", ".", "#", "#", "#"]
        #
        # broken_index = [b for b in range(len(A)) if A[b] == "?"]
        for i in b:
            print(i)
            # print(i)
            A = [j for j in A]
            # broken_index = [b for b in range(len(A)) if A[b] == "?"]
            print(A)
            # print(broken_index)
            # A[i] = "."
            if k % 2 == 0:
                # A[i], A[b[-1]] = A[b[-1]], A[i]
                A[i], A[b[-1]] = A[b[-1]], "."
            else:
                A[0], A[b[-1]] = A[b[-1]], "."
                # A[0], A[b[-1]] = A[b[-1]], A[0]
            A = "".join(A)
            print(A)
            print("==========")
            # generate(k - 1, A)


"""
procedure generate(k : integer, A : array of any):
    if k = 1 then
        output(A)
    else
        // Generate permutations with k-th unaltered
        // Initially k = length(A)
        generate(k - 1, A)

        // Generate permutations for k-th swapped with each k-1 initial
        for i := 0; i < k-1; i += 1 do
            // Swap choice dependent on parity of k (even or odd)
            if k is even then
                swap(A[i], A[k-1]) // zero-indexed, the k-th is at k-1
            else
                swap(A[0], A[k-1])
            end if
            generate(k - 1, A)
        end for
    end if

"""


def broken_springs(springs: list):
    # print(springs)
    p = 0
    for row in springs:
        spring, broken = row
        print(spring, broken)
        # cleaned = [i for i in spring.split(".") if len(i) > 0]
        # found = get_different_arrangements(spring, broken)
        # print("found: ", found)
        print("---------------------------")
        # arr = get_different_arrangements(cleaned, broken)
        # print("---------------------------")
        # print(arr, row)
        # break
        p += 1
        if p == 2:
            break


if __name__ == "__main__":
    springs = get_spring_rows("example.txt")
    broken_springs(springs)
