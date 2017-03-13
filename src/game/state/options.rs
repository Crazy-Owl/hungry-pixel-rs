use game::state::menu::{MenuState, MenuPosition};
use game::state::pixel::MOVEMENT_MAPPING;
use msg::{Movement, Msg};
use engine::data::EngineData;
use engine::state::StateT;

use sdl2::ttf::Font;
use sdl2::render::{Renderer, Texture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color::*;
use sdl2::rect::Rect;

pub struct OptionsState {
    menu: MenuState,
    message: Texture,
    current_receiver: Option<Movement>,
}

impl OptionsState {
    pub fn new<'m>(f: &Font<'m, 'static>, r: &mut Renderer) -> OptionsState {
        // TODO: save texture with text here
        let choices = vec![("Up".to_string(), Msg::OptionsSelect(Movement::Up)),
                           ("Down".to_string(), Msg::OptionsSelect(Movement::Down)),
                           ("Left".to_string(), Msg::OptionsSelect(Movement::Left)),
                           ("Right".to_string(), Msg::OptionsSelect(Movement::Right))];
        let menu = MenuState::new(f,
                                  r,
                                  choices,
                                  Some(Msg::PopState(1)),
                                  MenuPosition::Centered,
                                  Some((f, "Options".to_string())),
                                  true);
        let message = r.create_texture_from_surface(&(f.render("Press new control")
                .solid(RGB(255, 255, 255))
                .expect("Could not render text!")))
            .expect("Could not render text!");
        OptionsState {
            menu: menu,
            message: message,
            current_receiver: None,
        }
    }

    pub fn remap_key(&mut self, m: Movement, k: Keycode) -> Option<Msg> {
        let mut movement_map = MOVEMENT_MAPPING.lock().unwrap();

        let mut remove_keys: Vec<Keycode> = vec![];

        for (key, value) in movement_map.iter() {
            if *value == m || *key == k {
                remove_keys.push(*key);
            }
        }

        for key in &remove_keys {
            movement_map.remove(key);
        }

        movement_map.insert(k, m);
        None
    }
}

impl StateT for OptionsState {
    type Message = Msg;
    type EngineData = EngineData;

    fn process_message(&mut self, ed: &mut EngineData, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::Tick(_) |
            Msg::ButtonReleased(_) => None,
            Msg::ButtonPressed(keycode) => {
                if self.current_receiver.is_some() {
                    Some(Msg::OptionsSet(keycode))
                } else {
                    self.menu.process_message(ed, msg)
                }
            }
            Msg::OptionsSelect(movement) => {
                self.current_receiver = Some(movement);
                None
            }
            Msg::OptionsSet(keycode) => {
                if let Some(movement) = self.current_receiver {
                    self.current_receiver = None;
                    self.remap_key(movement, keycode)
                } else {
                    None
                }
            }
            msg => Some(msg),
        }
    }

    fn render(&mut self, r: &mut Renderer, ed: &EngineData) {
        self.menu.render(r, ed);
        if self.current_receiver.is_some() {
            let message_query = self.message.query();
            r.copy(&self.message,
                      None,
                      Some(Rect::new((ed.window_size.0 / 2 - message_query.width / 2) as i32,
                                     (ed.window_size.1 / 2 + self.menu.get_dimensions().1 / 2 +
                                      message_query.height * 2) as
                                     i32,
                                     message_query.width,
                                     message_query.height)))
                .unwrap();
        }
    }

    fn is_fullscreen(&self) -> bool {
        true
    }
}
