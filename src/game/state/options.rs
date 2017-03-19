use game::state::menu::{MenuState, MenuPosition};
use game::state::pixel::MOVEMENT_MAPPING;
use msg::{Movement, Msg};
use engine::data::EngineData;
use engine::state::StateT;
use engine::font::{FontCache, ColorMod};

use sdl2::render::{Renderer, Texture};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

pub struct OptionsState {
    menu: MenuState,
    message: Texture,
    current_receiver: Option<Movement>,
    is_dirty: bool,
}

impl OptionsState {
    pub fn new(font_cache: &mut FontCache, r: &mut Renderer) -> OptionsState {
        let choices = vec![("Up    Up", Msg::OptionsSelect(Movement::Up)),
                           ("Down  Down", Msg::OptionsSelect(Movement::Down)),
                           ("Left  Left", Msg::OptionsSelect(Movement::Left)),
                           ("Right Right", Msg::OptionsSelect(Movement::Right))];

        let menu = MenuState::new(r,
                                  font_cache,
                                  choices,
                                  Some(Msg::PopState(1)),
                                  MenuPosition::Centered,
                                  Some("Options"),
                                  true);
        let message = font_cache.render_texture(r, "default", "Press new control", None::<ColorMod>).unwrap();
        OptionsState {
            menu: menu,
            message: message,
            current_receiver: None,
            is_dirty: true,
        }
    }

    pub fn update_mappings(&mut self) {
        let movement_map = MOVEMENT_MAPPING.lock().unwrap();
        for &(idx, format, m) in
            &[(0, "Up", Movement::Up),
              (1, "Down", Movement::Down),
              (2, "Left", Movement::Left),
              (3, "Right", Movement::Right)] {
            for (key, value) in movement_map.iter() {
                if *value == m {
                    let new_string = format!("{:<6}{}", format, key);
                    self.menu.change_item_text(idx, new_string);
                }
            }
        }
        self.is_dirty = false;
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
        self.is_dirty = true;
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
            Msg::ButtonPressed(keycode) if self.current_receiver.is_some() => {
                Some(Msg::OptionsSet(keycode))
            }
            Msg::ButtonPressed(_) => self.menu.process_message(ed, msg),
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

    fn render(&mut self, r: &mut Renderer, ed: &mut EngineData) {
        if self.is_dirty {
            self.update_mappings();
        }
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
