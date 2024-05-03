#!/usr/bin/env python3

from collections import defaultdict
from pathlib import Path
import subprocess
import cv2
import numpy as np
import heapq
import typing


GIF_NAME = "skull.gif"
RESIZE_FACTOR = 0.5
THRESHOLD = 97


def resize_image(file_name: str) -> np.ndarray:
    """Resize image from 200x200 to 128x64."""
    image = cv2.imread(file_name)
    print(
        f"{file_name}, Height x Width x Channels: {image.shape}, dtype: {image.dtype}"
    )

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
    # output_image = cv2.threshold(gray_image, THRESHOLD, 255, cv2.THRESH_BINARY)[1]
    # output_image = cv2.threshold(gray_image, 128, 255, cv2.THRESH_BINARY | cv2.THRESH_OTSU)[1]
    # output_image = cv2.adaptiveThreshold(
    #     gray_image, 255, cv2.ADAPTIVE_THRESH_MEAN_C, cv2.THRESH_BINARY, 13, 5
    # )
    # Finds edges in an image using the Canny algorithm
    output_image = cv2.Canny(gray_image, 150, 250)
    print(
        f"Height x Width x Channels. Input {image.shape}, dtype: {image.dtype}."
        f" Output {output_image.shape}, dtype: {output_image.dtype}"
    )

    # cv2.imshow("image_window", output_image)
    # cv2.waitKey(0)

    return output_image


# def convert_image_to_array(image: np.ndarray) -> str:
#     """Convert image (numpy array) to rust array."""
#     output = "#[rustfmt::skip]\n"
#     output += "const SKULL_FRAME: &[u8] = &[\n"
#     # output_image[output_image > 0] = 1
#     for y in range(image.shape[0]):
#         output += "    "
#         for x in range(image.shape[1] // 8):
#             output += (
#                 "0b"
#                 + "".join(
#                     ["1" if val > 0 else "0" for val in image[y, x * 8 : (x * 8) + 8]]
#                 )
#                 + ","
#             )

#         output += "\n"

#     output += "];\n"

#     print(output)
#     return output

# https://www.geeksforgeeks.org/text-file-compression-and-decompression-using-huffman-coding/


class HuffmanCoding:
    NIL = -1
    MISSING_VALUE = -2

    class Node:
        def __init__(self, freq: int, value: typing.Optional[int]) -> None:
            self.freq = freq
            self.value = value
            self.left = None
            self.right = None

        def __repr__(self):
            return f"Node freq, value: {self.freq, self.value}"

        def __lt__(self, other: "HuffmanCoding.Node") -> bool:
            return self.freq < other.freq

    # FREQ_ARRAY_SIZE = 256

    def __init__(self) -> None:
        self._freq_map = defaultdict(int)

    def insert_value(self, value: int) -> None:
        self._freq_map[value] += 1

    def compress(self) -> None:
        root = self._build_binary_tree()
        bt_array = self._convert_binary_tree_to_array(root)
        print(bt_array)

    def _build_binary_tree(self) -> "HuffmanCoding.Node":
        min_heap = [
            HuffmanCoding.Node(freq, value) for value, freq in self._freq_map.items()
        ]
        heapq.heapify(min_heap)

        while len(min_heap) > 1:
            node_left = heapq.heappop(min_heap)
            node_right = heapq.heappop(min_heap)

            # if (node_left.value is not None):
            #     print(f"l: {chr(node_left.value)}")
            # else:
            #     print("l: None")

            # if (node_right.value is not None):
            #     print(f"r: {chr(node_right.value)}")
            # else:
            #     print("r: None")

            parent = HuffmanCoding.Node(node_left.freq + node_right.freq, None)
            parent.left = node_left
            parent.right = node_right
            heapq.heappush(min_heap, parent)

        root = heapq.heappop(min_heap)
        # print(root)
        return root

    def _convert_binary_tree_to_array(self, root: "HuffmanCoding.Node") -> typing.List:
        if root is None:
            raise Exception("root is None")

        bt_array = [HuffmanCoding.NIL, HuffmanCoding.MISSING_VALUE]
        if root.value is not None:
            bt_array[1] = root.value

        def dfs(parent: "HuffmanCoding.Node", parent_idx: int) -> None:
            if parent is None:
                return

            left_idx = 2 * parent_idx
            right_idx = 2 * parent_idx + 1

            if len(bt_array) <= right_idx:
                additional_len = right_idx - len(bt_array) + 1
                bt_array.extend([HuffmanCoding.NIL] * additional_len)

            bt_array[left_idx] = HuffmanCoding.NIL
            if parent.left is not None:
                if parent.left.value is None:
                    bt_array[left_idx] = HuffmanCoding.MISSING_VALUE
                else:
                    bt_array[left_idx] = parent.left.value

            bt_array[right_idx] = HuffmanCoding.NIL
            if parent.right is not None:
                if parent.right.value is None:
                    bt_array[right_idx] = HuffmanCoding.MISSING_VALUE
                else:
                    bt_array[right_idx] = parent.right.value

            dfs(parent.left, left_idx)
            dfs(parent.right, right_idx)

        dfs(root, 1)

        return bt_array


def convert_image_to_array2(image: np.ndarray, file_suffix: int) -> str:
    """Convert image (numpy array) to rust array."""

    output = f"pub static progmem SKULL_FRAME{file_suffix:02d}: [u8; 1024] = [\n"

    for y in range(0, image.shape[0], 8):
        output += "    "
        for x in range(image.shape[1]):
            if x != 0:
                output += " "

            bits = reversed(["1" if val > 0 else "0" for val in image[y : y + 8, x]])
            value = int("0b" + "".join(bits), 2)
            output += f"0x{value:02x},"

        output += "\n"

    output += "];\n"

    print(output)
    return output


def main() -> None:
    h = HuffmanCoding()
    for ch in "Stressed-desserts":
        h.insert_value(ord(ch))

    h.compress()


def main2() -> None:
    subprocess.run(
        ["ffmpeg", "-r", "1", "-i", f"{GIF_NAME}", "-r", " 1", r"frame_%04d.bmp"],
        stdout=subprocess.PIPE,
    )

    files_paths = sorted(
        [
            str(file_path)
            for file_path in Path("./").rglob("frame_*.bmp")
            if file_path.is_file()
        ]
    )

    with open("raw_image.rs", "w") as f:
        f.write("use avr_progmem::progmem;\n\n")
        f.write("#[rustfmt::skip]\n")
        f.write("progmem! {\n")

        # f.write(f"    pub const SKULL_FRAME: &[[u8; 1024]; {len(files_paths)}] = &[\n")
        for index, file_path in enumerate(files_paths):
            # image = resize_image("0001.bmp")
            image = resize_image(file_path)

            # convert_image_to_array(image)
            array = convert_image_to_array2(image, index)
            f.write(array)
            # images = [resize_image(file_name) for file_name in files_paths]

        # f.write("\n];\n")
        f.write("}\n")


if __name__ == "__main__":
    main()
