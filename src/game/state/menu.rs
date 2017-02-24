use sdl2::render::{Texture, Renderer};
use sdl2::ttf::Font;
use sdl2::pixels::Color::*;
use msg::Msg;
use model::Model;
use engine::state::StateT;

struct MenuItem {
    text: String,
    texture: Texture,
    msg: Msg,
}

struct MenuState {
    menu_items: Vec<MenuItem>,
    currently_selected: usize,
    top_left: (i32, i32),
}

impl<'m> MenuState {
    fn new(f: &Font<'m, 'static>, r: &mut Renderer, choices: Vec<(String, Msg)>) -> MenuState {
        let mut menu_items = Vec::new();
        for choice in choices {
            let surface =
                f.render(&choice.0).solid(RGB(255, 255, 255)).expect("Could not render text!");
            let texture = r.create_texture_from_surface(&surface).expect("Could not render text!");
            menu_items.push(MenuItem {
                text: choice.0,
                texture: texture,
                msg: choice.1,
            });
        }
        MenuState {
            menu_items: menu_items,
            currently_selected: 0,
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

    fn render(&mut self, r: &mut Renderer) {}
}
