use alloc::string::ToString;

use crate::nadk::display::ScreenPoint as Point;
use crate::nadk::display::{draw_string, push_rect_uniform};
use crate::nadk::display;
use crate::nadk::keyboard::Key;
use crate::nadk::time::{get_current_time_millis, wait_milliseconds};
use crate::nadk::display::Color565;

use crate::{common, nadk};
use crate::grid::Grid;
use crate::apple::Apple;
use crate::snake_body::{Snake, Directions};
use crate::save;

// Game speed
const VERY_FAST: (&str, u16) = ("Very fast", 100);
const FAST: (&str, u16) = ("Fast", 150);
const MEDIUM: (&str, u16) = ("Medium", 200);
const SLOW: (&str, u16) = ("Slow", 250);
const VERY_SLOW: (&str, u16) = ("Very slow", 300);
const GAME_SPEED: [(&str, u16); 5] = [VERY_FAST, FAST, MEDIUM, SLOW, VERY_SLOW];
const N_GAME_SPEED: usize = 4;

const DEATH_TIME: u32 = 1000;

// Game sate
enum GameState {
    Begin,
    InGame,
    BigTimer,
    TinyTimer,
}

// Timer text
const TIMER_TEXT: [&str; 4] = ["3 !", "2 !", "1 !", "GO !"];
const TIMER_SIZE: usize = 4;

/// Struct who manage the game
pub struct Game {
    score: u8,
    grid: Grid,
    apple: Apple,
    player: Snake,
    grid_snake_draw: Option<(u16, u16)>,
    time: u64,
    actual_game_speed: usize,
    temp_direction: Directions,
    state: GameState,
    timer: usize,
    best_score: u8,
    tiny_timer: u32,
}

impl Game {
    pub fn new() -> Self{
        let mut game = Self { 
            score: 0, 
            grid: Grid::new(
                Color565::from_rgb888(135, 206, 235), 
                Color565::from_rgb888(176, 224, 230)
            ),
            apple: Apple::new(),
            player: Snake::new(
                Color565::from_rgb888(80,200,120), 
                Color565::from_rgb888(46,111,64)
            ),
            grid_snake_draw: None,
            time: 0,
            actual_game_speed: 2, // Medium game speed
            state: GameState::Begin,
            temp_direction: Directions::EAST, // Avoid bugs between 2 move
            timer: 0,
            best_score: 0,
            tiny_timer: 0,
        };

        // Load best score
        game.best_score = save::load_score();

        // Init game
        game.init();

        game
    }

    /// Init the game to its initial state
    fn init(&mut self) {
        push_rect_uniform(display::SCREEN_RECT, common::BG_COLOR);

        // Set score
        self.score = 0;

        // Draw grid at creation
        self.grid.draw_all();

        // Draw score
        draw_string(
            "Score :",
            Point{
                x: 0,
                y: 0,
            }, 
            true, 
            display::COLOR_WHITE, 
            common::BG_COLOR,
        );
        draw_string(
            "\nHigh \nscore :",
            Point{
                x: 0,
                y: 40,
            }, 
            true, 
            display::COLOR_WHITE, 
            common::BG_COLOR,
        );

        // Generate Apple
        self.generate_apple();

        // Init snake
        self.grid_snake_draw = None;

        self.player = Snake::new(
                Color565::from_rgb888(60,170,80), 
                Color565::from_rgb888(46,111,64)
        );

        self.temp_direction = Directions::EAST;
        self.time = 0;
    }

    /// Start screen
    fn draw_begin_screen(&self) {
        push_rect_uniform(display::SCREEN_RECT, common::BG_COLOR);

        // Select game speed
        draw_string(
            "Game Speed :",
            Point { x: 30, y: 20 }, 
            true, 
            display::COLOR_WHITE, 
        common::BG_COLOR,
        );
        draw_string(
            GAME_SPEED[self.actual_game_speed].0,
            Point { x: 30, y: 40 }, 
            true, 
            display::COLOR_WHITE, 
        common::BG_COLOR,
        );
    }

    /// Draw game
    pub fn draw(&mut self) {
        match self.state {
            GameState::Begin => self.draw_begin_screen(),
            GameState::InGame => {
                // Draw apple
                self.apple.draw();

                // Draw score
                self.draw_score();

                // Draw grid
                if let Some(coos) = self.grid_snake_draw {
                    let (x, y) = coos;
                    self.grid.draw(x, y);
                    self.grid_snake_draw = None;
                }

                // Draw snake
                self.player.draw();
            }
            GameState::BigTimer => {
                draw_string(
                    TIMER_TEXT[self.timer],
                    Point { x: 0, y: 150 }, 
                    true, 
                    display::COLOR_WHITE, 
                common::BG_COLOR,
                );

                // Draw apple
                self.apple.draw();
                // Draw score
                self.draw_score();
                // Draw snake
                self.player.draw();
            },
            GameState::TinyTimer => self.player.draw_death_animation(),
        }
    }

    /// Update game state
    /// Return false if game should stop
    pub fn update(&mut self, just: nadk::keyboard::KeyboardState) -> bool{
        match self.state {
            GameState::Begin => {
                // Back key: exit the application.
                if just.key_down(Key::Back) || just.key_down(Key::Home) {
                    return false;
                }
                // Change game speed
                if (just.key_down(Key::Up) || just.key_down(Key::Plus)) && self.actual_game_speed > 0 {
                    self.actual_game_speed -= 1;
                }
                if (just.key_down(Key::Down) || just.key_down(Key::Minus)) && self.actual_game_speed < N_GAME_SPEED {
                    self.actual_game_speed += 1;
                }

                // Start game
                if just.key_down(Key::Ok) {
                    self.state = GameState::BigTimer;
                    self.timer = 0;
                    self.init();
                }
            }
            GameState::InGame => {
                // Back key: exit the application.
                if just.key_down(Key::Back) || just.key_down(Key::Home) {
                    return false;
                }
                if just.key_down(Key::Up) || just.key_down(Key::Eight) {
                    self.temp_direction = Directions::NORTH;
                } 
                else if just.key_down(Key::Down) || just.key_down(Key::Two) {
                    self.temp_direction = Directions::SOUTH;
                } 
                else if just.key_down(Key::Left) || just.key_down(Key::Four) {
                    self.temp_direction = Directions::WEST;
                } 
                else if just.key_down(Key::Right) || just.key_down(Key::Six) {
                    self.temp_direction = Directions::EAST;
                }

                // Calculate time
                let new_time = get_current_time_millis();
                if (new_time - self.time) > GAME_SPEED[self.actual_game_speed].1 as u64 {
                    self.time = new_time;

                    // Change direction and move
                    self.player.change_direction(self.temp_direction);
                    self.grid_snake_draw = Some(self.player.last_part_coos());
                    let can_move = self.player.move_snake();

                    // If the player can't move, we update best score and restart the game
                    if !can_move {
                        if self.best_score < self.score {
                            self.best_score = self.score;
                            save::save_score(self.best_score);
                        }
                        self.state = GameState::TinyTimer;
                        self.tiny_timer = self.player.init_death_animation(DEATH_TIME);
                    }
                }

                // If the head is on the apple
                let (x_apple, y_apple) = self.apple.coords();
                if self.player.in_head(x_apple, y_apple) {
                    self.player.add_part();
                    self.score += 1;
                    self.generate_apple();
                }
            }
            GameState::BigTimer => {
                // Back key: exit the application.
                if just.key_down(Key::Back) || just.key_down(Key::Home) {
                    return false;
                }
                // Wait 1 seconds
                wait_milliseconds(1000);
                self.timer += 1;

                // At timer's end, the game begin
                if self.timer >= TIMER_SIZE {
                    self.state = GameState::InGame;
                }
            },
            GameState::TinyTimer => {
                // Back key: exit the application.
                if just.key_down(Key::Back) || just.key_down(Key::Home) {
                    return false;
                }
                if self.player.is_dead() {
                    self.state = GameState::InGame;
                    self.init();
                }
                else {
                    wait_milliseconds(self.tiny_timer);
                }
            }         
        }
        true
    }

    /// Draw the score
    pub fn draw_score(&self){
        draw_string(
            &self.score.to_string(),
            Point{
                x: 5,
                y: 20,
            }, 
            true, 
            display::COLOR_WHITE, 
            common::BG_COLOR,
        );
        draw_string(
            &self.best_score.to_string(),
            Point{
                x: 5,
                y: 40+18+18+20,
            }, 
            true, 
            display::COLOR_WHITE, 
            common::BG_COLOR,
        );
    }

    /// Generate the apple and check if the apple is in the snake
    pub fn generate_apple(&mut self) {
        let (mut x,mut y) = self.apple.generate();

        while self.player.in_snake(x, y) {
            (x, y) = self.apple.generate();
        }
    }
}