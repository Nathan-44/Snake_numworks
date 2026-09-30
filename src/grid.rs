use crate::nadk::display;

use crate::common::{square};

pub struct Grid {
    color1: display::Color565,
    color2: display::Color565,
}

impl Grid {
    pub fn new(color1: display::Color565, color2: display::Color565) -> Grid {
        Grid { color1, color2 }
    }
    pub fn draw_all(&self) {
        for i in 0u16..10u16 {
            for y in 0u16..10u16 {
                let square = square(i, y);

                let i_pair = i % 2 == 0;
                let y_pair = y % 2 == 0;
                let color = if i_pair && y_pair || !i_pair && !y_pair {self.color1} else {self.color2};

                display::push_rect_uniform(square, color);
            }
        }
    }

    pub fn draw(&self, x: u16, y:u16) {
        let square = square(x, y);

        let x_pair = x % 2 == 0;
        let y_pair = y % 2 == 0;
        let color = if x_pair && y_pair || !x_pair && !y_pair {self.color1} else {self.color2};

        display::push_rect_uniform(square, color);
    }
}

