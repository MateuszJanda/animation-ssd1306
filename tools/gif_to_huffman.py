#!/usr/bin/env python3

import subprocess


def main() -> None:
    gif_name = "skull.gif"
    output = subprocess.run(
        ["ffmpeg", "-r", "1", "-i", f"{gif_name}", "-r", " 1", r"%04d.bmp"],
        stdout=subprocess.PIPE,
    )


if __name__ == "__main__":
    main()
