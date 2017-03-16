use sdl2::Sdl;

use sdl2;

/// SDL2 context
/// Holds all SDL2 context objects and is passed by reference to ensure proper lifetimes
pub struct SDL2Context {
    pub sdl2: Sdl,
}

impl SDL2Context {
    pub fn new() -> SDL2Context {
        let sdl_context: Sdl = sdl2::init().expect("Could not initialize SDL!");
        SDL2Context { sdl2: sdl_context }
    }
}
