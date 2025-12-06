import functools


def parse_problems(problem_str: str) -> tuple[list[list[int]], list[str]]:
    num_matrix = [
        [0] * (problem_str.count("\n") - 1)
        for _ in range(len(problem_str.split("\n", maxsplit=1)[0].split()))
    ]
    op_list = []
    for col, line in enumerate(problem_str.splitlines()):
        for row, val in enumerate(line.split()):
            if col == len(num_matrix[0]):
                op_list.append(val)
            else:
                num_matrix[row][col] = int(val)

    return num_matrix, op_list


def parse_problems_2(problem_str: str) -> tuple[list[list[int]], list[str]]:
    op_list = list(problem_str.splitlines()[-1].split())

    n_cols = len(problem_str.split("\n", maxsplit=1)[0])
    lines = problem_str.splitlines()[:-1]

    num_matrix = []
    current_nums = []
    for col in range(n_cols):
        if all(line[col] == " " for line in lines):
            num_matrix.append(current_nums)
            current_nums = []
            continue
        num_str = ""
        for line in lines:
            if line[col] != " ":
                num_str += line[col]
        current_nums.append(int(num_str))
    num_matrix.append(current_nums)

    return num_matrix, op_list


def calc_problems(num_matrix: list[list[int]], op_list: list[str]) -> int:
    total = 0

    def add(x: int, y: int) -> int:
        return x + y

    def mul(x: int, y: int) -> int:
        return x * y

    for num_list, op_symbol in zip(num_matrix, op_list):
        if op_symbol == "*":
            op = mul
        else:
            op = add

        total += functools.reduce(op, num_list)

    return total


def main() -> None:
    with open("input.txt") as f:
        problem_str = f.read()

    num_matrix, op_list = parse_problems(problem_str)
    print(f"Task 1: {calc_problems(num_matrix, op_list)}")
    num_matrix, op_list = parse_problems_2(problem_str)
    print(f"Task 2: {calc_problems(num_matrix, op_list)}")


if __name__ == "__main__":
    main()
