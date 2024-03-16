//! Non buffered graphics mode.

use display_interface::DisplayError;
use ssd1306::{
    command::AddrMode, mode::BasicMode, mode::DisplayConfig, rotation::DisplayRotation,
    size::DisplaySize, Ssd1306,
};

use display_interface::DataFormat::U8;

use core::ops::{Deref, DerefMut};

use ssd1306::prelude::WriteOnlyDataCommand;

// use arduino_hal::spi;
// use arduino_hal::Delay;
// use arduino_hal::Usart;
// // use arduino_hal::Atmega;
// use arduino_hal::clock::MHz16;
// use arduino_hal::pac::USART0;
// use arduino_hal::port::mode::Input;
// use arduino_hal::port::mode::Output;
// use arduino_hal::port::Pin;

// #[derive(Clone, Debug)]
// #[derive(Clone)]
pub struct NonBufferedMode<'a>
// where
// SIZE: DisplaySize,
{
    buffer: [u8; 1],
    min_x: u8,
    max_x: u8,
    min_y: u8,
    max_y: u8,
    last_x: u8,
    last_y: u8,
    // print_str: &'a mut dyn FnMut(&str) -> (),
    print_debug: &'a mut dyn FnMut(&str, i32) -> (),
}

// impl<SIZE> NonBufferedMode<SIZE>
impl<'a> NonBufferedMode<'a>
// where
// SIZE: DisplaySize,
{
    /// Create a new buffered graphics mode instance.
    // pub fn new() -> Self {
    // pub fn new(serial :& mut arduino_hal::hal::usart::Usart<Atmega, USART0, avr_hal_generic::port::Pin<Input, PD0>, avr_hal_generic::port::Pin<Output, PD1>, MHz16>) -> Self {
    // pub fn new(serial: &mut Usart<Atmega, USART0, Pin<Input, PD0>, Pin<Output, PD1>, MHz16>) -> Self {
    // pub fn new(print_debug: &'a mut dyn FnMut(&str) -> ()) -> Self {
    pub fn new(print_debug: &'a mut dyn FnMut(&str, i32) -> ()) -> Self {
        Self {
            buffer: [0],
            min_x: 255,
            max_x: 0,
            min_y: 255,
            max_y: 0,

            last_x: u8::MAX,
            last_y: u8::MAX,

            // serial:
            // print_str,
            print_debug,
        }
    }

    // /// TODO
    // pub fn reset(&mut self, width: u8, height: u8) {
    //     self.min_x = 0;
    //     self.max_x = width - 1;
    //     self.min_y = 0;
    //     self.max_y = height - 1;
    // }
}

// impl<DI, SIZE> DisplayConfig for Ssd1306<DI, SIZE, NonBufferedMode<SIZE>>
// where
//     DI: WriteOnlyDataCommand,
//     SIZE: DisplaySize,
// {
//     type Error = DisplayError;

//     /// Set the display rotation
//     ///
//     /// This method resets the cursor but does not clear the screen.
//     fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DisplayError> {
//         self.set_rotation(rot)
//     }

//     /// Initialise and clear the display in graphics mode.
//     fn init(&mut self) -> Result<(), DisplayError> {
//         self.clear_impl(false);
//         self.init_with_addr_mode(AddrMode::Horizontal)
//     }
// }

// #[cfg(feature = "graphics")]
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::Size,
    geometry::{Dimensions, OriginDimensions},
    pixelcolor::BinaryColor,
    Pixel,
};

// pub struct MyType<DI, SIZE>(pub Ssd1306<DI, SIZE, BasicMode>);

// type Target<'a> = Ssd1306<DI, SIZE, NonBufferedMode<'a>>;
pub struct MyType<'a, DI, SIZE>(pub Ssd1306<DI, SIZE, NonBufferedMode<'a>>);
// where
//     DI: WriteOnlyDataCommand,
//     SIZE: DisplaySize;

impl<'a, DI, SIZE> MyType<'a, DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    // pub fn into_mode(self, mode: NonBufferedMode) -> Self {
    //     MyType(self.0.into_mode(mode))
    // }

    pub fn new(ssd: Ssd1306<DI, SIZE, BasicMode>, mode: NonBufferedMode<'a>) -> Self {
        MyType(ssd.into_mode(mode))
    }

    fn clear_impl(&mut self, value: bool) {
        (self.mode_mut().print_debug)("Before clear", 0);
        let (width, height) = self.dimensions();
        self.mode_mut().min_x = 0;
        self.mode_mut().min_x = 0;
        self.mode_mut().max_x = width - 1;
        self.mode_mut().min_y = 0;
        self.mode_mut().max_y = height - 1;

        // Invalidate last_x and last_y
        self.mode_mut().last_x = u8::MAX;
        self.mode_mut().last_y = u8::MAX;

        // TODO trigger clear
        // (self.mode_mut().print_debug)("clear_impl");

        // self.set_pixel(1, 1, true);

        // let offset_x = SIZE::DRIVER_COLS - SIZE::WIDTH - SIZE::OFFSETX;

        let disp_min_x = 0;
        let disp_min_y = 0;
        let disp_max_x = width;
        // let xmax = 64 + offset_x;
        let disp_max_y = height;

        (self.mode_mut().print_debug)("clear_impl disp_min_x", disp_min_x as i32);
        (self.mode_mut().print_debug)("clear_impl disp_min_y", disp_min_y as i32);
        (self.mode_mut().print_debug)("clear_impl disp_max_x", disp_max_x as i32);
        (self.mode_mut().print_debug)("clear_impl disp_max_y", disp_max_y as i32);

        self.set_draw_area((disp_min_x, disp_min_y), (disp_max_x, disp_max_y))
            .unwrap();

        let c = match value {
            true => &[0xff],
            false => &[0x00],
        };
        let num_of_bytes = (width as u32 * height as u32) / 8;
        for _ in 0..num_of_bytes {
            // let c = &[0xff];
            // let c = &[0x00];
            // let c = &[0b1010_1010];
            self.interface_mut().send_data(U8(c)).unwrap();
        }

        // self.set_draw_area((xmin + 8, ymin + 20), (xmax, ymax)).unwrap();

        // for _ in 0..112 {
        //     // let c = &[0xff];
        //     let c = &[0b1010_1010];
        //     self.interface_mut().send_data(U8(c)).unwrap();
        // }

        (self.mode_mut().print_debug)("After clear", 0);
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: bool) {
        // {
        //     (self.mode_mut().print_debug)("asdf");
        // }

        // (self.mode_mut().print_debug)("set_pixel", 0);

        if self.is_pixel_out_of_buffer(x as u8, y as u8) {
            self.flush2().unwrap();
        }

        // (self.mode_mut().print_debug)("set_pixel after flush", 0);

        let rotation = self.rotation();

        let bit_num = match rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                // let idx = ((y as usize) / 8 * SIZE::WIDTH as usize) + (x as usize);
                let bit_num = y % 8;

                bit_num
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                // let idx = ((x as usize) / 8 * SIZE::WIDTH as usize) + (y as usize);
                let bit_num = x % 8;

                bit_num
            }
        };

        // if let Some(byte) = self.mode.buffer.as_mut().get_mut(idx) {
        // Keep track of max and min values
        self.mode_mut().min_x = self.mode().min_x.min(x as u8);
        self.mode_mut().max_x = self.mode().max_x.max(x as u8);

        self.mode_mut().min_y = self.mode().min_y.min(y as u8);
        self.mode_mut().max_y = self.mode().max_y.max(y as u8);

        //     // Set pixel value in byte
        //     // Ref this comment https://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit#comment46654671_47990
        //     *byte = *byte & !(1 << bit) | (value << bit);
        // }

        let value = value as u8;
        self.mode_mut().buffer[0] =
            self.mode_mut().buffer[0] & !(1 << bit_num) | (value << bit_num);
        // *byte = *byte & !(1 << bit) | (value << bit);

        self.mode_mut().last_x = x as u8;
        self.mode_mut().last_y = y as u8;

        // Self::flush_buffer_chunks(
        //     &mut self.interface,
        //     self.mode.buffer.as_mut(),
        //     height as usize,
        //     (disp_min_y, disp_min_x),
        //     (disp_max_y, disp_max_x),
        // );

        // let c: [u8; 1] = [4];
        // TODO Result
        // self.interface_mut().send_data(U8(&c)).unwrap();

        // TODO Result

        // self.flush().unwrap();
    }

    fn is_pixel_out_of_buffer(&mut self, x: u8, y: u8) -> bool {
        if self.mode().last_x == u8::MAX || self.mode().last_y == u8::MAX {
            return false;
        }

        let last_x = self.mode_mut().last_x;
        let last_y = self.mode_mut().last_y;
        // (self.mode_mut().print_debug)("is-check x", x as i32);
        // (self.mode_mut().print_debug)("is-check y", y as i32);
        // (self.mode_mut().print_debug)("is-check last_x", last_x as i32);
        // (self.mode_mut().print_debug)("is-check last_y", last_y as i32);

        let r = match self.rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                x != self.mode().last_x || (y / 8) != (self.mode().last_y / 8)
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                (x / 8) != (self.mode().last_x / 8) || y != self.mode().last_y
            }
        };

        // (self.mode_mut().print_debug)("is-check result", r as i32);

        r
    }

    /// TODO
    pub fn setup(&mut self) -> Result<(), DisplayError> {
        let (width, height) = self.dimensions();

        let disp_min_x = 0;
        let disp_min_y = 0;
        let disp_max_x = width;
        // let xmax = 64 + offset_x;
        let disp_max_y = height;

        self.set_draw_area((disp_min_x, disp_min_y), (disp_max_x, disp_max_y))
    }

    /// TODO
    pub fn draw_strips_from_buffer(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        // let (width, height) = self.dimensions();
        // let num_of_bytes = (width as usize * height as usize) / 8;
        // for _ in 0..num_of_bytes {
        //     // let c = &[0xff];
        //     // let c = &[0x00];
        //     // let c = &[0b1010_1010];
        //     self.interface_mut().send_data(U8(c)).unwrap();
        // }

        self.interface_mut().send_data(U8(buffer))

        // buffer
        //     .chunks(1)
        //     .take(num_of_bytes)
        //     // .map(|s| &s[page_lower..page_upper])
        //     .try_for_each(|c| self.interface_mut().send_data(U8(c)))
    }

    /// TODO
    pub fn flush2(&mut self) -> Result<(), DisplayError> {
        if self.mode().last_x == u8::MAX || self.mode().last_y == u8::MAX {
            return Ok(());
        }

        (self.mode_mut().print_debug)("flush2 ", 0);

        let (disp_min_x, disp_min_y, disp_max_x, disp_max_y) = match self.rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (
                self.mode().last_x,
                (self.mode().last_y / 8) * 8,
                self.mode().last_x,
                (self.mode().last_y / 8) * 8 + 8,
            ),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (
                (self.mode().last_x / 8) * 8,
                self.mode().last_y,
                (self.mode().last_x / 8) * 8 + 8,
                self.mode().last_y,
            ),
        };

        (self.mode_mut().print_debug)("flush2 disp_min_x", disp_min_x as i32);
        (self.mode_mut().print_debug)("flush2 disp_min_y", disp_min_y as i32);
        (self.mode_mut().print_debug)("flush2 disp_max_x", disp_max_x as i32);
        (self.mode_mut().print_debug)("flush2 disp_max_y", disp_max_y as i32);

        self.set_draw_area((disp_min_x, disp_min_y), (disp_max_x, disp_max_y))
            .unwrap();
        let byte_buffer = self.mode().buffer;
        self.interface_mut().send_data(U8(&byte_buffer)).unwrap();

        // Empty byte buffer
        self.mode_mut().buffer[0] = 0x00;

        // Invalidate last_x and last_y
        self.mode_mut().last_x = u8::MAX;
        self.mode_mut().last_y = u8::MAX;

        return Ok(());
    }

    /// Write out data to a display.
    ///
    /// This only updates the parts of the display that have changed since the last flush.
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        // Nothing to do if no pixels have changed since the last update
        if self.mode().max_x < self.mode().min_x || self.mode().max_y < self.mode().min_y {
            return Ok(());
        }

        // (self.mode_mut().print_debug)("flush");

        let (width, height) = self.dimensions();

        // Determine which bytes need to be sent
        let disp_min_x = self.mode().min_x;
        let disp_min_y = self.mode().min_y;

        let (disp_max_x, disp_max_y) = match self.rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => (
                (self.mode().max_x + 1).min(width),
                (self.mode().max_y | 7).min(height),
            ),
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => (
                (self.mode().max_x | 7).min(width),
                (self.mode().max_y + 1).min(height),
            ),
        };

        // Invalidate mode area
        self.mode_mut().min_x = 255;
        self.mode_mut().max_x = 0;
        self.mode_mut().min_y = 255;
        self.mode_mut().max_y = 0;

        // Tell the display to update only the part that has changed
        let offset_x = match self.rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate270 => SIZE::OFFSETX,
            DisplayRotation::Rotate180 | DisplayRotation::Rotate90 => {
                // If segment remapping is flipped, we need to calculate
                // the offset from the other edge of the display.
                SIZE::DRIVER_COLS - SIZE::WIDTH - SIZE::OFFSETX
            }
        };

        // let ccc: [u8; 1] = [4];
        // let byte_buffer = self.mode().buffer;
        let byte_buffer = [255];

        match self.rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                self.set_draw_area(
                    (disp_min_x + offset_x, disp_min_y + SIZE::OFFSETY),
                    (disp_max_x + offset_x, disp_max_y + SIZE::OFFSETY),
                )?;

                // (self.mode_mut().print_debug)("Flush disp_min_x", disp_min_x as i32);
                // (self.mode_mut().print_debug)("Flush disp_max_x", disp_max_x as i32);
                // (self.mode_mut().print_debug)("Flush disp_min_y", disp_min_y as i32);
                // (self.mode_mut().print_debug)("Flush disp_max_y", disp_max_y as i32);

                (self.mode_mut().print_debug)("Flush start.x", (disp_min_x + offset_x) as i32);
                (self.mode_mut().print_debug)("Flush start.y", (disp_min_y + SIZE::OFFSETY) as i32);
                (self.mode_mut().print_debug)("Flush end.x", (disp_max_x + offset_x) as i32);
                (self.mode_mut().print_debug)("Flush end.y", (disp_max_y + SIZE::OFFSETY) as i32);

                // Ssd1306::<DI, SIZE, NonBufferedMode>::flush_buffer_chunks(
                //     &mut self.interface_mut(),
                //     &byte_buffer,
                //     width as usize,
                //     (disp_min_x, disp_min_y),
                //     (disp_max_x, disp_max_y),
                // )
                self.interface_mut().send_data(U8(&byte_buffer)).unwrap();

                Ok(())
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                self.set_draw_area(
                    (disp_min_y + offset_x, disp_min_x + SIZE::OFFSETY),
                    (disp_max_y + offset_x, disp_max_x + SIZE::OFFSETY),
                )?;

                Ssd1306::<DI, SIZE, NonBufferedMode>::flush_buffer_chunks(
                    &mut self.interface_mut(),
                    &byte_buffer,
                    height as usize,
                    (disp_min_y, disp_min_x),
                    (disp_max_y, disp_max_x),
                )
            }
        }
    }
}

impl<'a, DI, SIZE> DrawTarget for MyType<'a, DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Color = BinaryColor;
    type Error = DisplayError;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let bb = self.bounding_box();

        pixels
            .into_iter()
            .filter(|Pixel(pos, _color)| bb.contains(*pos))
            .for_each(|Pixel(pos, color)| {
                self.set_pixel(pos.x as u32, pos.y as u32, color.is_on());
            });

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.clear_impl(color.is_on());
        Ok(())
    }
}

impl<'a, DI, SIZE> OriginDimensions for MyType<'a, DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    fn size(&self) -> Size {
        let (w, h) = self.0.dimensions();

        Size::new(w.into(), h.into())
    }
}

impl<'a, DI, SIZE> DisplayConfig for MyType<'a, DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Error = DisplayError;

    /// Set the display rotation
    ///
    /// This method resets the cursor but does not clear the screen.
    fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DisplayError> {
        self.0.set_rotation(rot)
    }

    /// Initialise and clear the display in graphics mode.
    fn init(&mut self) -> Result<(), DisplayError> {
        (self.mode_mut().print_debug)("MyType init", 0);
        self.clear_impl(false);
        self.init_with_addr_mode(AddrMode::Horizontal)
    }
}

impl<'a, DI, SIZE> Deref for MyType<'a, DI, SIZE> {
    type Target = Ssd1306<DI, SIZE, NonBufferedMode<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, DI, SIZE> DerefMut for MyType<'a, DI, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
