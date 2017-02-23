use sdl2::render::{Texture, Renderer};
use msg::Msg;
use model::Model;
use engine::state::StateT;

struct MenuState {
    choices: Vec<(String, Texture, Msg)>,
    top_left: (i32, i32),
}

impl MenuState {
    fn new() -> MenuState {
        MenuState {
            choices: Vec::new(),
            top_left: (0, 0),
        }
    }
}

impl StateT for MenuState {
    type Message = Msg;
    type Model = Model;

    fn process_message(&mut self, model: &mut Model, msg: Msg) -> Option<Msg> {
        None
    }

    fn render(&mut self, r: &mut Renderer) {

    }
}
