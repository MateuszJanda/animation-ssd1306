#![no_std]
#![no_main]

use core::fmt::Write;

use arduino_hal::hal::port::PB0;
use arduino_hal::port::mode;
use arduino_hal::port::Pin;
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

    // let mut led = pins.d13.into_output();
    let dc_pin = pins.d7.into_output();
    let mut rst_pin = pins.d8.into_output();
    let mut delay = Delay::new();

    // let interface = SPIInterfaceNoCS::new(spi, dc_pin);
    let interface = SPIInterface::new(spi, dc_pin, cs_pin);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    // let mut display =
    //     Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();

    display.reset(&mut rst_pin, &mut delay).unwrap();
    arduino_hal::delay_ms(100);
    // reset(&mut rst_pin);
    // init_reg(&mut display);

    // #define SSD1306_DISPLAYOFF 0xAE
    // display.interface.send_commands(0xAE).unwrap();

    display.init().unwrap();
    // let _ = display.clear();

    // display.write_str("asdf");
    /* Endless loop */
    // loop {
    //     for c in 97..123 {
    //         let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
    //     }
    //     for c in 65..91 {
    //         let _ = display.write_str(unsafe { core::str::from_utf8_unchecked(&[c]) });
    //     }
    // }

    // display.init().unwrap();
    // display.clear(BinaryColor::On).unwrap();

    // display.set_pixel(19, 19, true);

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
        arduino_hal::delay_ms(500);
        ufmt::uwriteln!(&mut serial, "Ping").unwrap();
    }
}

// fn reset(rst: &mut Pin<mode::Output, PB0>) {
//     rst.set_high();
//     arduino_hal::delay_ms(100);
//     rst.set_low();
//     arduino_hal::delay_ms(100);
//     rst.set_high();
//     arduino_hal::delay_ms(100);
// }
