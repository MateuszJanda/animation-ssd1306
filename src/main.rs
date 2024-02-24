#![no_std]
#![no_main]

use arduino_hal::spi;
use arduino_hal::Delay;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyleBuilder, Rectangle},
};

use panic_halt as _;
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
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.reset(&mut rst_pin, &mut delay).unwrap();
    display.init().unwrap();

    // Print rectangle
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    let yoffset = 20;
    Rectangle::new(Point::new(52, yoffset), Size::new_equal(16))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    display.flush().unwrap();

    loop {
        arduino_hal::delay_ms(500);
        ufmt::uwriteln!(&mut serial, "Ping.").unwrap();
    }
}
