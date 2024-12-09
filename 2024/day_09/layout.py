from dataclasses import dataclass

FREE_ID = -1


@dataclass
class Block:
    id: int
    length: int
    prev: "Block | None" = None  # Predecessor
    next: "Block | None" = None  # Successor

    @classmethod
    def create_free_block(cls, length: int):
        return cls(FREE_ID, length)

    def free(self) -> None:
        self.id = FREE_ID

    def is_free(self) -> bool:
        return self.id == FREE_ID


class Layout:
    def __init__(self):
        self.head: Block | None = None
        self.tail: Block | None = None

    def append(self, block: Block) -> None:
        """Appends a block to the end of the list."""
        if not self.tail:  # Empty list
            self.head = self.tail = block
        else:
            self.tail.next = block
            block.prev = self.tail
            self.tail = block

    def remove(self, block: Block) -> None:
        """Removes a block from the list."""
        if block.prev:
            block.prev.next = block.next
        else:
            self.head = block.next  # Update head if removing the first block

        if block.next:
            block.next.prev = block.prev
        else:
            self.tail = block.prev  # Update tail if removing the last block

        block.prev = block.next = None  # Clean up references

    def insert_before(self, ref_block: Block, new_block: Block) -> None:
        """Inserts a new block before a reference block."""
        new_block.next = ref_block
        new_block.prev = ref_block.prev

        if ref_block.prev:
            ref_block.prev.next = new_block
        else:
            self.head = new_block  # Update head if inserting at the start

        ref_block.prev = new_block

    def __iter__(self):
        """Forward iterator."""
        current = self.head
        while current:
            yield current
            current = current.next

    def __reversed__(self):
        """Backward iterator."""
        current = self.tail
        while current:
            yield current
            current = current.prev
