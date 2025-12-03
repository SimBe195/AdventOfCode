def get_largest_digit(digit_str: str) -> tuple[int, int]:
    max_digit = 0
    max_digit_pos = -1
    for i in range(len(digit_str)):
        curr_digit = int(digit_str[i])
        if curr_digit > max_digit:
            max_digit = curr_digit
            max_digit_pos = i
    return max_digit, max_digit_pos


def get_joltage(bank: str, num_batteries: int) -> int:
    joltage = 0
    current_left = 0
    for digit_index in range(num_batteries):
        right = -num_batteries + digit_index + 1
        if right == 0:
            right = None
        max_digit, max_digit_pos = get_largest_digit(bank[current_left:right])
        current_left += max_digit_pos + 1
        joltage += 10 ** (num_batteries - digit_index - 1) * max_digit
    return joltage


def task_1(banks: list[str]) -> int:
    return sum(get_joltage(bank, 2) for bank in banks)


def task_2(banks: list[str]) -> int:
    return sum(get_joltage(bank, 12) for bank in banks)


def main():
    with open("input.txt") as f:
        banks = f.read().splitlines()

    print(f"Task 1: {task_1(banks)}")
    print(f"Task 2: {task_2(banks)}")


if __name__ == "__main__":
    main()
