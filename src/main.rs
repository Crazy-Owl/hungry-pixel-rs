extern crate rogue;

use rogue::{Engine, TEngine, SDL2Context};

fn main() {
    let mut sdl_context = SDL2Context::new();
    let mut engine: Engine = Engine::new(&mut sdl_context);
    'running: loop {
        if !engine.process() {
            break 'running;
        }
    }
}
