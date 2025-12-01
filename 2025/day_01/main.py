def part_1() -> int:
    zero_reached_count = 0
    current = 50
    with open("input.txt") as file:
        for line in file:
            if line.startswith("L"):
                sign = -1
            else:
                sign = 1
            amount = int(line[1:])
            current = (current + sign * amount) % 100
            zero_reached_count += current == 0

    return zero_reached_count


def times_zero_is_crossed(start: int, end: int) -> int:
    counter = 0

    if end > start:
        while end > 99:
            end -= 100
            counter += 1
    elif end < start:
        if start == 0:
            counter = -1
        while end < 0:
            end += 100
            counter += 1
        if end == 0:
            counter += 1

    return counter


def part_2() -> int:
    zero_reached_count = 0
    current = 50
    with open("input.txt") as file:
        for line in file:
            if line.startswith("L"):
                sign = -1
            elif line.startswith("R"):
                sign = 1
            else:
                continue
            amount = int(line[1:])

            next = current + sign * amount
            zero_reached_count += times_zero_is_crossed(current, next)
            current = next % 100

    return zero_reached_count


def main() -> None:
    print(f"Part 1: {part_1()}")
    print(f"Part 2: {part_2()}")


if __name__ == "__main__":
    main()
