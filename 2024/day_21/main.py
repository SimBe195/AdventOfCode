from functools import cache
from typing import Literal


NumericalKey = int | Literal["A"]
NumericalCode = list[NumericalKey]
DirectionalKey = Literal["<", ">", "^", "v", "A"]
DirectionalCode = list[DirectionalKey]


def parse_input(filename: str) -> list[NumericalCode]:
    result: list[NumericalCode] = []
    with open(filename, "r") as f:
        for line in f:
            code: NumericalCode = []
            for char in line.strip():
                if char == "A":
                    code.append(char)
                else:
                    code.append(int(char))
            result.append(code)

    return result


@cache
def numerical_key_to_coord(key: NumericalKey) -> tuple[int, int]:
    match key:
        case 7:
            return (0, 0)
        case 8:
            return (0, 1)
        case 9:
            return (0, 2)
        case 4:
            return (1, 0)
        case 5:
            return (1, 1)
        case 6:
            return (1, 2)
        case 1:
            return (2, 0)
        case 2:
            return (2, 1)
        case 3:
            return (2, 2)
        case 0:
            return (3, 1)
        case "A":
            return (3, 2)
        case _:
            raise ValueError(f"Invalid key {key}.")


@cache
def directional_key_to_coord(key: DirectionalKey) -> tuple[int, int]:
    match key:
        case "^":
            return (0, 1)
        case "A":
            return (0, 2)
        case "<":
            return (1, 0)
        case "v":
            return (1, 1)
        case ">":
            return (1, 2)


shortest_numerical_key_path = {
    "A": {
        "A": [],
        0: list("<"),
        1: list("^<<"),
        2: list("<^"),
        3: list("^"),
        4: list("^^<<"),
        5: list("<^^"),
        6: list("^^"),
        7: list("^^^<<"),
        8: list("<^^^"),
        9: list("^^^"),
    },
    0: {
        "A": list(">"),
        0: [],
        1: list("^<"),
        2: list("^"),
        3: list("^>"),
        4: list("^^<"),
        5: list("^^"),
        6: list("^^>"),
        7: list("^^^<"),
        8: list("^^^"),
        9: list("^^^>"),
    },
    1: {
        "A": list(">>v"),
        0: list(">v"),
        1: [],
        2: list(">"),
        3: list(">>"),
        4: list("^"),
        5: list("^>"),
        6: list("^>>"),
        7: list("^^"),
        8: list("^^>"),
        9: list("^^>>"),
    },
    2: {
        "A": list("v>"),
        0: list("v"),
        1: list("<"),
        2: [],
        3: list(">"),
        4: list("<^"),
        5: list("^"),
        6: list("^>"),
        7: list("<^^"),
        8: list("^^"),
        9: list("^^>"),
    },
    3: {
        "A": list("v"),
        0: list("<v"),
        1: list("<<"),
        2: list("<"),
        3: [],
        4: list("<<^"),
        5: list("<^"),
        6: list("^"),
        7: list("<<^^"),
        8: list("<^^"),
        9: list("^^"),
    },
    4: {
        "A": list(">>vv"),
        0: list(">vv"),
        1: list("v"),
        2: list("v>"),
        3: list("v>>"),
        4: [],
        5: list(">"),
        6: list(">>"),
        7: list("^"),
        8: list("^>"),
        9: list("^>>"),
    },
    5: {
        "A": list("vv>"),
        0: list("vv"),
        1: list("<v"),
        2: list("v"),
        3: list("v>"),
        4: list("<"),
        5: [],
        6: list(">"),
        7: list("<^"),
        8: list("^"),
        9: list("^>"),
    },
    6: {
        "A": list("vv"),
        0: list("<vv"),
        1: list("<<v"),
        2: list("<v"),
        3: list("v"),
        4: list("<<"),
        5: list("<"),
        6: [],
        7: list("<<^"),
        8: list("<^"),
        9: list("^"),
    },
    7: {
        "A": list(">>vvv"),
        0: list(">vvv"),
        1: list("vv"),
        2: list("vv>"),
        3: list("vv>>"),
        4: list("v"),
        5: list("v>"),
        6: list("v>>"),
        7: [],
        8: list(">"),
        9: list(">>"),
    },
    8: {
        "A": list("vvv>"),
        0: list("vvv"),
        1: list("<vv"),
        2: list("vv"),
        3: list("vv>"),
        4: list("<v"),
        5: list("v"),
        6: list("v>"),
        7: list("<"),
        8: [],
        9: list(">"),
    },
    9: {
        "A": list("vvv"),
        0: list("<vvv"),
        1: list("<<vv"),
        2: list("<vv"),
        3: list("vv"),
        4: list("<<v"),
        5: list("<v"),
        6: list("v"),
        7: list("<<"),
        8: list("<"),
        9: [],
    },
}


def get_shortest_numerical_key_path(
    from_key: NumericalKey, to_key: NumericalKey
) -> DirectionalCode:
    return shortest_numerical_key_path[from_key][to_key]


shortest_directional_key_path = {
    "A": {
        "A": [],
        "^": list("<"),
        "<": list("v<<"),
        "v": list("<v"),
        ">": list("v"),
    },
    "^": {
        "A": list(">"),
        "^": [],
        "<": list("v<"),
        "v": list("v"),
        ">": list("v>"),
    },
    "<": {
        "A": list(">>^"),
        "^": list(">^"),
        "<": [],
        "v": list(">"),
        ">": list(">>"),
    },
    "v": {
        "A": list("^>"),
        "^": list("^"),
        "<": list("<"),
        "v": [],
        ">": list(">"),
    },
    ">": {
        "A": list("^"),
        "^": list("<^"),
        "<": list("<<"),
        "v": list("<"),
        ">": [],
    },
}


def get_shortest_directional_key_path(
    from_key: DirectionalKey, to_key: DirectionalKey
) -> DirectionalCode:
    return shortest_directional_key_path[from_key][to_key]


def expand_numerical_sequence(
    keys: NumericalCode,
) -> DirectionalCode:
    best_sequence: DirectionalCode = []
    previous_key = "A"
    for key in keys:
        partial_path = get_shortest_numerical_key_path(previous_key, key)
        best_sequence += partial_path
        best_sequence.append("A")
        previous_key = key

    return best_sequence


@cache
def cost(
    from_key: DirectionalKey, to_key: DirectionalKey, remaining_expansions: int
) -> int:
    if remaining_expansions == 0:
        return 1
    total_cost = 0
    path = get_shortest_directional_key_path(from_key, to_key) + ["A"]
    previous_key = "A"
    for key in path:
        total_cost += cost(previous_key, key, remaining_expansions - 1)
        previous_key = key
    return total_cost


def complexity(keys: NumericalCode, directional_keypads: int) -> int:
    dirpad_sequence = expand_numerical_sequence(keys)
    total_cost = 0
    previous_key = "A"

    for key in dirpad_sequence:
        total_cost += cost(previous_key, key, directional_keypads)
        previous_key = key
    numerical_value = int("".join(map(str, keys))[:-1])

    return total_cost * numerical_value


def complexity_sum(codes: list[NumericalCode], directional_keypads: int) -> int:
    return sum(complexity(code, directional_keypads) for code in codes)


def main() -> None:
    print("Challenge 1 test: ", complexity_sum(parse_input("testinput.txt"), 2))
    print("Challenge 1: ", complexity_sum(parse_input("input.txt"), 2))

    print("Challenge 2: ", complexity_sum(parse_input("input.txt"), 25))


if __name__ == "__main__":
    main()
