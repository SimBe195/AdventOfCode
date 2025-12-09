import heapq


def parse_coords(input_str: str) -> list[tuple[int, int, int]]:
    return [tuple(map(int, line.split(","))) for line in input_str.splitlines()]  # type: ignore


def coord_pairs(
    coords: list[tuple[int, int, int]],
) -> list[tuple[tuple[int, int, int], tuple[int, int, int]]]:
    return [
        (coords[i], coords[j])
        for i in range(len(coords) - 1)
        for j in range(i + 1, len(coords))
    ]


def task_1(coords: list[tuple[int, int, int]], max_steps: int) -> int:
    pairs = coord_pairs(coords)
    heapq.heapify(pairs)

    def distance(pair: tuple[tuple[int, int, int], tuple[int, int, int]]) -> int:
        return sum((a - b) * (a - b) for a, b in zip(pair[0], pair[1]))

    sorted_pairs = heapq.nsmallest(max_steps, pairs, key=distance)

    coord_to_id = {}
    id_to_coords = {}
    max_id = 0

    for coord_1, coord_2 in sorted_pairs:
        if coord_1 not in coord_to_id and coord_2 not in coord_to_id:
            max_id += 1
            coord_to_id[coord_1] = max_id
            coord_to_id[coord_2] = max_id
            id_to_coords[max_id] = {coord_1, coord_2}
        elif coord_1 not in coord_to_id and coord_2 in coord_to_id:
            id = coord_to_id[coord_2]
            coord_to_id[coord_1] = id
            id_to_coords[id].add(coord_1)
        elif coord_1 in coord_to_id and coord_2 not in coord_to_id:
            id = coord_to_id[coord_1]
            coord_to_id[coord_2] = id
            id_to_coords[id].add(coord_2)
        else:
            id_1 = coord_to_id[coord_1]
            id_2 = coord_to_id[coord_2]
            if id_1 != id_2:
                for coord in id_to_coords[id_2]:
                    coord_to_id[coord] = id_1
                    id_to_coords[id_1].add(coord)
                del id_to_coords[id_2]

    product = 1
    for coord_list in heapq.nlargest(3, id_to_coords.values(), key=len):
        product *= len(coord_list)

    return product


def task_2(coords: list[tuple[int, int, int]]) -> int:
    pairs = coord_pairs(coords)
    heapq.heapify(pairs)

    def distance(pair: tuple[tuple[int, int, int], tuple[int, int, int]]) -> int:
        return sum((a - b) * (a - b) for a, b in zip(pair[0], pair[1]))

    coord_to_id = {}
    id_to_coords = {}
    max_id = 0

    for coord_1, coord_2 in sorted(pairs, key=distance):
        if coord_1 not in coord_to_id and coord_2 not in coord_to_id:
            max_id += 1
            coord_to_id[coord_1] = max_id
            coord_to_id[coord_2] = max_id
            id_to_coords[max_id] = {coord_1, coord_2}
        elif coord_1 not in coord_to_id and coord_2 in coord_to_id:
            id = coord_to_id[coord_2]
            coord_to_id[coord_1] = id
            id_to_coords[id].add(coord_1)
        elif coord_1 in coord_to_id and coord_2 not in coord_to_id:
            id = coord_to_id[coord_1]
            coord_to_id[coord_2] = id
            id_to_coords[id].add(coord_2)
        else:
            id_1 = coord_to_id[coord_1]
            id_2 = coord_to_id[coord_2]
            if id_1 != id_2:
                for coord in id_to_coords[id_2]:
                    coord_to_id[coord] = id_1
                    id_to_coords[id_1].add(coord)
                del id_to_coords[id_2]

        if len(id_to_coords) == 1 and len(next(iter(id_to_coords.values()))) == len(
            coords
        ):
            return coord_1[0] * coord_2[0]

    return 0


def main():
    with open("input.txt") as f:
        coords = parse_coords(f.read())

    print(f"Task 1: {task_1(coords, 1000)}")
    print(f"Task 2: {task_2(coords)}")


if __name__ == "__main__":
    main()
