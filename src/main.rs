#![no_std]
#![no_main]

use arduino_hal::spi;
use arduino_hal::Delay;
// use embedded_hal::blocking::delay::DelayMs ;
// use embedded_hal::blocking::delay::Delay ;

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

    // let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    // let mut serial = arduino_uno::Serial::new(
    //     dp.USART0,
    //     pins.d0,
    //     pins.d1.into_output(&mut pins.ddr),
    //     57600.into_baudrate(),
    // );

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "BUKA").unwrap();

    // Create SPI interface.
    let (spi, cs_pin) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default(),
    );

    // let mut led = pins.d13.into_output();
    let dc_pin = pins.d7.into_output();
    let mut rst_pin = pins.d8.into_output();
    let mut delay = Delay::new();

    // let interface = SPIInterfaceNoCS::new(spi, dc_pin);
    let interface = SPIInterface::new(spi, dc_pin, cs_pin);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.reset(&mut rst_pin, &mut delay).unwrap();
    display.init().unwrap();

    display.set_pixel(19, 19, true);

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
        // led.toggle();
        // arduino_hal::delay_ms(250);
    }
}
