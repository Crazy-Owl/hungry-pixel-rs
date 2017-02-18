use sdl2::render::Renderer;

pub mod game;

pub trait StateT {
    type Message;
    type Model;

    fn process_message(&mut self, &mut Self::Model, Self::Message) -> Option<Self::Message>;
    fn render(&mut self, &mut Renderer);
}

// pub struct MenuState {
//     selected: usize,
//     items: Vec<String>,
// }
