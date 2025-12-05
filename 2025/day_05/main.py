def parse_ranges(ranges_str: str) -> list[tuple[int, int]]:
    return [
        tuple(map(int, line.split("-")))
        for line in ranges_str.split("\n")
        if len(line) > 0
    ]  # type: ignore


def parse_ingredients(ingredients_str: str) -> list[int]:
    return list(map(int, ingredients_str.split("\n")))


def merge_ranges(ranges: list[tuple[int, int]]) -> tuple[list[tuple[int, int]], bool]:
    merged_ranges = []
    merged_any = False
    for start, end in ranges:
        merged = False
        for i in range(len(merged_ranges)):
            start2, end2 = merged_ranges[i]
            if (
                (start <= start2 <= end)
                or (start <= end2 <= end)
                or (start2 <= start <= end2)
                or (start2 <= end <= end2)
            ):
                merged_ranges[i] = (min(start, start2), max(end, end2))
                merged = True
                merged_any = True
                break

        if not merged:
            merged_ranges.append((start, end))
    return merged_ranges, merged_any


def merge_ranges_recursive(ranges: list[tuple[int, int]]) -> list[tuple[int, int]]:
    merged_ranges, merged_any = merge_ranges(ranges)
    if merged_any:
        merged_ranges = merge_ranges_recursive(merged_ranges)
    return merged_ranges


def task_1(ranges: list[tuple[int, int]], ingredients: list[int]) -> int:
    counter = 0
    for ingredient in ingredients:
        for start, end in ranges:
            if start <= ingredient <= end:
                counter += 1
                break
    return counter


def task_2(ranges: list[tuple[int, int]]) -> int:
    merged_ranges = merge_ranges_recursive(ranges)

    total_size = 0
    for start, end in merged_ranges:
        total_size += end - start + 1
    return total_size


def main() -> None:
    with open("input.txt") as f:
        ranges_str, ingredients_str = f.read().strip().split("\n\n")
    ranges = parse_ranges(ranges_str)
    ingredients = parse_ingredients(ingredients_str)

    print(f"Task 1: {task_1(ranges, ingredients)}")
    print(f"Task 2: {task_2(ranges)}")


if __name__ == "__main__":
    main()
