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
    gray_image = cv2.threshold(gray_image, THRESHOLD, 255, cv2.THRESH_BINARY)[1]
    # gray_image = cv2.threshold(gray_image, 128, 255, cv2.THRESH_BINARY | cv2.THRESH_OTSU)[1]

    cv2.imshow("image_window", gray_image)
    cv2.waitKey(0)

    return output_image


def main() -> None:
    # output = subprocess.run(
    #     ["ffmpeg", "-r", "1", "-i", f"{GIF_NAME}", "-r", " 1", r"%04d.bmp"],
    #     stdout=subprocess.PIPE,
    # )

    files_paths = sorted(
        [file_path for file_path in Path("./").rglob("*.bmp") if file_path.is_file()]
    )

    resize_image("0001.bmp")
    # images = [resize_image(file_name) for file_name in files_paths]


if __name__ == "__main__":
    main()
