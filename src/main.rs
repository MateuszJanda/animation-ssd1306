#![no_std]
#![no_main]

use arduino_hal::spi;
use arduino_hal::Delay;
// use embedded_hal::blocking::delay::DelayMs ;
// use embedded_hal::blocking::delay::Delay ;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder, Rectangle, Triangle},
};

use panic_halt as _;
use ssd1306::{prelude::*, Ssd1306};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Create SPI interface.
    let (mut spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    // let mut led = pins.d13.into_output();
    let mut dc_pin = pins.d7.into_output();
    // let mut cs_pin = pins.d7.into_output();

    let interface = SPIInterfaceNoCS::new(spi, dc_pin);
    // let interface = SPIInterface::new(spi, dc_pin, cs_pin);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    let mut rst_pin = pins.d8.into_output();

    let mut delay = Delay::new();
    display.reset(&mut rst_pin, &mut delay).unwrap();
    display.init().unwrap();

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    // screen outline
    // default display size is 128x64 if you don't pass a _DisplaySize_
    // enum to the _Builder_ struct
    Rectangle::new(Point::new(0, 0), Size::new(127, 63))
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

    loop {
        // led.toggle();
        // arduino_hal::delay_ms(250);
    }
}
