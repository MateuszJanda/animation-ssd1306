#![no_std]
#![no_main]

// use animation_ssd1306::raw_image::DATA;
use animation_ssd1306::raw_image::*;
// use animation_ssd1306::raw_image::SKULL_FRAME01;
use arduino_hal::spi;
use arduino_hal::Delay;

use avr_progmem::wrapper::ProgMem;
use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

use animation_ssd1306::non_buffered_mode::MyType;
use animation_ssd1306::non_buffered_mode::NonBufferedMode;
use animation_ssd1306::raw_image::*;
use animation_ssd1306::*;
use panic_halt as _;
use ssd1306::mode::BasicMode;
use ssd1306::{prelude::*, Ssd1306};

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

    // Create SPI interface.
    let (spi, cs_pin) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        settings,
    );

    // Init SSD1306 driver.
    let dc_pin = pins.d7.into_output();
    let mut rst_pin = pins.d8.into_output();
    let mut delay = Delay::new();

    let interface = SPIInterface::new(spi, dc_pin, cs_pin);
    // let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180)
    //     .into_buffered_graphics_mode();

    // let mut display = Ssd1306::<_, _, BasicMode>::new(
    //     interface,
    //     DisplaySize128x64,
    //     DisplayRotation::Rotate180,
    // );

    // let a =         Ssd1306 {
    //     interface: interface,
    //     mode: mode,
    //     size: DisplaySize128x64,
    //     addr_mode: AddrMode::Page,
    //     rotation:DisplayRotation::Rotate180
    // };

    // let mode = NonBufferedMode::new();
    // let mut display = MyType(Ssd1306::new(
    //     interface,
    //     DisplaySize128x64,
    //     DisplayRotation::Rotate180,
    // )).into_mode(mode);

    // let () = serial;
    // let mut print_str = |text: &str| -> () { ufmt::uwriteln!(&mut serial, "{}", text).unwrap() };
    // let mut print_debug =
    //     |text: &str, num: i32| -> () { ufmt::uwriteln!(&mut serial, "{} {}", text, num).unwrap() };
    let mut print_debug = |text: &str, num: i32| -> () {};
    // let mut print_str = || -> () { 1337; };

    let mode = NonBufferedMode::new(&mut print_debug);
    let mut display = MyType::new(
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180),
        mode,
    );

    display.reset(&mut rst_pin, &mut delay).unwrap();
    display.init().unwrap();

    display.clear(BinaryColor::Off).unwrap();

    // ufmt::uwriteln!(&mut serial, "After init").unwrap();

    // let style = PrimitiveStyleBuilder::new()
    //     .stroke_width(1)
    //     .stroke_color(BinaryColor::On)
    //     .build();
    // match Rectangle::new(Point::new(1, 1), Size::new_equal(50))
    //     .into_styled(style)
    //     .draw(&mut display)
    // {
    //     // Ok(_) => ufmt::uwriteln!(&mut serial, "BUKA rectangle success.").unwrap(),
    //     Ok(_) => (),
    //     // Err(_) => ufmt::uwriteln!(&mut serial, "BUKA rectangle fail.").unwrap(),
    //     Err(_) => (),
    // }
    // display.flush2().unwrap();
    // ufmt::uwriteln!(&mut serial, "BUKA rectangle.").unwrap();

    // let raw_image = ImageRaw::<BinaryColor>::new(SKULL_FRAME, 128);
    // let image = Image::new(&raw_image, Point::zero());
    // match image.draw(&mut display) {
    //     Ok(_) => ufmt::uwriteln!(&mut serial, "BUKA skull success.").unwrap(),
    //     Err(_) => ufmt::uwriteln!(&mut serial, "BUKA skull fail.").unwrap(),
    // }
    // ufmt::uwriteln!(&mut serial, "BUKA skull.").unwrap();

    display.setup().unwrap();

    // let v = [
    //     &SKULL_FRAME00,
    //     &SKULL_FRAME01,
    //     &SKULL_FRAME02,
    //     // &SKULL_FRAME03,
    //     // &SKULL_FRAME04,
    //     // &SKULL_FRAME05,
    //     // &SKULL_FRAME06,
    //     // &SKULL_FRAME07,
    //     // &SKULL_FRAME08,
    //     // &SKULL_FRAME09,
    //     // &SKULL_FRAME10,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    //     // &SKULL_FRAME00,
    // ];

    /*
        for index in (0..=23).into_iter().cycle() {
            // let aa = v[index];

            // match index {
            //         0 => display.draw_strips_from_buffer(&SKULL_FRAME00).unwrap(),
            //         1 => display.draw_strips_from_buffer(&SKULL_FRAME01).unwrap(),
            //         _ => (),
            // };

            let v= match index {
                0 => &SKULL_FRAME00,
                1 => &SKULL_FRAME01,
                2 => &SKULL_FRAME02,
                3 => &SKULL_FRAME03,
                4 => &SKULL_FRAME04,
                5 => &SKULL_FRAME05,
                6 => &SKULL_FRAME06,
                7 => &SKULL_FRAME07,
                8 => &SKULL_FRAME08,
                9 => &SKULL_FRAME09,

                10 => &SKULL_FRAME10,
                11 => &SKULL_FRAME11,
                12 => &SKULL_FRAME12,
                13 => &SKULL_FRAME13,
                14 => &SKULL_FRAME14,
                15 => &SKULL_FRAME15,
                16 => &SKULL_FRAME16,
                17 => &SKULL_FRAME17,
                18 => &SKULL_FRAME18,
                19 => &SKULL_FRAME19,

                20 => &SKULL_FRAME20,
                21 => &SKULL_FRAME21,
                22 => &SKULL_FRAME22,
                23 => &SKULL_FRAME23,
                // 24 => &SKULL_FRAME24,
                // 25 => &SKULL_FRAME25,
                // 26 => &SKULL_FRAME26,
                // 27 => &SKULL_FRAME27,
                // 28 => &SKULL_FRAME28,
                // 29 => &SKULL_FRAME29,

                _ => &SKULL_FRAME00,
            };

            display.setup().unwrap();
            display.draw_strips_from_buffer(&v.load_sub_array::<128>(0)).unwrap();
            display.draw_strips_from_buffer(&v.load_sub_array::<128>(128)).unwrap();
            display.draw_strips_from_buffer(&v.load_sub_array::<128>(256)).unwrap();
            display.draw_strips_from_buffer(&v.load_sub_array::<128>(384)).unwrap();
            display.draw_strips_from_buffer(&v.load_sub_array::<128>(512)).unwrap();
            display.draw_strips_from_buffer(&v.load_sub_array::<128>(640)).unwrap();
            display.draw_strips_from_buffer(&v.load_sub_array::<128>(896)).unwrap();

            // ufmt::uwriteln!(&mut serial, "Ping.").unwrap();
            arduino_hal::delay_ms(50);
        }
    */

    //     // display.draw_strips_from_buffer(aa).unwrap();
    //     // display.draw_strips_from_buffer(&SKULL_FRAME[index]).unwrap();
    // }

    // let data: &[u8] = &SKULL_FRAME00.load();
    // display.draw_strips_from_buffer(&SKULL_FRAME00.load_sub_array::<128>(0)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME00.load_sub_array::<128>(128)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME00.load_sub_array::<128>(256)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME00.load_sub_array::<128>(384)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME00.load_sub_array::<128>(512)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME00.load_sub_array::<128>(640)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME00.load_sub_array::<128>(896)).unwrap();
    // // let data = &SKULL_FRAME00.load_sub_array::<128>(1024);
    // // display.draw_strips_from_buffer(data).unwrap();

    // display.setup().unwrap();

    // display.draw_strips_from_buffer(&SKULL_FRAME01.load_sub_array::<128>(0)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME01.load_sub_array::<128>(128)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME01.load_sub_array::<128>(256)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME01.load_sub_array::<128>(384)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME01.load_sub_array::<128>(512)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME01.load_sub_array::<128>(640)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME01.load_sub_array::<128>(896)).unwrap();

    // display.setup().unwrap();
    let mut frame_decoder = FrameDecoder::new(display);
    frame_decoder.setup();
    frame_decoder.decode(SKULL_FRAME02_BITS_SIZE.load(), &SKULL_FRAME02);

    for index in (0..=29).into_iter().cycle() {
        // let aa = v[index];

        // match index {
        //         0 => display.draw_strips_from_buffer(&SKULL_FRAME00).unwrap(),
        //         1 => display.draw_strips_from_buffer(&SKULL_FRAME01).unwrap(),
        //         _ => (),
        // };

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

        // display.setup().unwrap();

        // TODO: Is needed??
        // frame_decoder.setup();
        frame_decoder.decode(frame_bits_size, frame_array);
        // display.draw_strips_from_buffer(&v.load_sub_array::<128>(0)).unwrap();
        // display.draw_strips_from_buffer(&v.load_sub_array::<128>(128)).unwrap();
        // display.draw_strips_from_buffer(&v.load_sub_array::<128>(256)).unwrap();
        // display.draw_strips_from_buffer(&v.load_sub_array::<128>(384)).unwrap();
        // display.draw_strips_from_buffer(&v.load_sub_array::<128>(512)).unwrap();
        // display.draw_strips_from_buffer(&v.load_sub_array::<128>(640)).unwrap();
        // display.draw_strips_from_buffer(&v.load_sub_array::<128>(896)).unwrap();

        // ufmt::uwriteln!(&mut serial, "Ping.").unwrap();
        arduino_hal::delay_ms(1);
    }

    // --- BUKA1 start

    // display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(0)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(128)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(256)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(384)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(512)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(640)).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(896)).unwrap();

    // --- BUKA1 end

    /*
        let frame_bits_size = SKULL_FRAME02_BITS_SIZE.load();
        // let x = BINARY_TREE_LEAFS.load_sub_array::<128>(0);
        let mut bt_start = 0;
        let mut bt: [u8; 128] = BINARY_TREE_LEAFS.load_sub_array::<128>(bt_start);

        let mut frame_start = 0;
        let mut frame: [u8; 128] = SKULL_FRAME02.load_sub_array::<128>(frame_start);
        let mut current_index = 1;
        let mut current_code = 0;

        // let mut value_start = 0;
        // let mut value: [u8; 128] = BINARY_TREE_INDEXES_TO_VALUES.load_sub_array::<128>(value_start);

        let mut buf = [0; 128];
        let mut buf_i = 0;
        for i in 0..frame_bits_size {
            // ufmt::uwriteln!(&mut serial, "BUKA i: {}, buf_i: {}, current_index: {}", i, buf_i, current_index).unwrap();

            let frame_byte = i / 8;
            // let frame_bit = i % 8;

            if frame_byte - frame_start >= 128 {
                frame_start = (frame_byte / 128) * 128;
                frame = SKULL_FRAME02.load_sub_array::<128>(frame_start);
            }

            let sub_frame_byte = frame_byte % 128;
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

            if bt_byte < bt_start || bt_byte - bt_start >= 128 {
                bt_start = (bt_byte / 128) * 128;
                bt = BINARY_TREE_LEAFS.load_sub_array::<128>(bt_start);
            }
            let sub_bt_byte = bt_byte % 128;
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
                    display.draw_strips_from_buffer(&buf).unwrap();
                    buf_i = 0;
                }
            }
        }

    */

    // let data = &SKULL_FRAME00.load_sub_array::<128>(0);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME00.load_sub_array::<128>(128);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME00.load_sub_array::<128>(256);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME00.load_sub_array::<128>(384);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME00.load_sub_array::<128>(512);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME00.load_sub_array::<128>(640);
    // display.draw_strips_from_buffer(data).unwrap();

    // let data = &SKULL_FRAME01.load_sub_array::<128>(0);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME01.load_sub_array::<128>(128);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME01.load_sub_array::<128>(256);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME01.load_sub_array::<128>(384);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME01.load_sub_array::<128>(512);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME01.load_sub_array::<128>(640);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME01.load_sub_array::<128>(768);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME01.load_sub_array::<128>(896);
    // display.draw_strips_from_buffer(data).unwrap();
    // let data = &SKULL_FRAME01.load_sub_array::<128>(1024);
    // display.draw_strips_from_buffer(data).unwrap();

    // display.draw_strips_from_buffer(&SKULL_FRAME01).unwrap();
    // display.draw_strips_from_buffer(&SKULL_FRAME00).unwrap();

    loop {
        ufmt::uwriteln!(&mut serial, "Ping.").unwrap();
        arduino_hal::delay_ms(500);
    }
}

struct FrameDecoder<'a, DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    display: MyType<'a, DI, SIZE>,
}

impl<'a, DI, SIZE> FrameDecoder<'a, DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    pub fn new(display: MyType<'a, DI, SIZE>) -> Self {
        Self { display }
    }

    pub fn setup(&mut self) {
        self.display.setup().unwrap();
    }

    pub fn decode(&mut self, frame_bits_size: usize, frame_array: &ProgMem<[u8; 384]>) {
        // let frame_bits_size = SKULL_FRAME02_BITS_SIZE.load();
        // let x = BINARY_TREE_LEAFS.load_sub_array::<128>(0);
        let mut bt_start = 0;
        let mut bt: [u8; 128] = BINARY_TREE_LEAFS.load_sub_array::<128>(bt_start);

        let mut frame_start = 0;
        let mut frame: [u8; 128] = frame_array.load_sub_array::<128>(frame_start);
        let mut current_index = 1;
        let mut current_code = 0;

        // let mut value_start = 0;
        // let mut value: [u8; 128] = BINARY_TREE_INDEXES_TO_VALUES.load_sub_array::<128>(value_start);

        let mut buf = [0; 128];
        let mut buf_i = 0;
        for i in 0..frame_bits_size {
            // ufmt::uwriteln!(&mut serial, "BUKA i: {}, buf_i: {}, current_index: {}", i, buf_i, current_index).unwrap();

            let frame_byte = i / 8;
            // let frame_bit = i % 8;

            if frame_byte - frame_start >= 128 {
                frame_start = (frame_byte / 128) * 128;
                frame = frame_array.load_sub_array::<128>(frame_start);
            }

            let sub_frame_byte = frame_byte % 128;
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

            if bt_byte < bt_start || bt_byte - bt_start >= 128 {
                bt_start = (bt_byte / 128) * 128;
                bt = BINARY_TREE_LEAFS.load_sub_array::<128>(bt_start);
            }
            let sub_bt_byte = bt_byte % 128;
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

// fn print_type_of<T>(_: &T) {
//     ufmt::uwriteln!("{}", std::any::type_name::<T>())
// }
