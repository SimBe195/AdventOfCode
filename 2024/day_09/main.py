from layout import Block, Layout


def parse_file(filename: str) -> Layout:
    """Parse file and create a layout of blocks"""
    with open(filename, "r") as f:
        line = f.readline()
    result = Layout()
    for idx, length in enumerate(map(int, line[:-1])):
        if idx % 2:
            result.append(Block.create_free_block(length))
        else:
            result.append(Block(idx // 2, length))
    result.append(Block.create_free_block(0))  # Add trailing free block
    return result


def consolidate_space(layout: Layout) -> None:
    """Merges consecutive blocks with the same ID."""
    current = layout.head
    while current and current.next:
        if current.id == current.next.id:
            current.length += current.next.length
            layout.remove(current.next)
            current = current.next
        else:
            current = current.next


def is_compacted(layout: Layout) -> bool:
    """Checks if the free space in the layout is consolidated into one continuous block."""
    return sum(1 for block in layout if block.is_free()) <= 1


def find_block_by_id(layout: Layout, id: int) -> Block:
    """Find the block with the given ID in the layout."""
    for block in layout:
        if block.id == id:
            return block
    raise ValueError(f"No block with id {id} present in layout")


def clean_empty_blocks(layout: Layout) -> None:
    """Removes all blocks with a length of 0"""
    current = layout.head
    while current:
        next_block = current.next
        if current.length == 0:
            layout.remove(current)
        current = next_block


def move_file_step(layout: Layout) -> None:
    """Performs one step of moving memory from the rightmost file to the leftmost free space."""
    free_block = next(block for block in layout if block.is_free())
    file_block = next(block for block in reversed(layout) if not block.is_free())

    moved_space = min(free_block.length, file_block.length)

    new_block = Block(file_block.id, moved_space)
    layout.insert_before(free_block, new_block)

    file_block.length -= moved_space
    free_block.length -= moved_space

    clean_empty_blocks(layout)
    consolidate_space(layout)


def try_move_whole_file_step(layout: Layout, file_id: int) -> None:
    """Attempts to move the entire file with the given id to the leftmost free space that can fit it."""
    file_block = find_block_by_id(layout, file_id)

    for block in layout:
        if block == file_block:
            return
        if block.is_free() and block.length >= file_block.length:
            new_block = Block(file_id, file_block.length)
            layout.insert_before(block, new_block)
            block.length -= file_block.length
            file_block.free()

            clean_empty_blocks(layout)
            consolidate_space(layout)
            return


def compact_file(layout: Layout) -> None:
    """Compacts the file layout step by step."""
    while not is_compacted(layout):
        move_file_step(layout)


def compact_file_without_fragmentation(layout: Layout) -> None:
    """Compacts the file layout by attempting to move entire files in order from highest to lowest id."""
    max_id = max(block.id for block in layout if not block.is_free())

    for id in range(max_id, -1, -1):
        try_move_whole_file_step(layout, id)


def calc_checksum(layout: Layout) -> int:
    idx = 0
    checksum = 0
    for block in layout:
        if not block.is_free():
            checksum += block.id * sum(range(idx, idx + block.length))
        idx += block.length
    return checksum


def challenge_1(filename: str) -> int:
    layout = parse_file(filename)
    compact_file(layout)
    return calc_checksum(layout)


def challenge_2(filename: str) -> int:
    layout = parse_file(filename)
    compact_file_without_fragmentation(layout)
    return calc_checksum(layout)


if __name__ == "__main__":
    print(f"Challenge 1 test: {challenge_1("testinput.txt")}")
    print(f"Challenge 1: {challenge_1("input.txt")}")
    print(f"Challenge 2 test: {challenge_2("testinput.txt")}")
    print(f"Challenge 2: {challenge_2("input.txt")}")
