extern crate rogue;

use rogue::{Engine, TEngine};

fn main() {
    let mut engine: Engine = Engine::new();
    'running: loop {
        if !engine.process() {
            break 'running;
        }
    }
}
