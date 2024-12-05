#!/usr/bin/python3

from collections import Counter

if __name__ == "__main__":
    # File parsing
    left_list: list[int] = []
    right_list: list[int] = []
    with open("input.txt", "r") as f:
        for line in f:
            left_number, right_number = line.split()
            left_list.append(int(left_number))
            right_list.append(int(right_number))

    # --- Challenge part 1 ---

    left_list.sort()
    right_list.sort()

    diff_sum = sum(
        [
            abs(left_number - right_number)
            for left_number, right_number in zip(left_list, right_list)
        ]
    )

    print(f"Diff sum: {diff_sum}")

    # --- Challenge part 2 ---

    right_number_count = Counter(right_list)

    sim_score = sum(
        [
            left_number * right_number_count.get(left_number, 0)
            for left_number in left_list
        ]
    )

    print(f"Similarity score: {sim_score}")
