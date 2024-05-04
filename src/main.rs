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
    pub fn new(display: Ssd1306DriverWrapper<DI, SIZE>) -> Self {
        Self { display }
    }

    pub fn decode(&mut self, frame_bits_size: usize, frame_array: &ProgMem<[u8; 384]>) {
        // let frame_bits_size = SKULL_FRAME02_BITS_SIZE.load();
        // let x = BINARY_TREE_LEAFS.load_sub_array::<128>(0);
        let mut bt_start = 0;
        let mut bt = BINARY_TREE_LEAFS.load_sub_array::<ARRAY_CHUNK_SIZE>(bt_start);

        let mut frame_start = 0;
        let mut frame = frame_array.load_sub_array::<ARRAY_CHUNK_SIZE>(frame_start);
        let mut current_index = 1;
        let mut current_code = 0;

        // let mut value_start = 0;
        // let mut value: [u8; 128] = BINARY_TREE_INDEXES_TO_VALUES.load_sub_array::<128>(value_start);

        let mut buf = [0; ARRAY_CHUNK_SIZE];
        let mut buf_i = 0;
        for i in 0..frame_bits_size {
            // ufmt::uwriteln!(&mut serial, "BUKA i: {}, buf_i: {}, current_index: {}", i, buf_i, current_index).unwrap();

            let frame_byte = i / 8;
            // let frame_bit = i % 8;

            if frame_byte - frame_start >= ARRAY_CHUNK_SIZE {
                frame_start = (frame_byte / ARRAY_CHUNK_SIZE) * ARRAY_CHUNK_SIZE;
                frame = frame_array.load_sub_array::<ARRAY_CHUNK_SIZE>(frame_start);
            }

            let sub_frame_byte = frame_byte % ARRAY_CHUNK_SIZE;
            let sub_frame_bit = i % 8;

            if frame[sub_frame_byte] & (0b1000_0000 >> sub_frame_bit) != 0 {
                current_index = 2 * current_index + 1;
                current_code = current_code << 1 | 1;
            } else {
                current_index = 2 * current_index;
                current_code = current_code << 1;
            }
            // ufmt::uwriteln!(&mut serial, "BUKA current_index {}", current_index).unwrap();
            // ufmt::uwriteln!(
            //     &mut serial,
            //     "BUKA i: {}, buf_i: {}, current_index: {}, current_code: {}, bit: {}",
            //     i,
            //     buf_i,
            //     current_index,
            //     current_code,
            //     frame[sub_frame_byte] & (0b1000_0000 >> sub_frame_bit) != 0
            // )
            // .unwrap();

            let bt_byte = current_index / 8;

            // ufmt::uwriteln!(
            //     &mut serial,
            //     "BUKA b bt_byte:{} bt_start:{}",
            //     bt_byte,
            //     bt_start
            // )
            // .unwrap();
            // serial.flush();

            if bt_byte < bt_start || bt_byte - bt_start >= ARRAY_CHUNK_SIZE {
                bt_start = (bt_byte / ARRAY_CHUNK_SIZE) * ARRAY_CHUNK_SIZE;
                bt = BINARY_TREE_LEAFS.load_sub_array::<ARRAY_CHUNK_SIZE>(bt_start);
            }
            let sub_bt_byte = bt_byte % ARRAY_CHUNK_SIZE;
            let sub_bt_bit = current_index % 8;

            if bt[sub_bt_byte] & (0b1000_0000 >> sub_bt_bit) != 0 {
                // is leaf

                let mut lo: usize = 0;
                let mut hi: usize = BINARY_TREE_CODES.len() - 1;
                let mut mi: usize = (hi - lo) / 2 + lo;

                // let search_code = current_index - 2;
                // let search_code = current_code;
                // ufmt::uwriteln!(&mut serial, "BUKA current_code 0x{:04x}", current_code).unwrap();
                while lo <= hi {
                    mi = (hi - lo) / 2 + lo;
                    // ufmt::uwriteln!(&mut serial, "BUKA  mi: {}", mi).unwrap();

                    if current_code == BINARY_TREE_CODES[mi] as usize {
                        // ufmt::uwriteln!(&mut serial, "BUKA break mi: {}", mi).unwrap();
                        break;
                    } else if current_code < BINARY_TREE_CODES[mi] as usize {
                        hi = mi - 1;
                    } else {
                        lo = mi + 1;
                    }
                }
                // ufmt::uwriteln!(&mut serial, "BUKA  mi: {}", mi).unwrap();

                // let mut mi_byte: usize = mi;
                // if mi_byte < value_start || mi_byte - value_start >= 128 {
                //     value_start = (mi_byte / 128) * 128;
                //     value = BINARY_TREE_INDEXES_TO_VALUES.load_sub_array::<128>(value_start);
                //     mi_byte = mi_byte % 128;
                // }

                buf[buf_i] = BINARY_TREE_VALUES[mi];
                // ufmt::uwriteln!(&mut serial, "BUKA mi {}, value 0x{:x} ", mi, buf[buf_i]).unwrap();
                current_index = 1;
                current_code = 0;

                // let mi_byte =BINARY_TREE_INDEXES_TO_VALUES

                buf_i += 1;
                if buf_i == buf.len() {
                    self.display.draw_strips_from_buffer(&buf).unwrap();
                    buf_i = 0;
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

    // Should be unreachable code if application work correct.
    loop {
        ufmt::uwriteln!(&mut serial, "Ping.").unwrap();
        arduino_hal::delay_ms(500);
    }
}
