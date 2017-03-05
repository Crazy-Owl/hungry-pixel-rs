extern crate hungry_pixel_rs;

use hungry_pixel_rs::{Engine, TEngine};
use hungry_pixel_rs::engine::context::SDL2Context;

fn main() {
    let mut sdl_context = SDL2Context::new();
    let mut engine: Engine = Engine::new(&mut sdl_context);
    engine.start_game();
    'running: loop {
        if !engine.process() {
            break 'running;
        }
    }
}
