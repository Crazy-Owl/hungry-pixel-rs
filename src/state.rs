use sdl2::render::Renderer;

pub trait StateT {
    type Message;
    type Model;

    fn process_message (&mut self, Self::Model, Self::Message) -> Self::Model;
}

pub trait RenderableT {
    fn render (&mut self, Renderer);
}

pub struct MenuState {
    selected: usize,
    items: Vec<String>
}

impl RenderableT for MenuState {
    fn render (&mut self, r : Renderer) {
    }
}