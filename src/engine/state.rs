use sdl2::render::Renderer;

pub trait StateT {
    type Message;
    type EngineData;

    fn process_message(&mut self, &mut Self::EngineData, Self::Message) -> Option<Self::Message>;
    fn render(&mut self, &mut Renderer, &Self::EngineData);
    fn is_fullscreen(&self) -> bool;
}
