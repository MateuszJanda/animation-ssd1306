#![no_std]
#![no_main]

use animation_ssd1306::driver_wrapper::MinBufferMode;
use animation_ssd1306::driver_wrapper::Ssd1306DriverWrapper;
use animation_ssd1306::encoded_frames::*;
use arduino_hal::spi;
use arduino_hal::Delay;
use avr_progmem::wrapper::ProgMem;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use panic_halt as _;
use ssd1306::{prelude::*, Ssd1306};

const ARRAY_CHUNK_SIZE: usize = 128;

/// Huffman code decoder.
struct HuffmanFrameDecoder<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    display: Ssd1306DriverWrapper<DI, SIZE>,
}

impl<DI, SIZE> HuffmanFrameDecoder<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    /// Create instance of Huffman code decoder.
    pub fn new(display: Ssd1306DriverWrapper<DI, SIZE>) -> Self {
        Self { display }
    }

    /// Decode frame_chunk and draw it on display.
    pub fn decode(&mut self, frame_bits_size: usize, frame_array: &ProgMem<[u8; 384]>) {
        let mut bt_chunk_start = 0;
        let mut bt_chunk = BINARY_TREE_LEAFS.load_sub_array::<ARRAY_CHUNK_SIZE>(bt_chunk_start);

        let mut frame_chunk_start = 0;
        let mut frame_chunk = frame_array.load_sub_array::<ARRAY_CHUNK_SIZE>(frame_chunk_start);

        let mut frame_bit_index = 1;
        let mut huffman_code = 0;

        let mut buffer = [0; ARRAY_CHUNK_SIZE];
        let mut buffer_byte_count = 0;

        for i in 0..frame_bits_size {
            let frame_byte_index = i / 8;
            // If frame byte is not in current chunk, read proper one.
            if frame_byte_index - frame_chunk_start >= ARRAY_CHUNK_SIZE {
                frame_chunk_start = (frame_byte_index / ARRAY_CHUNK_SIZE) * ARRAY_CHUNK_SIZE;
                frame_chunk = frame_array.load_sub_array::<ARRAY_CHUNK_SIZE>(frame_chunk_start);
            }

            let frame_chunk_byte_index = frame_byte_index % ARRAY_CHUNK_SIZE;
            let frame_chunk_bit_index = i % 8;

            // For "1" choose right branch, for "0" choose left branch.
            if frame_chunk[frame_chunk_byte_index] & (0b1000_0000 >> frame_chunk_bit_index) != 0 {
                frame_bit_index = 2 * frame_bit_index + 1;
                huffman_code = huffman_code << 1 | 1;
            } else {
                frame_bit_index = 2 * frame_bit_index;
                huffman_code = huffman_code << 1;
            }

            let bt_byte_index = frame_bit_index / 8;
            // If binary tree (bt) byte in not in current chunk, read proper one.
            if bt_byte_index < bt_chunk_start || bt_byte_index - bt_chunk_start >= ARRAY_CHUNK_SIZE
            {
                bt_chunk_start = (bt_byte_index / ARRAY_CHUNK_SIZE) * ARRAY_CHUNK_SIZE;
                bt_chunk = BINARY_TREE_LEAFS.load_sub_array::<ARRAY_CHUNK_SIZE>(bt_chunk_start);
            }

            let bt_chunk_byte_index = bt_byte_index % ARRAY_CHUNK_SIZE;
            let bt_chunk_bit_index = frame_bit_index % 8;

            // Check if is a leaf (marked as bit 1) in binary tree.
            if bt_chunk[bt_chunk_byte_index] & (0b1000_0000 >> bt_chunk_bit_index) != 0 {
                let mut lo: usize = 0;
                let mut hi: usize = BINARY_TREE_CODES.len() - 1;
                let mut mi: usize = (hi - lo) / 2 + lo;

                while lo <= hi {
                    mi = (hi - lo) / 2 + lo;
                    if huffman_code == BINARY_TREE_CODES[mi] as usize {
                        break;
                    } else if huffman_code < BINARY_TREE_CODES[mi] as usize {
                        hi = mi - 1;
                    } else {
                        lo = mi + 1;
                    }
                }

                buffer[buffer_byte_count] = BINARY_TREE_VALUES[mi];
                frame_bit_index = 1;
                huffman_code = 0;
                buffer_byte_count += 1;

                // Flush buffer if is full
                if buffer_byte_count == buffer.len() {
                    self.display.draw_strips_from_buffer(&buffer).unwrap();
                    buffer_byte_count = 0;
                }
            }
        }
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Init SPI.").unwrap();

    let settings = spi::Settings {
        data_order: spi::DataOrder::MostSignificantFirst,
        clock: spi::SerialClockRate::OscfOver4,
        mode: embedded_hal::spi::MODE_3,
    };

    // Create SPI spi_interface.
    let (spi, cs_pin) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        settings,
    );

    ufmt::uwriteln!(&mut serial, "Init SSD1306 driver.").unwrap();
    // Init SSD1306 driver (wrapper).
    let dc_pin = pins.d7.into_output();
    let mut rst_pin = pins.d8.into_output();
    let mut delay = Delay::new();

    let spi_interface = SPIInterface::new(spi, dc_pin, cs_pin);
    let mode = MinBufferMode::new();
    let mut display = Ssd1306DriverWrapper::new(
        Ssd1306::new(spi_interface, DisplaySize128x64, DisplayRotation::Rotate180),
        mode,
    );

    // Set reset pin
    display.reset(&mut rst_pin, &mut delay).unwrap();

    // Set AddrMode::Horizontal
    display.init().unwrap();
    // Clear screen
    display.clear(BinaryColor::Off).unwrap();
    // New wrapper method. Setup drawing area.
    display.setup().unwrap();

    ufmt::uwriteln!(&mut serial, "Run animation.").unwrap();
    let mut huffman_frame_decoder = HuffmanFrameDecoder::new(display);
    for index in (0..=29).into_iter().cycle() {
        let (frame_bits_size, frame_array) = match index {
            0 => (SKULL_FRAME00_BITS_SIZE.load(), &SKULL_FRAME00),
            1 => (SKULL_FRAME01_BITS_SIZE.load(), &SKULL_FRAME01),
            2 => (SKULL_FRAME02_BITS_SIZE.load(), &SKULL_FRAME02),
            3 => (SKULL_FRAME03_BITS_SIZE.load(), &SKULL_FRAME03),
            4 => (SKULL_FRAME04_BITS_SIZE.load(), &SKULL_FRAME04),
            5 => (SKULL_FRAME05_BITS_SIZE.load(), &SKULL_FRAME05),
            6 => (SKULL_FRAME06_BITS_SIZE.load(), &SKULL_FRAME06),
            7 => (SKULL_FRAME07_BITS_SIZE.load(), &SKULL_FRAME07),
            8 => (SKULL_FRAME08_BITS_SIZE.load(), &SKULL_FRAME08),
            9 => (SKULL_FRAME09_BITS_SIZE.load(), &SKULL_FRAME09),
            10 => (SKULL_FRAME10_BITS_SIZE.load(), &SKULL_FRAME10),
            11 => (SKULL_FRAME11_BITS_SIZE.load(), &SKULL_FRAME11),
            12 => (SKULL_FRAME12_BITS_SIZE.load(), &SKULL_FRAME12),
            13 => (SKULL_FRAME13_BITS_SIZE.load(), &SKULL_FRAME13),
            14 => (SKULL_FRAME14_BITS_SIZE.load(), &SKULL_FRAME14),
            15 => (SKULL_FRAME15_BITS_SIZE.load(), &SKULL_FRAME15),
            16 => (SKULL_FRAME16_BITS_SIZE.load(), &SKULL_FRAME16),
            17 => (SKULL_FRAME17_BITS_SIZE.load(), &SKULL_FRAME17),
            18 => (SKULL_FRAME18_BITS_SIZE.load(), &SKULL_FRAME18),
            19 => (SKULL_FRAME19_BITS_SIZE.load(), &SKULL_FRAME19),
            20 => (SKULL_FRAME20_BITS_SIZE.load(), &SKULL_FRAME20),
            21 => (SKULL_FRAME21_BITS_SIZE.load(), &SKULL_FRAME21),
            22 => (SKULL_FRAME22_BITS_SIZE.load(), &SKULL_FRAME22),
            23 => (SKULL_FRAME23_BITS_SIZE.load(), &SKULL_FRAME23),
            24 => (SKULL_FRAME24_BITS_SIZE.load(), &SKULL_FRAME24),
            25 => (SKULL_FRAME25_BITS_SIZE.load(), &SKULL_FRAME25),
            26 => (SKULL_FRAME26_BITS_SIZE.load(), &SKULL_FRAME26),
            27 => (SKULL_FRAME27_BITS_SIZE.load(), &SKULL_FRAME27),
            28 => (SKULL_FRAME28_BITS_SIZE.load(), &SKULL_FRAME28),
            29 => (SKULL_FRAME29_BITS_SIZE.load(), &SKULL_FRAME29),
            _ => (SKULL_FRAME00_BITS_SIZE.load(), &SKULL_FRAME00),
        };

        huffman_frame_decoder.decode(frame_bits_size, frame_array);
        arduino_hal::delay_ms(1);
    }

    // Should be unreachable huffman_code if application work correct.
    loop {
        ufmt::uwriteln!(&mut serial, "Ping.").unwrap();
        arduino_hal::delay_ms(500);
    }
}
