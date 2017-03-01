extern crate hungry_pixel_rs;

use hungry_pixel_rs::{Engine, TEngine, SDL2Context};

fn main() {
    let mut sdl_context = SDL2Context::new();
    let mut engine: Engine = Engine::new(&mut sdl_context);
    'running: loop {
        if !engine.process() {
            break 'running;
        }
    }
}
