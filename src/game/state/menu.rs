use sdl2::render::{Texture, Renderer};
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use sdl2::pixels::Color::*;
use msg::{Msg, ControlCommand};
use engine::data::EngineData;
use engine::state::StateT;

struct MenuItem {
    texture: Texture,
    dimensions: (u32, u32),
    msg: Msg,
}

pub struct MenuState {
    menu_items: Vec<MenuItem>,
    currently_selected: i8,
    top_left: (i32, i32),
}

impl<'m> MenuState {
    pub fn new(f: &Font<'m, 'static>, r: &mut Renderer, choices: Vec<(String, Msg)>) -> MenuState {
        let mut menu_items = Vec::new();
        for choice in choices {
            let surface =
                f.render(&choice.0).solid(RGB(255, 255, 255)).expect("Could not render text!");
            let texture = r.create_texture_from_surface(&surface).expect("Could not render text!");
            let query = texture.query();
            menu_items.push(MenuItem {
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
    type EngineData = EngineData;

    fn process_message(&mut self, engine_data: &mut EngineData, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::ButtonPressed(ControlCommand::Up) => {
                self.currently_selected -= 1;
                if self.currently_selected < 0 {
                    self.currently_selected = self.menu_items.len() as i8 - 1;
                }
                None
            }
            Msg::ButtonPressed(ControlCommand::Down) => {
                self.currently_selected += 1;
                if self.currently_selected > self.menu_items.len() as i8 - 1 {
                    self.currently_selected = 0;
                }
                None
            }
            Msg::ButtonPressed(ControlCommand::Enter) => {
                Some(self.menu_items[self.currently_selected as usize].msg)
            }
            Msg::ButtonPressed(ControlCommand::Escape) => Some(Msg::Exit),
            _ => None,
        }
    }

    fn render(&mut self, r: &mut Renderer) {
        let mut current_y: u32 = self.top_left.1 as u32;
        let mut running_counter: usize = 0;
        for item in &self.menu_items {
            r.copy(&item.texture,
                      None,
                      Some(Rect::new(self.top_left.0 as i32,
                                     current_y as i32,
                                     item.dimensions.0,
                                     item.dimensions.1)))
                .unwrap();
            if running_counter == self.currently_selected as usize {
                r.set_draw_color(RGB(255, 255, 255));
                r.draw_rect(Rect::new(self.top_left.0 - 20,
                                         current_y as i32,
                                         20,
                                         item.dimensions.1))
                    .unwrap();
            }
            current_y += item.dimensions.1 + 2;
            running_counter += 1;
        }
    }
}
