# About

Simple skull animation on Arduino Uno R3 and [WaveShare 0.96inch OLED (B)](https://www.waveshare.com/wiki/0.96inch_OLED_%28B%29#User_Guides_for_Arduino) display.
Frames are compressed and decompress by [Huffman algorithm](https://en.wikipedia.org/wiki/Huffman_coding) to fit into 32 KB flash memory.
Project uses two great libraries: [avr-hal](https://github.com/Rahix/avr-hal) and [modified](https://github.com/MateuszJanda/ssd1306) [ssd1306 driver](https://github.com/jamwaffles/ssd1306) to bypass buffering (and save memory).

# Hardware

- SSD1306 Display - WaveShare 0.96inch OLED (B) - connected to 4-wire SPI (default case)

# Pinning

Arduino Uno R3 has one Serial Peripheral Interface (SPI) controller with predefined pins. In this case CIPO (D12) is not used.

Arduino Pin | Name | Other common names | [Description](https://en.wikipedia.org/wiki/Serial_Peripheral_Interface)
----------- | ---- | ------------------ | ---
D10 | SS | **CS**, nCS, CSB, CSN, nSS, STE, SYNC | Chip Select
D11 | COPI | **MOSI**, SIMO, SDO, DO, DOUT, SO, MTSR | Master out, slave in
D12 | CIPO | **MISO**, SOMI, SDI, DI, DIN, SI, MRST | Master in, slave out
D13 | SCK | **SCLK**, CLK | Serial Clock

WaveShare 0.96inch (SSD1306) configured in 4-wire mode.

OLED Pin - signatures on the device | Description in 4-wire mode | Connected with Arduino Pin
-------------------------------- | ----------------------- | ---
VCC | +3.3V/+5V Power input | +5V
GND | Ground | GND
NC | - | Not used
DIN | COPI (**MOSI** - Master out, slave in) | D11
CLK | SCK (**SCLK** - Clock input) | D13
CS | **CS** (Chip select, low active) | D10
D/C | Command signal, low level for command, high level for data | D7
RES | Reset signal, low active | D8

# Setup

To compile and run automatically on Arduino:

```bash
$ cargo run --release
```

# References

- [SSD1306 data sheet](https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf)
- [WaveShare 0.96inch OLED (B)](https://www.waveshare.com/wiki/0.96inch_OLED_%28B%29#User_Guides_for_Arduino)
- [SSD1306 Rust driver](https://github.com/jamwaffles/ssd1306)
- [Huffman algorithm](https://en.wikipedia.org/wiki/Huffman_coding)
- [Text File Compression And Decompression Using Huffman Coding](https://www.geeksforgeeks.org/text-file-compression-and-decompression-using-huffman-coding/)
