#!/bin/bash

VIDEO_FILE="captured_screen.mp4"
VIDEO_FILE_SCALED="captured_screen_scaled.mp4"


PALETTE_FILE="tmp_pallete.png"
OUTPUT_GIF="output.gif"
FILTERS="fps=25"

ffmpeg -i $VIDEO_FILE -s 512x288 -c:a copy $VIDEO_FILE_SCALED
ffmpeg -v warning -i $VIDEO_FILE_SCALED -vf "$FILTERS,palettegen" -y $PALETTE_FILE
ffmpeg -v warning -i $VIDEO_FILE_SCALED -i $PALETTE_FILE -lavfi "$FILTERS [x]; [x][1:v] paletteuse" -y $OUTPUT_GIF
