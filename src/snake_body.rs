//! Manage the snake itself

use crate::nadk::display::push_rect_uniform;
use crate::nadk::display;

use crate::common;

/// A part of the snake's body
#[derive(Clone, Copy)]
struct SnakePart {
    x: i16,
    y: i16,
}

impl SnakePart {
    fn draw(&self, color: display::Color565) {
        let square = common::square(self.x as u16, self.y as u16);

        push_rect_uniform(square, color);
    }

    /// Return true if the coos is the part
    fn is_part(&self, x: u16, y: u16) -> bool {
        self.x as u16 == x && self.y as u16 == y
    }

    fn coos_u16(&self) -> (u16, u16) {
        (self.x as u16, self.y as u16)
    }

    fn coos_i16(&self) -> (i16, i16) {
        (self.x, self.y)
    }

    // Move part without check
    fn move_part(&mut self, direction: &Directions) {
        match direction {
            Directions::EST => self.x += 1,
            Directions::NORTH => self.y -= 1,
            Directions::SOUTH => self.y += 1,
            Directions::WEST => self.x -= 1,
        }
    }

    fn move_part_to(&mut self, x: i16, y: i16) {
        self.x = x;
        self.y = y;
    }
}

#[derive(Clone, Copy)]
/// Directions of the snbake
pub enum Directions {
    NORTH,
    EST,
    WEST,
    SOUTH,
}

/// Entire snake body
pub struct Snake {
    body: [Option<SnakePart>; 100],
    direction: Directions,
    length: u16,
    color1: display::Color565,
    color_head: display::Color565,
    death_step: usize,
}

impl Snake {
    pub fn new(color1: display::Color565, color_head: display::Color565) -> Self {
        let mut snke = Self {
            body: [None; 100],
            direction: Directions::EST,
            length: 3,
            color1: color1,
            color_head: color_head,
            death_step: 0
        };

        // Adding head and 2 part of body
        snke.body[0] = Some(SnakePart{x: 2, y: 5});
        snke.body[1] = Some(SnakePart{x: 1, y: 5});
        snke.body[2] = Some(SnakePart{x: 0, y: 5});

        snke
    }

    pub fn draw(&self) {
        // We draw the head
        self.body[0].unwrap().draw(self.color_head);

        for part in &self.body[1..] {
            if let Some(snake_part) = part {
                snake_part.draw(self.color1);
            } else {
                break;
            }
        }
    }
    /// Getter to the last part's coos
    pub fn last_part_coos(&self) -> (u16, u16) {
        let index = (self.length - 1) as usize;
        self.body[index].unwrap().coos_u16()
    }

    /// Add a snake's part
    pub fn add_part(&mut self) {
        let index_old_last = (self.length - 1) as usize;
        let index_last = self.length as usize;

        let old_last = &self.body[index_old_last];

        let (old_x, old_y) = old_last.unwrap().coos_u16();

        self.body[index_last] = Some(SnakePart { 
            x: old_x as i16, 
            y: old_y as i16
        });

        self.length += 1;
    }

    pub fn move_snake(&mut self) -> bool{
        // We take the coos of the head to put them on the next part
        let (mut old_x, mut old_y) = self.body[0].unwrap().coos_i16();

        // Move the head
        self.body[0].as_mut().unwrap().move_part(&self.direction);

        // Check if the head is in a normal position
        let (mut new_x, mut new_y) = self.body[0].unwrap().coos_i16();
        if new_x < 0 || new_x > 9 || new_y < 0 || new_y > 9 {
            // We replace the head to draw
            self.body[0] = Some(
                SnakePart {
                    x: old_x, 
                    y: old_y 
                }
            );
            return false
        }

        // We move the snake's body
        for part_index in 1..self.length {
            let index = part_index as usize;

            (new_x, new_y) = self.body[index].unwrap().coos_i16();
            self.body[index].as_mut().unwrap().move_part_to(old_x, old_y);
            (old_x, old_y) = (new_x, new_y);
        }

        // Check if the head is in the body
        let (x, y) = self.body[0].unwrap().coos_u16();
        if self.in_snake_without_head(x, y) {
            return false;
        }
        
        true
    }

    /// Change direction
    pub fn change_direction(&mut self, direction: Directions) {
        let new_direction_n: i8;

        match direction {
            Directions::EST => new_direction_n = -1,
            Directions::NORTH => new_direction_n = -2,
            Directions::SOUTH => new_direction_n = 2,
            Directions::WEST => new_direction_n = 1,
        }

        let old_direction_n: i8;

        match self.direction {
            Directions::EST => old_direction_n = -1,
            Directions::NORTH => old_direction_n = -2,
            Directions::SOUTH => old_direction_n = 2,
            Directions::WEST => old_direction_n = 1,
        }

        if (new_direction_n + old_direction_n) != 0 {
            self.direction = direction;
        }
    }

    /// Return true if coos are in the snake
    pub fn in_snake(&self, x: u16, y: u16) -> bool {
        for part in &self.body {
            if let Some(snake_part) = part {
                if snake_part.is_part(x, y) {
                    return true;
                }
            } else {
                break;
            }
        }
        false
    }

    /// Return true if coos are in the snake (except for the head)
    pub fn in_snake_without_head(&self, x: u16, y: u16) -> bool {
        for part in &self.body[1..] {
            if let Some(snake_part) = part {
                if snake_part.is_part(x, y) {
                    return true;
                }
            } else {
                break;
            }
        }
        false
    }

    /// Return true if coos are the head
    pub fn in_head(&self, x: u16, y: u16) -> bool {
        if let Some(snake_part) = self.body[0] {
            if snake_part.is_part(x, y) {
                return true;
            }
        }
        false
    }

    // Init death animation and return time to wait
    pub fn init_death_animation(&mut self, time_death: u32) -> u32 {
        self.death_step = 0;
        
        (time_death as f64 / self.length as f64) as u32
    }

    pub fn draw_death_animation(&mut self) {
        // We draw a part of the body in red
        for part in &self.body[..self.death_step + 1] {
            if let Some(snake_part) = part {
                snake_part.draw(display::COLOR_RED);
            } else {
                break;
            }
        }
        for part in &self.body[self.death_step + 1..] {
            if let Some(snake_part) = part {
                snake_part.draw(self.color1);
            } else {
                break;
            }
        }
        self.death_step += 1;
    }

    pub fn is_dead(&self) -> bool {
        if self.death_step <= self.length as usize { false }
        else { true }
    }
}
