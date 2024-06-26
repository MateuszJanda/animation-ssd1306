#!/usr/bin/env python3

from collections import defaultdict
from pathlib import Path
import subprocess
import cv2
import numpy as np
import heapq
import typing


GIF_FILE_NAME = "skull.gif"
RESIZE_FACTOR = 0.5

ARRAY_CHUNK = 128


class HuffmanCoding:
    """
    - https://en.wikipedia.org/wiki/Huffman_coding
    - https://www.geeksforgeeks.org/text-file-compression-and-decompression-using-huffman-coding/
    """

    NIL = -1
    MISSING_VALUE = -2

    class Node:
        def __init__(self, freq: int, value: typing.Optional[int]) -> None:
            """Node in binary tree."""
            self.freq = freq
            self.value = value
            self.left = None
            self.right = None

        def __repr__(self) -> str:
            """Node stored data (freq, value)."""
            return f"Node (freq: {self.freq}, value: {self.value})"

        def __lt__(self, other: "HuffmanCoding.Node") -> bool:
            """Compare nodes by their frequencies (less than comparator)."""
            return self.freq < other.freq

    def __init__(self) -> None:
        """Init Huffman coding fields."""
        self._freq_map = defaultdict(int)
        self._root = None
        self._bt_array = None
        self._coding_table = {}

    def _build_binary_tree(self) -> "HuffmanCoding.Node":
        """Build binary tree from frequency map."""
        min_heap = [
            HuffmanCoding.Node(freq, value) for value, freq in self._freq_map.items()
        ]
        heapq.heapify(min_heap)

        while len(min_heap) > 1:
            node_left = heapq.heappop(min_heap)
            node_right = heapq.heappop(min_heap)

            parent = HuffmanCoding.Node(node_left.freq + node_right.freq, None)
            parent.left = node_left
            parent.right = node_right
            heapq.heappush(min_heap, parent)

        self._root = heapq.heappop(min_heap)

    def _convert_binary_tree_to_array(self) -> list:
        """Convert binary tree to array (list)."""
        if self._root is None:
            raise Exception("root is None")

        # Root is at index 1 to simplify calculation
        self._bt_array = [HuffmanCoding.NIL, HuffmanCoding.MISSING_VALUE]
        if self._root.value is not None:
            self._bt_array[1] = self._root.value

        def dfs(parent: "HuffmanCoding.Node", parent_idx: int) -> None:
            """
            Process all nodes (using recursive DFS - depth-first search algorithm) and fill
            binary tree array.
            """
            if parent is None:
                return

            if parent.left is None and parent.right is None:
                return

            left_idx = 2 * parent_idx
            right_idx = 2 * parent_idx + 1

            if len(self._bt_array) <= right_idx:
                additional_len = right_idx - len(self._bt_array) + 1
                self._bt_array.extend([HuffmanCoding.NIL] * additional_len)

            self._bt_array[left_idx] = HuffmanCoding.NIL
            if parent.left is not None:
                if parent.left.value is None:
                    self._bt_array[left_idx] = HuffmanCoding.MISSING_VALUE
                else:
                    self._bt_array[left_idx] = parent.left.value

            self._bt_array[right_idx] = HuffmanCoding.NIL
            if parent.right is not None:
                if parent.right.value is None:
                    self._bt_array[right_idx] = HuffmanCoding.MISSING_VALUE
                else:
                    self._bt_array[right_idx] = parent.right.value

            dfs(parent.left, left_idx)
            dfs(parent.right, right_idx)

        dfs(self._root, 1)

    def _create_coding_table(self, node: "HuffmanCoding.Node", bin_str="") -> dict:
        """Create coding table by traversing binary tree in recursive manner."""
        if node.value is not None:
            return {node.value: bin_str}

        table = {}
        table.update(self._create_coding_table(node.left, bin_str + "0"))
        table.update(self._create_coding_table(node.right, bin_str + "1"))
        return table

    def insert_value(self, value: int) -> None:
        """Insert value to frequency map."""
        self._freq_map[value] += 1

    def calculate_coding(self) -> None:
        """Build coding table to all stored values."""
        self._build_binary_tree()
        self._convert_binary_tree_to_array()
        self._coding_table = self._create_coding_table(self._root)

    def info(self) -> None:
        """Print info from collected data."""
        if self._root is None:
            raise Exception("root doesn't exist")

        # print(self._bt_array)
        print(f"\nBinary tree array size: {len(self._bt_array)} (nodes)")

        print("Huffman codes. Values -> Code:")
        print("----------------------")
        count_bits = 0
        for value, freq in self._freq_map.items():
            count_bits += len(self._coding_table[value]) * freq
            print(" %-4r |%15s" % (value, self._coding_table[value]))

        print(f"Compression: {count_bits} bits, {count_bits/8:.2f} bytes.")

    def get_binary_tree_array(self) -> list:
        """Getter for binary tree array."""
        return self._bt_array

    def get_huffman_coding(self) -> dict:
        """Getter for Huffman coding table (dict)."""
        return self._coding_table


def convert_image(file_name: str) -> np.ndarray:
    """Resize image from 200x200 to 128x64 and do additional manipulation for better look."""
    image = cv2.imread(file_name)
    # print(
    #     f"{file_name}, Height x Width x Channels: {image.shape}, dtype: {image.dtype}"
    # )

    # Resize by half
    resized_image = cv2.resize(image, (0, 0), fx=RESIZE_FACTOR, fy=RESIZE_FACTOR)
    # Crop empty rows
    cropped_image = resized_image[15:79, :]

    # Add margin (columns) to match 128x64 size
    margin = np.zeros((64, 14, 3), dtype=image.dtype)
    new_size_image = np.hstack((margin, cropped_image))
    new_size_image = np.hstack((new_size_image, margin))

    # Convert to grayscale
    gray_image = cv2.cvtColor(new_size_image, cv2.COLOR_BGR2GRAY)

    # Finds edges in an image using the Canny algorithm
    output_image = cv2.Canny(gray_image, 150, 250)
    # print(
    #     f"Height x Width x Channels. Input {image.shape}, dtype: {image.dtype}."
    #     f" Output {output_image.shape}, dtype: {output_image.dtype}"
    # )

    return output_image


def convert_image_to_list(image: np.ndarray) -> list:
    """Convert image (numpy array) to list."""
    result = []

    for y in range(0, image.shape[0], 8):
        for x in range(image.shape[1]):
            bits = reversed(["1" if val > 0 else "0" for val in image[y : y + 8, x]])
            value = int("0b" + "".join(bits), 2)
            result.append(value)

    return result


def rs_insert_header(f: typing.BinaryIO) -> None:
    """Insert header section into encoded_frames.rs."""
    f.write("use avr_progmem::progmem;\n\n")
    f.write("// Autogenerated file (gif_to_huffman.py), so rust formatting is disabled.\n")
    f.write("#[rustfmt::skip]\n")


def rs_insert_progmem_start(f: typing.BinaryIO) -> None:
    """Insert progmem block (start) into encoded_frames.rs."""
    f.write("\nprogmem! {\n\n")


def rs_insert_progmem_end(f: typing.BinaryIO) -> None:
    """Insert progmem block (end) into encoded_frames.rs."""
    f.write("\n} // progmem\n")


def rs_insert_huffman_indexes_and_values(f: typing.BinaryIO, hc: HuffmanCoding) -> None:
    """
    Insert BINARY_TREE_CODES BINARY_TREE_VALUES into encoded_frames.rs. They are not in
    progmem block!
    """
    # Insert binary tree codes (indexes to values) in ascending order
    coding_table = hc.get_huffman_coding()
    code_to_value = sorted(
        [(int(f"0b{code}", 2), value) for value, code in coding_table.items()]
    )
    f.write(f"\npub static BINARY_TREE_CODES: [u16; {len(code_to_value)}] = [\n    ")
    for code, _ in code_to_value:
        f.write(f"0x{code:04x},")
    f.write("\n];\n")

    # Insert binary tree values
    f.write(f"\npub static BINARY_TREE_VALUES: [u8; {len(code_to_value)}] = [\n    ")
    for _, value in code_to_value:
        f.write(f"0x{value:02x},")
    f.write("\n];\n")


def rs_insert_huffman_coding_tree(f: typing.BinaryIO, hc: HuffmanCoding) -> None:
    """Insert Huffman coding tree (represented as bit array) into encoded_frames.rs."""
    # Insert bit array with leafs marked as 1 (0 in other case)
    bt_array = hc.get_binary_tree_array()
    f.write(
        f"pub static progmem BINARY_TREE_LEAFS_BITS_SIZE: usize = {len(bt_array)};\n"
    )

    # Extend bit array if the size is not a multiple of (128*8). Needed by load_sub_array<128>()
    bits_array = ["0" if node_value < 0 else "1" for node_value in bt_array]
    if len(bits_array) % (ARRAY_CHUNK * 8) != 0:
        padding_size = (ARRAY_CHUNK * 8) - (len(bits_array) % (ARRAY_CHUNK * 8))
        bits_array.extend("0" * padding_size)

    f.write(
        f"pub static progmem BINARY_TREE_LEAFS: [u8; {len(bits_array) // 8}] = [\n    "
    )
    for i in range(0, len(bits_array), 8):
        value = int("0b" + "".join(bits_array[i : i + 8]), 2)
        f.write(f"0x{value:02x},")
    f.write("\n];\n")


def rs_insert_frame(f: typing.BinaryIO, hc: HuffmanCoding, image: np.ndarray, index: int) -> None:
    """Insert frame (array) into encoded_frames.rs."""
    coding_table = hc.get_huffman_coding()
    array = convert_image_to_list(image)

    bits_str = "".join(coding_table[value] for value in array)
    f.write(
        f"\npub static progmem SKULL_FRAME{index:02d}_BITS_SIZE: usize = {len(bits_str)};\n"
    )

    # Extend bits_str if the size is not a multiple of (128*8). Needed by load_sub_array<128>()
    if len(bits_str) % (ARRAY_CHUNK * 8) != 0:
        padding_size = (ARRAY_CHUNK * 8) - (len(bits_str) % (ARRAY_CHUNK * 8))
        bits_str += "0" * padding_size

    f.write(
        f"pub static progmem SKULL_FRAME{index:02d}: [u8; {len(bits_str) // 8}] = [\n    "
    )
    for i in range(0, len(bits_str), 8):
        value = int("0b" + "".join(bits_str[i : i + 8]), 2)
        f.write(f"0x{value:02x},")
    f.write("\n];\n")


def main() -> None:
    """Main function. Convert skull.gif -> *.bmp frames -> encoded_frames.rs (Huffman coding)."""
    # Extract frames from gif file
    subprocess.run(
        ["ffmpeg", "-r", "1", "-i", f"{GIF_FILE_NAME}", "-r", " 1", r"frame_%04d.bmp"],
        stdout=subprocess.PIPE,
    )

    # Create sorted list of frames files
    files_paths = sorted(
        [
            str(file_path)
            for file_path in Path("./").rglob("frame_*.bmp")
            if file_path.is_file()
        ]
    )

    # Create huffman coding for all frames
    hc = HuffmanCoding()
    for file_path in files_paths:
        image = convert_image(file_path)
        array = convert_image_to_list(image)

        for value in array:
            hc.insert_value(value)

    hc.calculate_coding()
    hc.info()

    # Encode all frames and create encoded_frames.rs
    with open("encoded_frames.rs", "w") as f:
        rs_insert_header(f)
        rs_insert_huffman_indexes_and_values(f, hc)

        rs_insert_progmem_start(f)
        rs_insert_huffman_coding_tree(f, hc)

        for index, file_path in enumerate(files_paths):
            image = convert_image(file_path)
            rs_insert_frame(f, hc, image, index)

        rs_insert_progmem_end(f)


if __name__ == "__main__":
    main()
