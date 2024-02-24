#!/usr/bin/env python3

import subprocess
import cv2
import numpy as np


GIF_NAME = "skull.gif"
RESIZE_FACTOR = 0.5

def resize_image(file_name: str) -> np.ndarray:
    """Resize image from 200x200 to 128x64."""
    image = cv2.imread(file_name)
    print(f"Height x Width x Channels: {image.shape}, dtype: {image.dtype}")

    small_image = cv2.resize(image, (0, 0), fx=RESIZE_FACTOR, fy=RESIZE_FACTOR)
    crop_image = small_image[15:79, :]

    margin_image = np.zeros((64, 14, 3), dtype=image.dtype)
    output_image = np.hstack((margin_image, crop_image))
    output_image = np.hstack((output_image, margin_image))

    cv2.imshow("image_window", output_image)
    cv2.waitKey(0)

    return output_image


def main() -> None:
    # output = subprocess.run(
    #     ["ffmpeg", "-r", "1", "-i", f"{GIF_NAME}", "-r", " 1", r"%04d.bmp"],
    #     stdout=subprocess.PIPE,
    # )

    resize_image("0001.bmp")



if __name__ == "__main__":
    main()
