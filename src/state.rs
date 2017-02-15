use sdl2::render::Renderer;

use super::msg::{Msg, ControlCommand};
use super::model::Model;

pub trait StateT {
    type Message;
    type Model;

    fn process_message(&mut self, &mut Self::Model, Self::Message) -> Option<Self::Message>;
    fn render(&mut self, &Renderer);
}

// pub struct MenuState {
//     selected: usize,
//     items: Vec<String>,
// }


pub struct Edible(f32, f32, f32);

pub struct GameState {
    running: bool,
    player: (f32, f32, f32),
    player_direction: Vec<ControlCommand>,
    items: Vec<Edible>,
}

impl StateT for GameState {
    type Message = Msg;
    type Model = Model;

    fn process_message(&mut self,
                       model: &mut Self::Model,
                       msg: Self::Message)
                       -> Option<Self::Message> {
        match msg {
            Msg::ButtonPressed(ControlCommand::Escape) => Some(Msg::Exit),
            Msg::ButtonPressed(_) => None,
            _ => None,
        }
    }

    fn render(&mut self, r: &Renderer) {}
}
