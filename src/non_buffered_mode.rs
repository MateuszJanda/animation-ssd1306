//! Non buffered graphics mode.

use display_interface::DisplayError;
use ssd1306::{
    command::AddrMode, mode::BasicMode, mode::DisplayConfig, rotation::DisplayRotation,
    size::DisplaySize, Ssd1306,
};

use core::ops::{Deref, DerefMut};

use ssd1306::prelude::WriteOnlyDataCommand;

#[derive(Clone, Debug)]
pub struct NonBufferedMode
// where
// SIZE: DisplaySize,
{
    min_x: u8,
    max_x: u8,
    min_y: u8,
    max_y: u8,
}

// impl<SIZE> NonBufferedMode<SIZE>
impl NonBufferedMode
// where
// SIZE: DisplaySize,
{
    /// Create a new buffered graphics mode instance.
    pub fn new() -> Self {
        Self {
            min_x: 255,
            max_x: 0,
            min_y: 255,
            max_y: 0,
        }
    }
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
pub struct MyType<DI, SIZE>(pub Ssd1306<DI, SIZE, NonBufferedMode>);
// where
//     DI: WriteOnlyDataCommand,
//     SIZE: DisplaySize;

impl<DI, SIZE> MyType<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    pub fn into_mode(self, mode: NonBufferedMode) -> Self {
        MyType(self.0.into_mode(mode))
    }

    pub fn new(ssd: Ssd1306<DI, SIZE, BasicMode>, mode: NonBufferedMode) -> Self
    {
        MyType(ssd.into_mode(mode))
    }

    fn clear_impl(&mut self, value: bool) {
        // let (width, height) = self.0.dimensions();
        // self.0.mode.min_x = 0;
        // self.0.mode.max_x = width - 1;
        // self.0.mode.min_y = 0;
        // self.0.mode.max_y = height - 1;
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, value: bool) {
        // let value = value as u8;
        // let rotation = self.rotation;

        // let (idx, bit) = match rotation {
        //     DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
        //         let idx = ((y as usize) / 8 * SIZE::WIDTH as usize) + (x as usize);
        //         let bit = y % 8;

        //         (idx, bit)
        //     }
        //     DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
        //         let idx = ((x as usize) / 8 * SIZE::WIDTH as usize) + (y as usize);
        //         let bit = x % 8;

        //         (idx, bit)
        //     }
        // };

        // if let Some(byte) = self.mode.buffer.as_mut().get_mut(idx) {
        //     // Keep track of max and min values
        //     self.mode.min_x = self.mode.min_x.min(x as u8);
        //     self.mode.max_x = self.mode.max_x.max(x as u8);

        //     self.mode.min_y = self.mode.min_y.min(y as u8);
        //     self.mode.max_y = self.mode.max_y.max(y as u8);

        //     // Set pixel value in byte
        //     // Ref this comment https://stackoverflow.com/questions/47981/how-do-you-set-clear-and-toggle-a-single-bit#comment46654671_47990
        //     *byte = *byte & !(1 << bit) | (value << bit);
        // }
    }
}

impl<DI, SIZE> DrawTarget for MyType<DI, SIZE>
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
        // let bb = self.bounding_box();

        // pixels
        //     .into_iter()
        //     .filter(|Pixel(pos, _color)| bb.contains(*pos))
        //     .for_each(|Pixel(pos, color)| {
        //         self.set_pixel(pos.x as u32, pos.y as u32, color.is_on());
        //     });

        Ok(())
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        // self.clear_impl(color.is_on());
        Ok(())
    }
}

impl<DI, SIZE> OriginDimensions for MyType<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    fn size(&self) -> Size {
        let (w, h) = self.0.dimensions();

        Size::new(w.into(), h.into())
    }
}

impl<DI, SIZE> DisplayConfig for MyType<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Error = DisplayError;

    /// Set the display rotation
    ///
    /// This method resets the cursor but does not clear the screen.
    fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DisplayError> {
        // self.set_rotation(rot)
        Ok(())
    }

    /// Initialise and clear the display in graphics mode.
    fn init(&mut self) -> Result<(), DisplayError> {
        // self.clear_impl(false);
        // self.init_with_addr_mode(AddrMode::Horizontal)
        Ok(())
    }
}

impl<DI, SIZE> Deref for MyType<DI, SIZE> {
    type Target = Ssd1306<DI, SIZE, NonBufferedMode>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<DI, SIZE> DerefMut for MyType<DI, SIZE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
