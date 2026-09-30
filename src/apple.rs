use crate::nadk::{self, random};
use crate::nadk::display::push_rect_uniform;

use crate::common;

pub struct Apple {
    x: u16,
    y: u16,
}

impl Apple {
    pub fn new() -> Self{
        Self {
            x: 0,
            y: 0,
        }
    }
    pub fn generate(&mut self) -> (u16, u16) {
        let x = random::get_random_in_range(0, 10) as u16;
        let y = random::get_random_in_range(0, 10) as u16;

        self.x = x;
        self.y = y;

        (x, y)
    }
    pub fn draw(&self) {
        let (x, y) = self.coos();
        let square = common::square(x, y);

        push_rect_uniform(square, nadk::display::COLOR_RED);
    }
    pub fn coos(&self) -> (u16, u16) {
        (self.x, self.y)
    }
}
