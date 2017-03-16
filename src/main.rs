extern crate hungry_pixel_rs;
extern crate sdl2;

use hungry_pixel_rs::{Engine, TEngine};
use hungry_pixel_rs::engine::context::SDL2Context;

fn main() {
    let sdl_context = SDL2Context::new();
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut engine: Engine = Engine::new(sdl_context, ttf_context);
    engine.start_game();
    'running: loop {
        if !engine.process() {
            break 'running;
        }
    }
}
