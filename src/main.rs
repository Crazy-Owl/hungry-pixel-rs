extern crate rogue;

use rogue::{Engine, Msg, TEngine};

fn main() {
    let mut engine : Engine = Engine::new();
    engine.messages.push_back(Msg::Change("Kek".to_string()));
    engine.messages.push_back(Msg::NoOp);
    'running: loop {
        if !engine.process() {
            break 'running;
        }
    }
}
