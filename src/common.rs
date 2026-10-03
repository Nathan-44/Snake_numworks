use crate::nadk::display::{ScreenPoint, ScreenRect as Rect};
use crate::nadk::{display, image};

// Screen dimension : 320x240
pub const GRID_X: u16 = 100;
pub const GRID_Y: u16 = 20;
pub const SQUARE_WIDTH: u16 = 20;
pub const SQUARE_HEIGHT: u16 = 20;

// Create a rect's square on grid
pub fn square(x: u16, y: u16) -> Rect{
    Rect { 
        x: (x * SQUARE_WIDTH) + GRID_X,
        y:  (y * SQUARE_HEIGHT) + GRID_Y,
        width: SQUARE_WIDTH,
        height: SQUARE_HEIGHT,
    }
}

pub fn square_point(x: u16, y: u16) -> ScreenPoint {
    ScreenPoint { 
        x: (x * SQUARE_WIDTH) + GRID_X, 
        y: (y * SQUARE_HEIGHT) + GRID_Y,
    }
}

pub const BG_COLOR: display::Color565 = display::Color565::from_rgb888(21, 34, 56);

pub static IMAGE_HEAD_LEFT: image::Image<'_> = image::Image::new(include_bytes!("../assets/tete.png.nat"));
