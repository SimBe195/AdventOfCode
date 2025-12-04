def parse_grid(grid_str: str) -> list[list[str]]:
    return [list(row) for row in grid_str.split("\n") if len(row) > 0]


def get_neighbor_counts(grid: list[list[str]]) -> list[list[int]]:
    n_rows = len(grid)
    n_cols = len(grid[0])
    neighbor_counts = [[0] * n_cols for _ in range(n_rows)]

    for row in range(n_rows):
        for col in range(n_cols):
            if grid[row][col] == ".":
                continue
            for row_offset in range(-1, 2):
                if row + row_offset < 0 or row + row_offset >= n_rows:
                    continue
                for col_offset in range(-1, 2):
                    if col + col_offset < 0 or col + col_offset >= n_cols:
                        continue
                    if row_offset == 0 and col_offset == 0:
                        continue
                    neighbor_counts[row + row_offset][col + col_offset] += 1

    return neighbor_counts


def task_1(grid: list[list[str]]) -> int:
    neighbor_counts = get_neighbor_counts(grid)

    roll_counter = 0
    for row in range(len(grid)):
        for col in range(len(grid[0])):
            roll_counter += grid[row][col] == "@" and neighbor_counts[row][col] < 4

    return roll_counter


def task_2(grid: list[list[str]]) -> int:
    neighbor_counts = get_neighbor_counts(grid)

    roll_counter = 0
    while True:
        extra_rolls_removed = 0
        for row in range(len(grid)):
            for col in range(len(grid[0])):
                if grid[row][col] == "@" and neighbor_counts[row][col] < 4:
                    extra_rolls_removed += 1
                    grid[row][col] = "."
                    for row_offset in range(-1, 2):
                        if row + row_offset < 0 or row + row_offset >= len(grid):
                            continue
                        for col_offset in range(-1, 2):
                            if col + col_offset < 0 or col + col_offset >= len(grid[0]):
                                continue
                            if row_offset == 0 and col_offset == 0:
                                continue
                            neighbor_counts[row + row_offset][col + col_offset] -= 1

        roll_counter += extra_rolls_removed
        if extra_rolls_removed == 0:
            break

    return roll_counter


def main():
    with open("input.txt") as f:
        grid = parse_grid(f.read())

    print(f"Task 1: {task_1(grid)}")
    print(f"Task 2: {task_2(grid)}")


if __name__ == "__main__":
    main()
