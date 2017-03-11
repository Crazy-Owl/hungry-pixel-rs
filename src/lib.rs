#[macro_use]
extern crate lazy_static;
extern crate sdl2;
extern crate rand;

pub mod resources;
pub mod game;
pub mod engine;
mod msg;

pub use engine::{Engine, TEngine};
