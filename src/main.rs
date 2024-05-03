#![no_std]
#![no_main]

// use animation_ssd1306::raw_image::DATA;
use animation_ssd1306::raw_image::*;
// use animation_ssd1306::raw_image::SKULL_FRAME01;
use arduino_hal::spi;
use arduino_hal::Delay;

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
    let mut print_debug =
        |text: &str, num: i32| -> () { ufmt::uwriteln!(&mut serial, "{} {}", text, num).unwrap() };
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

    display.setup().unwrap();

    display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(0)).unwrap();
    display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(128)).unwrap();
    display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(256)).unwrap();
    display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(384)).unwrap();
    display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(512)).unwrap();
    display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(640)).unwrap();
    display.draw_strips_from_buffer(&SKULL_FRAME02.load_sub_array::<128>(896)).unwrap();

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



// fn print_type_of<T>(_: &T) {
//     ufmt::uwriteln!("{}", std::any::type_name::<T>())
// }
