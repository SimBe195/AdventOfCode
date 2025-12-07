import copy


def parse_grid(input_str: str) -> list[list[str]]:
    return [list(line) for line in input_str.splitlines()]


def task_1(grid: list[list[str]]) -> int:
    grid = copy.deepcopy(grid)
    n_rows = len(grid)
    n_cols = len(grid[0])
    split_count = 0
    for row in range(1, n_rows):
        for col in range(n_cols):
            if grid[row][col] == "." and (
                grid[row - 1][col] == "|" or grid[row - 1][col] == "S"
            ):
                grid[row][col] = "|"
            elif grid[row][col] == "^" and (
                grid[row - 1][col] == "|" or grid[row - 1][col] == "S"
            ):
                if col > 0:
                    grid[row][col - 1] = "|"
                if col < n_cols - 1:
                    grid[row][col + 1] = "|"

                split_count += 1

    return split_count


def task_2(grid: list[list[str]]) -> int:
    n_rows = len(grid)
    n_cols = len(grid[0])
    possibility_grid = [[0] * n_cols for _ in range(n_rows)]
    for col in range(n_cols):
        if grid[0][col] == "S":
            possibility_grid[0][col] = 1

    for row in range(1, n_rows):
        for col in range(n_cols):
            upper_level = possibility_grid[row - 1][col]
            if grid[row][col] == ".":
                possibility_grid[row][col] += upper_level
            elif grid[row][col] == "^":
                if col > 0:
                    possibility_grid[row][col - 1] += upper_level
                if col < n_cols - 1:
                    possibility_grid[row][col + 1] += upper_level

    return sum(possibility_grid[-1])


def main():
    with open("input.txt") as f:
        grid = parse_grid(f.read())

    print(f"Task 1: {task_1(grid)}")
    print(f"Task 2: {task_2(grid)}")


if __name__ == "__main__":
    main()
