use sdl2::render::{Texture, Renderer};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::pixels::Color::*;

use msg::Msg;
use engine::data::EngineData;
use engine::state::StateT;
use engine::font::{FontCache, ColorMod};
use std::cmp;

pub enum MenuPosition {
    Centered,
    Pos(u32, u32),
}

struct MenuItem {
    text: String,
    texture: Texture,
    dimensions: (u32, u32),
    msg: Msg,
}

pub struct MenuState {
    menu_items: Vec<MenuItem>,
    currently_selected: i8,
    dimensions: (u32, u32),
    on_escape: Option<Msg>,
    position: MenuPosition,
    decoration: Option<MenuItem>,
    is_fullscreen: bool,
    is_dirty: bool,
}

impl<'m, 'b> MenuState {
    pub fn new<T: Into<String>>(r: &mut Renderer,
                                font_cache: &mut FontCache,
                                choices: Vec<(T, Msg)>,
                                on_escape: Option<Msg>,
                                position: MenuPosition,
                                decoration_parameters: Option<(T)>,
                                is_fullscreen: bool)
                                -> MenuState {
        let mut menu_items = Vec::new();
        let mut max_width: u32 = 0;
        let mut max_height: u32 = 0;
        for choice in choices {
            let text = choice.0.into();
            let texture = font_cache.render_texture(r, "default", &text, None::<ColorMod>).unwrap();
            let query = texture.query();
            menu_items.push(MenuItem {
                text: text,
                texture: texture,
                dimensions: (query.width, query.height),
                msg: choice.1,
            });
            if query.width > max_width {
                max_width = query.width;
            }
            max_height += query.height;
        }
        let decoration_item = if let Some(s) = decoration_parameters {
            let text = s.into();
            let texture = font_cache.render_texture(r, "default", &text, None::<ColorMod>).unwrap();
            let query = texture.query();
            Some(MenuItem {
                text: text,
                texture: texture,
                dimensions: (query.width, query.height),
                msg: Msg::NoOp,
            })
        } else {
            None
        };
        MenuState {
            menu_items: menu_items,
            currently_selected: 0,
            dimensions: (max_width, max_height),
            on_escape: on_escape,
            position: position,
            decoration: decoration_item,
            is_fullscreen: is_fullscreen,
            is_dirty: false, // TODO
        }
    }

    pub fn rerender_menu_items(&mut self, r: &mut Renderer, fc: &mut FontCache) {
        for menu_item in &mut self.menu_items {
            menu_item.texture = fc.render_texture(r, "default", &menu_item.text, None::<ColorMod>).unwrap();
            let query = menu_item.texture.query();
            menu_item.dimensions = (query.width, query.height);
        }

        if let Some(ref mut decoration) = self.decoration {
            decoration.texture = fc.render_texture(r, "default", &decoration.text, None::<ColorMod>).unwrap();
            let query = decoration.texture.query();
            decoration.dimensions = (query.width, query.height);
        }

        self.resize();

        self.is_dirty = false;
    }

    pub fn resize(&mut self) {
        let mut max_width = 0;
        for menu_item in &self.menu_items {
            if menu_item.dimensions.0 > max_width {
                max_width = menu_item.dimensions.0;
            }
        }
        self.dimensions.0 = max_width;
    }

    pub fn change_item_text<T: Into<String>>(&mut self, idx: usize, new_text: T) {
        let item: &mut MenuItem = &mut self.menu_items[idx];
        item.text = new_text.into();
        self.is_dirty = true;
    }

    pub fn process_button(&mut self, k: Keycode) -> Option<Msg> {
        match k {
            Keycode::Up => {
                self.currently_selected -= 1;
                if self.currently_selected < 0 {
                    self.currently_selected = self.menu_items.len() as i8 - 1;
                }
                None
            }
            Keycode::Down => {
                self.currently_selected += 1;
                if self.currently_selected > self.menu_items.len() as i8 - 1 {
                    self.currently_selected = 0;
                }
                None
            }
            Keycode::Return => Some(self.menu_items[self.currently_selected as usize].msg),
            Keycode::Escape => self.on_escape,
            _ => None,
        }
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        if let Some(ref decoration) = self.decoration {
            let x = cmp::max(self.dimensions.0, decoration.dimensions.0);
            let y = self.dimensions.1 + decoration.dimensions.1;
            (x, y)
        } else {
            (self.dimensions.0, self.dimensions.1)
        }
    }
}

impl StateT for MenuState {
    type Message = Msg;
    type EngineData = EngineData;

    fn process_message(&mut self, _: &mut EngineData, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::Tick(_) |
            Msg::ButtonReleased(_) => None,
            Msg::ButtonPressed(keycode) => self.process_button(keycode),
            msg => Some(msg),
        }
    }

    fn render(&mut self, r: &mut Renderer, ed: &mut EngineData) {
        if self.is_dirty {
            self.rerender_menu_items(r, &mut ed.font_cache);
        }
        let mut current_y: u32 = match self.position {
            MenuPosition::Centered => (ed.window_size.1 / 2) - (self.dimensions.1 / 2),
            MenuPosition::Pos(_, y) => y,
        };
        let x: u32 = match self.position {
            MenuPosition::Centered => (ed.window_size.0 / 2) - (self.dimensions.0 / 2),
            MenuPosition::Pos(x, _) => x,
        };
        if let Some(ref it) = self.decoration {
            r.copy(&it.texture,
                      None,
                      Some(Rect::new((ed.window_size.0 / 2 - it.dimensions.0 / 2) as i32,
                                     (current_y - it.dimensions.1) as i32,
                                     it.dimensions.0,
                                     it.dimensions.1)))
                .unwrap();
        }
        for (running_counter, item) in self.menu_items.iter().enumerate() {
            r.copy(&item.texture,
                      None,
                      Some(Rect::new(x as i32,
                                     current_y as i32,
                                     item.dimensions.0,
                                     item.dimensions.1)))
                .unwrap();
            if running_counter == self.currently_selected as usize {
                r.set_draw_color(RGB(255, 255, 255));
                r.fill_rect(Some(Rect::new(x as i32 - 20,
                                              current_y as i32,
                                              15,
                                              item.dimensions.1)))
                    .unwrap();
            }
            current_y += item.dimensions.1 + 2;
        }
    }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }
}
