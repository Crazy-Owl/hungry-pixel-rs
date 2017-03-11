use sdl2::render::{Texture, Renderer};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::ttf::Font;
use sdl2::pixels::Color::*;
use msg::Msg;
use engine::data::EngineData;
use engine::state::StateT;

pub enum MenuPosition {
    Centered,
    Pos(u32, u32),
}

struct MenuItem {
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
}

impl<'m> MenuState {
    pub fn new(f: &Font<'m, 'static>,
               r: &mut Renderer,
               choices: Vec<(String, Msg)>,
               quit_on_esc: bool,
               position: MenuPosition,
               decoration_parameters: Option<(&Font<'m, 'static>, String)>,
               is_fullscreen: bool)
               -> MenuState {
        let mut menu_items = Vec::new();
        let mut max_width: u32 = 0;
        let mut max_height: u32 = 0;
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
            if query.width > max_width {
                max_width = query.width;
            }
            max_height += query.height;
        }
        let decoration_item = if let Some((fd, sd)) = decoration_parameters {
            let surface = fd.render(&sd).solid(RGB(255, 255, 255)).expect("Could not render text!");
            let texture = r.create_texture_from_surface(&surface).expect("Could not render text!");
            let query = texture.query();
            Some(MenuItem {
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
            on_escape: if quit_on_esc { Some(Msg::Exit) } else { None },
            position: position,
            decoration: decoration_item,
            is_fullscreen: is_fullscreen,
        }
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
}

impl StateT for MenuState {
    type Message = Msg;
    type EngineData = EngineData;

    fn process_message(&mut self, _: &mut EngineData, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::Tick(_) => None,
            Msg::ButtonPressed(keycode) => self.process_button(keycode),
            Msg::ButtonReleased(_) => None,
            msg => Some(msg),
        }
    }

    fn render(&mut self, r: &mut Renderer, ed: &EngineData) {
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
        let mut running_counter: usize = 0;
        for item in &self.menu_items {
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
            running_counter += 1;
        }
    }

    fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }
}
