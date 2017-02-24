use sdl2::render::{Texture, Renderer};
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use sdl2::pixels::Color::*;
use msg::Msg;
use model::Model;
use engine::state::StateT;

struct MenuItem {
    text: String,
    texture: Texture,
    dimensions: (u32, u32),
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
            let query = texture.query();
            menu_items.push(MenuItem {
                text: choice.0,
                texture: texture,
                dimensions: (query.width, query.height),
                msg: choice.1,
            });
        }
        MenuState {
            menu_items: menu_items,
            currently_selected: 0,
            top_left: (60, 0),
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
        let mut current_y: u32 = self.top_left.1 as u32;
        let mut running_counter: usize = 0;
        for item in &self.menu_items {
            r.copy(&item.texture, None, Some(Rect::new(current_y as i32, self.top_left.0 as i32, item.dimensions.0, item.dimensions.1))).unwrap();
            current_y += item.dimensions.1 + 2;
            if running_counter == self.currently_selected {
                r.set_draw_color(RGB(255, 255, 255));
                r.draw_rect(Rect::new(self.top_left.0 - 20, current_y as i32, 20, item.dimensions.1)).unwrap();
            }
            running_counter += 1;
        }
    }
}
