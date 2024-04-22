# About

Project uses two great libraries: [avr-hal](https://github.com/Rahix/avr-hal) and [ssd1306 driver](https://github.com/jamwaffles/ssd1306).

Alternative naming conventions for SPI:

- **SCLK** : SCK, CLK.
- **MOSI** : SIMO, SDO, DO, DOUT, SO, MTSR.
- **MISO** : SOMI, SDI, DI, DIN, SI, MRST.
- **SS** : nCS, CS, CSB, CSN, nSS, STE, SYNC.

# Setup

To compile and run automatically on Arduino enter:

```bash
$ cargo run --release
```
