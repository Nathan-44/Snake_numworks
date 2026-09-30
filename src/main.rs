#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[macro_use]
mod nadk;

// The app name must be a C string and the app name size must include the end line NULL character
configure_app!(b"Snake\0", 6, "../target/icon.nwi", 3107);

// Setup the heap allocator if you need one
setup_allocator!();

#[cfg(not(target_os = "none"))]
extern crate alloc;

mod game;
mod apple;
mod common;
mod grid;
mod save;
mod snake_body;

use nadk::keyboard::KeyboardState;

#[unsafe(no_mangle)]
fn main() {
    // You must call setup_allocator!() before
    init_heap!();

     // === Main loop

    // Keyboard state from the previous frame, used to compute just-pressed keys.
    let mut prev = KeyboardState::scan();

    // Application main loop flag. Set to false to exit.
    let mut running = true;

    // Init game
    let mut snake_game = game::Game::new();

    while running {
        // Scan the current keyboard state.
        let now = KeyboardState::scan();

        let just = now.get_just_pressed(prev);

        // Update game state
        running = snake_game.update(just);

        // Wait for the vertical blank
        nadk::display::wait_for_vblank();

        // Draw game
        snake_game.draw();

        // Save the current keyboard state for just-pressed detection next frame.
        prev = now;
    }
}
