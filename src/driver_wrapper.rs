//! Ssd1306 driver wrapper with custom MinBufferMode graphics mode.

use core::ops::{Deref, DerefMut};
use display_interface::DataFormat::U8;
use display_interface::DisplayError;
use ssd1306::prelude::WriteOnlyDataCommand;
use ssd1306::{
    command::AddrMode, mode::BasicMode, mode::DisplayConfig, rotation::DisplayRotation,
    size::DisplaySize, Ssd1306,
};
// #[cfg(feature = "graphics")]
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::Size,
    geometry::{Dimensions, OriginDimensions},
    pixelcolor::BinaryColor,
    Pixel,
};

/// Custom buffered graphics mode.
///
/// MinBufferMode with array of size 1.
pub struct MinBufferMode {
    /// One-byte array
    buffer: [u8; 1],
    /// Last x position of set pixel.
    last_x: u8,
    /// Last y position of set pixel.
    last_y: u8,
}

impl MinBufferMode {
    /// Create a new buffered graphics mode (MinBufferMode with array of size 1) instance.
    /// Here driver read and write some temporary information.
    pub fn new() -> Self {
        Self {
            buffer: [0x00],
            last_x: u8::MAX,
            last_y: u8::MAX,
        }
    }
}

/// Wrapper (tuple struct) on Ssd1306 driver to use custom MinBufferMode.
pub struct Ssd1306DriverWrapper<DI, SIZE>(pub Ssd1306<DI, SIZE, MinBufferMode>);

impl<DI, SIZE> Ssd1306DriverWrapper<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    /// Create instance of Ssd1306DriverWrapper
    pub fn new(ssd: Ssd1306<DI, SIZE, BasicMode>, mode: MinBufferMode) -> Self {
        Ssd1306DriverWrapper(ssd.into_mode(mode))
    }

    /// Reset all data stored in mode, and clear display
    fn clear_impl(&mut self, pixels_on: bool) {
        // Invalidate last_x and last_y
        self.mode_mut().last_x = u8::MAX;
        self.mode_mut().last_y = u8::MAX;

        // Clear display
        let (width, height) = self.dimensions();
        self.set_draw_area((0, 0), (width, height)).unwrap();
        let pixels_strip = match pixels_on {
            true => &[0xff],
            false => &[0x00],
        };
        let num_of_bytes = (width as u32 * height as u32) / 8;
        for _ in 0..num_of_bytes {
            self.interface_mut().send_data(U8(pixels_strip)).unwrap();
        }
    }

    /// Turn a pixel on or off. Before sending to device, pixels are stored in one-byte buffer.
    pub fn set_pixel(&mut self, x: u32, y: u32, pixel_on: bool) {
        // If given pixel is not covered by buffer, flush buffer with currently collected pixels.
        if self.is_pixel_out_of_buffer(x as u8, y as u8) {
            self.flush().unwrap();
        }

        let rotation = self.rotation();
        let bit_num = match rotation {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                let bit_num = y % 8;
                bit_num
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                let bit_num = x % 8;
                bit_num
            }
        };

        // Save pixel in one-byte buffer
        let pixel_on = pixel_on as u8;
        self.mode_mut().buffer[0] =
            self.mode_mut().buffer[0] & !(1 << bit_num) | (pixel_on << bit_num);

        // Save current pixel position as last modified
        self.mode_mut().last_x = x as u8;
        self.mode_mut().last_y = y as u8;
    }

    /// Check if pixel is out of area currently covered by buffer
    fn is_pixel_out_of_buffer(&mut self, x: u8, y: u8) -> bool {
        if self.mode().last_x == u8::MAX || self.mode().last_y == u8::MAX {
            return false;
        }

        let result = match self.rotation() {
            DisplayRotation::Rotate0 | DisplayRotation::Rotate180 => {
                x != self.mode().last_x || (y / 8) != (self.mode().last_y / 8)
            }
            DisplayRotation::Rotate90 | DisplayRotation::Rotate270 => {
                (x / 8) != (self.mode().last_x / 8) || y != self.mode().last_y
            }
        };

        result
    }

    /// New wrapper method. Setup draw area.
    pub fn setup(&mut self) -> Result<(), DisplayError> {
        let (width, height) = self.dimensions();
        self.set_draw_area((0, 0), (width, height))
    }

    /// New wrapper method. Pass buffer (8-bit long vertical strips) directly to device (by SPI interface).
    pub fn draw_strips_from_buffer(&mut self, buffer: &[u8]) -> Result<(), DisplayError> {
        self.interface_mut().send_data(U8(buffer))
    }

    /// Send data from (one-byte) buffer to device (by SPI interface).
    pub fn flush(&mut self) -> Result<(), DisplayError> {
        if self.mode().last_x == u8::MAX || self.mode().last_y == u8::MAX {
            return Ok(());
        }

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

        self.set_draw_area((disp_min_x, disp_min_y), (disp_max_x, disp_max_y))
            .unwrap();
        let buffer_copy = self.mode().buffer;
        self.interface_mut().send_data(U8(&buffer_copy)).unwrap();

        // Empty byte buffer
        self.mode_mut().buffer[0] = 0x00;

        // Invalidate last_x and last_y
        self.mode_mut().last_x = u8::MAX;
        self.mode_mut().last_y = u8::MAX;

        return Ok(());
    }
}

impl<DI, SIZE> DrawTarget for Ssd1306DriverWrapper<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Color = BinaryColor;
    type Error = DisplayError;

    /// Copied from BufferedGraphicsMode.
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

    /// Copied from BufferedGraphicsMode.
    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.clear_impl(color.is_on());
        Ok(())
    }
}

impl<DI, SIZE> OriginDimensions for Ssd1306DriverWrapper<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    /// Copied from BufferedGraphicsMode. Get Display size.
    fn size(&self) -> Size {
        let (w, h) = self.0.dimensions();

        Size::new(w.into(), h.into())
    }
}

impl<DI, SIZE> DisplayConfig for Ssd1306DriverWrapper<DI, SIZE>
where
    DI: WriteOnlyDataCommand,
    SIZE: DisplaySize,
{
    type Error = DisplayError;

    /// Set the display rotation.
    ///
    /// This method resets the cursor but does not clear the screen.
    fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), DisplayError> {
        self.0.set_rotation(rot)
    }

    /// Initialize (with AddrMode::Horizontal ad default) and clear the display in graphics mode.
    fn init(&mut self) -> Result<(), DisplayError> {
        self.clear_impl(false);
        self.init_with_addr_mode(AddrMode::Horizontal)
    }
}

impl<DI, SIZE> Deref for Ssd1306DriverWrapper<DI, SIZE> {
    type Target = Ssd1306<DI, SIZE, MinBufferMode>;

    /// Add deref to access Ssd1306DriverWrapper same way as to Ssd1306. Without "self.0" as
    /// Ssd1306DriverWrapper is tuple struct.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<DI, SIZE> DerefMut for Ssd1306DriverWrapper<DI, SIZE> {
    /// Add deref to access Ssd1306DriverWrapper same way as to Ssd1306. Without "self.0" as
    /// Ssd1306DriverWrapper is tuple struct.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
