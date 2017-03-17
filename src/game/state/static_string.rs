use sdl2::render::{Texture, Renderer};
use sdl2::rect::Rect;
use msg::Msg;
use engine::data::EngineData;
use engine::state::StateT;

pub struct ScreenLine {
    texture: Texture,
    dimensions: (u32, u32),
}

pub struct StaticState {
    time_left: i32,
    skippable: bool,
    lines: Vec<ScreenLine>,
    next_msg: Msg,
    dimensions: (u32, u32),
}

impl StaticState {
    pub fn new<'a, 'b>(strings: Vec<Texture>, pause: u32, next: Msg) -> StaticState {
        let mut lines: Vec<ScreenLine> = Vec::with_capacity(strings.len());
        let mut max_width: u32 = 0;
        let mut max_height: u32 = 0;

        for texture in strings {
            let query = texture.query();
            max_height += query.height;
            if query.width > max_width {
                max_width = query.width;
            }

            lines.push(ScreenLine {
                texture: texture,
                dimensions: (query.width, query.height),
            });
        }

        StaticState {
            time_left: pause as i32,
            skippable: false,
            lines: lines,
            next_msg: next,
            dimensions: (max_width, max_height),
        }
    }
}

impl StateT for StaticState {
    type Message = Msg;
    type EngineData = EngineData;

    fn process_message(&mut self, _: &mut EngineData, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::Tick(x) => {
                if !self.skippable {
                    self.time_left -= x as i32;
                    self.skippable = self.time_left <= 0;
                }
                None
            }
            Msg::ButtonPressed(_) => {
                if self.skippable {
                    Some(self.next_msg)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn render(&mut self, r: &mut Renderer, ed: &mut EngineData) {
        let mut current_y: u32 = (ed.window_size.1 / 2) - (self.dimensions.1 / 2);
        let x: u32 = (ed.window_size.0 / 2) - (self.dimensions.0 / 2);
        for scr_line in &self.lines {
            r.copy(&scr_line.texture,
                      None,
                      Some(Rect::new(x as i32,
                                     current_y as i32,
                                     scr_line.dimensions.0,
                                     scr_line.dimensions.1)))
                .unwrap();
            current_y += scr_line.dimensions.1;
        }
    }

    fn is_fullscreen(&self) -> bool {
        true
    }
}
