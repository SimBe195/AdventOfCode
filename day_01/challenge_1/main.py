if __name__ == "__main__":
    left_list = []
    right_list = []
    with open("inputs.txt", "r") as f:
        for line in f:
            left_number, right_number = line.split()
            left_list.append(int(left_number))
            right_list.append(int(right_number))

    left_list.sort()
    right_list.sort()

    diff_sum = sum(
        [
            abs(left_number - right_number)
            for left_number, right_number in zip(left_list, right_list)
        ]
    )

    print(f"Diff sum: {diff_sum}")
