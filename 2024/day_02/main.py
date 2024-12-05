def is_safe(report: list[int]) -> bool:
    diffs = [num_2 - num_1 for num_1, num_2 in zip(report[:-1], report[1:])]

    if any(abs(diff) > 3 for diff in diffs):
        return False
    elif all(diff > 0 for diff in diffs):
        return True
    elif all(diff < 0 for diff in diffs):
        return True
    else:
        return False


if __name__ == "__main__":
    safe_count = 0
    unsafe_count = 0

    dampened_safe_count = 0
    dampened_unsafe_count = 0

    with open("input.txt", "r") as f:
        for line in f:
            report = list(map(int, line.split()))
            if not report:
                continue

            # --- Challenge 1 ---

            if is_safe(report):
                safe_count += 1
            else:
                unsafe_count += 1

            # --- Challenge 2 ---

            if is_safe(report) or any(
                is_safe(report[:idx] + report[idx + 1 :]) for idx in range(len(report))
            ):
                dampened_safe_count += 1
            else:
                dampened_unsafe_count += 1

    print(f"Safe reports: {safe_count}. Unsafe reports: {unsafe_count}.")
    print(
        f"Safe reports with problem dampener: {dampened_safe_count}. Unsafe reports with problem dampener: {dampened_unsafe_count}."
    )
