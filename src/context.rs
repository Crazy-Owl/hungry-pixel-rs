use sdl2::Sdl;
use sdl2::ttf::Sdl2TtfContext;

use sdl2;

/// SDL2 context
/// Holds all SDL2 context objects and is passed by reference to ensure proper lifetimes
pub struct SDL2Context {
    pub sdl2: Sdl,
    pub ttf: Sdl2TtfContext,
}

impl SDL2Context {
    pub fn new() -> SDL2Context {
        let sdl_context: Sdl = sdl2::init().expect("Could not initialize SDL!");
        let ttf_context: Sdl2TtfContext = sdl2::ttf::init()
            .expect("Could not initialize TTF context!");
        SDL2Context {
            sdl2: sdl_context,
            ttf: ttf_context,
        }
    }
}
