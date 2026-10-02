use crate::nadk::display::{self, Color565, ScreenPoint, ScreenRect, push_rect, push_rect_uniform};

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flags {
    Base = 0,
    Transparent = 1,
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NumImageFormat {
    Simple = 0,
}

/// Struct who manage images
pub struct Image<'a> {
    format: NumImageFormat,
    width: u16,
    height: u16,
    data: &'a [u8],
    offset: usize,
}

impl Image <'_>{
    pub const fn new(data: &'static [u8]) -> Self {
        // Extract format and size
        let format = u16::from_le_bytes([
            data[0],
            data[1],
        ]); 
        let width = u16::from_le_bytes([
            data[2],
            data[3],
        ]); 
        let height = u16::from_le_bytes([
            data[4],
            data[5],
        ]); 

        let pixel_count = width as usize * height as usize;

        if pixel_count * 3 != data.len() - 6 {
            panic!("Corrupted data");
        }

        if format == NumImageFormat::Simple as u16 {
            Self {
                format: NumImageFormat::Simple,
                width,
                height,
                data,
                offset: 6,
            }
        } else {
            panic!("Unsupported format");
        }
    }

    pub fn draw(&self, x: u16, y: u16) {
        match self.format {
            NumImageFormat::Simple => self.draw_simple(x, y),
        }
    }

    /// Draw for simple format
    fn draw_simple(&self, x_d: u16, y_d: u16) {
        // Define mutable variables for coords
        let (mut x, mut y) = (x_d, y_d);

        for pix in self.data[self.offset..].chunks_exact(3) {
            // We get color and flag
            let color = u16::from_le_bytes([pix[0], pix[1]]);
            let flag = pix[2];

            // We draw the pixel
            if flag != Flags::Transparent as u8 {
                let pixel = ScreenRect::new(x, y, 1, 1);

                push_rect_uniform(pixel, Color565::from_u16(color));
            }

            // We update coords and we check values
            x += 1;
            if x >= self.width + x_d {
                x = x_d;
                y += 1;
            }

            if x > display::SCREEN_WIDTH || y > display::SCREEN_HEIGHT {
                break;
            }
        }
    }
}


