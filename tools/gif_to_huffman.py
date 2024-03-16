#!/usr/bin/env python3

from pathlib import Path
import subprocess
import cv2
import numpy as np


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
