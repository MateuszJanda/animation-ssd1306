#![no_std]
#![no_main]

use animation_ssd1306::raw_image::DATA;
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
use animation_ssd1306::raw_image::SKULL_FRAME;
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
    // let mut ppp = |text: &str| -> () { ufmt::uwriteln!(&mut serial, "{}", text).unwrap() };
    let mut ppp2 = |num: u32| -> () { ufmt::uwriteln!(&mut serial, "{}", num).unwrap() };
    // let mut ppp = || -> () { 1337; };

    let mode = NonBufferedMode::new(&mut ppp2);
    let mut display = MyType::new(
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate180),
        mode,
    );

    display.reset(&mut rst_pin, &mut delay).unwrap();
    display.init().unwrap();

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();
    match Rectangle::new(Point::new(1, 1), Size::new_equal(5))
        .into_styled(style)
        .draw(&mut display)
    {
        Ok(_) => ufmt::uwriteln!(&mut serial, "BUKA rectangle success.").unwrap(),
        Err(_) => ufmt::uwriteln!(&mut serial, "BUKA rectangle fail.").unwrap(),
    }
    ufmt::uwriteln!(&mut serial, "BUKA rectangle.").unwrap();

    // let raw_image = ImageRaw::<BinaryColor>::new(SKULL_FRAME, 128);
    // let image = Image::new(&raw_image, Point::zero());
    // match image.draw(&mut display) {
    //     Ok(_) => ufmt::uwriteln!(&mut serial, "BUKA skull success.").unwrap(),
    //     Err(_) => ufmt::uwriteln!(&mut serial, "BUKA skull fail.").unwrap(),
    // }
    // ufmt::uwriteln!(&mut serial, "BUKA skull.").unwrap();

    loop {
        ufmt::uwriteln!(&mut serial, "Ping.").unwrap();
        arduino_hal::delay_ms(500);
    }
}

// fn print_type_of<T>(_: &T) {
//     ufmt::uwriteln!("{}", std::any::type_name::<T>())
// }
