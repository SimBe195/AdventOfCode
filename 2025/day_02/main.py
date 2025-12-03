import functools


def parse_ranges(range_str: str) -> list[tuple[int, int]]:
    ranges = []
    for range_str in range_str.split(","):
        start_str, end_str = range_str.split("-")
        ranges.append((int(start_str), int(end_str)))

    return ranges


def split_ranges(ranges: list[tuple[int, int]]) -> list[tuple[int, int]]:
    new_ranges = []
    for start, end in ranges:
        min_digits = len(str(start))
        max_digits = len(str(end))
        curr = start
        for num_digits in range(min_digits, max_digits):
            new_ranges.append((curr, int("9" * num_digits)))
            curr = int("1" + "0" * num_digits)
        new_ranges.append((curr, end))

    return new_ranges


@functools.lru_cache
def get_factors(num: int) -> list[int]:
    factors = []
    for i in range(1, num):
        if num % i == 0:
            factors.append(i)

    return factors


def get_divisor(num_digits: int) -> int | None:
    if num_digits % 2 == 1:
        return None
    return int("1" + "0" * (num_digits // 2 - 1) + "1")


def get_divisors(num_digits: int) -> list[int]:
    divisors = []

    factors = get_factors(num_digits)
    for sub_len in factors:
        repeats = num_digits // sub_len
        divisors.append(int(("0" * (sub_len - 1) + "1") * repeats))

    return divisors


def min_invalid_id_after(id: int, divisor: int) -> int | None:
    return (-(-id // divisor)) * divisor


def max_invalid_id_before(id: int, divisor: int) -> int | None:
    return (id // divisor) * divisor


def get_invalid_ids_in_range(start: int, end: int) -> list[int]:
    num_digits = len(str(start))
    divisor = get_divisor(num_digits)
    if divisor is None:
        return []
    min_id = min_invalid_id_after(start, divisor)
    max_id = max_invalid_id_before(end, divisor)
    if min_id is None or max_id is None:
        return []
    return list(range(min_id, max_id + 1, divisor))


def get_invalid_ids_in_range_2(start: int, end: int) -> list[int]:
    num_digits = len(str(start))
    divisors = get_divisors(num_digits)
    invalid_ids = []
    for divisor in divisors:
        min_id = min_invalid_id_after(start, divisor)
        max_id = max_invalid_id_before(end, divisor)
        if min_id is None or max_id is None:
            return []
        invalid_ids_for_divisor = list(range(min_id, max_id + 1, divisor))
        invalid_ids.extend(invalid_ids_for_divisor)
    return list(set(invalid_ids))


def task_1(ranges: list[tuple[int, int]]) -> int:
    invalid_id_sum = 0
    for start, end in ranges:
        invalid_ids = get_invalid_ids_in_range(start, end)
        invalid_id_sum += sum(invalid_ids)

    return invalid_id_sum


def task_2(ranges: list[tuple[int, int]]) -> int:
    invalid_id_sum = 0
    for start, end in ranges:
        invalid_ids = get_invalid_ids_in_range_2(start, end)
        invalid_id_sum += sum(invalid_ids)

    return invalid_id_sum


def main() -> None:
    with open("input.txt") as f:
        ranges = split_ranges(parse_ranges(f.read()))
    print(f"Task 1: {task_1(ranges)}")
    print(f"Task 2: {task_2(ranges)}")


if __name__ == "__main__":
    main()
