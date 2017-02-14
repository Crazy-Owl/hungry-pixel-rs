extern crate sdl2;

pub mod resources;
mod state;
pub mod model;
pub mod engine;
pub mod context;

pub use engine::{Engine, TEngine};
pub use context::SDL2Context;

/// Message type
#[derive(Debug)]
pub enum Msg {
    NoOp,
    Exit,
    Change(String),
    Tick(u32),
}
