use sdl2::render::Renderer;
use sdl2::pixels::Color::*;
use sdl2::keyboard::Keycode;

use engine::state::StateT;
use rand;

use msg::{Msg, Control, MenuMsg};
use engine::data::EngineData;
use super::player::Player;
use super::edible::Edible;

pub struct GameSettings {
    pub max_velocity: f32,
    pub deterioration_rate: f32,
    pub edible_deterioration_rate: f32,
    pub acceleration_rate: f32,
    pub edibles_spawn_rate: f32,
    pub edible_bounds: (u32, u32),
}

impl GameSettings {
    fn new() -> GameSettings {
        GameSettings {
            max_velocity: 30.0,
            deterioration_rate: 0.5,
            edible_deterioration_rate: 0.25,
            acceleration_rate: 50.0,
            edibles_spawn_rate: 5.0,
            edible_bounds: (10, 45),
        }
    }
}

pub struct GameState {
    running: bool,
    player: Player,
    edible_eta: f32,
    edibles: Vec<Edible>,
    settings: GameSettings,
}

impl GameState {
    pub fn new() -> GameState {
        let settings = GameSettings::new();
        GameState {
            running: true,
            player: Player::new(),
            edible_eta: settings.edibles_spawn_rate,
            edibles: Vec::new(),
            settings: settings,
        }
    }

    pub fn spawn_edible(&mut self, max_x: u32, max_y: u32) {
        let coords = rand::random::<(u32, u32, u32)>();
        self.edibles.push(Edible::new((coords.0 % max_x) as i32,
                                      (coords.1 % max_y) as i32,
                                      (self.settings.edible_bounds.0 +
                                       (coords.2 %
                                        (self.settings.edible_bounds.1 -
                                         self.settings.edible_bounds.0))) as
                                      f32));
    }
}

impl StateT for GameState {
    type Message = Msg;
    type EngineData = EngineData;

    fn process_message(&mut self,
                       engine_data: &mut Self::EngineData,
                       msg: Self::Message)
                       -> Option<Self::Message> {
        match msg {
            Msg::Tick(x) => {
                if self.running {

                    if !self.player.process(x as f32, engine_data, &self.settings) {
                        return Some(Msg::Exit);
                    }
                    self.edible_eta -= (x as f32) / 1000.0;
                    if self.edible_eta <= 0.0 {
                        self.spawn_edible(engine_data.window_size.0 - 15,
                                          engine_data.window_size.1 - 15);
                        self.edible_eta = self.settings.edibles_spawn_rate;
                    }
                    let mut collisions = Vec::<usize>::new();
                    for edible_idx in 0..self.edibles.len() {
                        let edible = &mut self.edibles[edible_idx];
                        edible.deteriorate(self.settings.edible_deterioration_rate * (x as f32) /
                                           1000.0);
                        if let Some(_) = self.player.rect.intersection(edible.rect) {
                            self.player.size += edible.nutrition;
                            collisions.push(edible_idx);
                        }
                    }
                    collisions.sort();
                    collisions.reverse();
                    for collided in collisions {
                        self.edibles.swap_remove(collided);
                    }
                }
                Some(Msg::Tick(x))
            }
            Msg::ControlCommand(Control::Pause) => {
                self.running = !self.running;
                None
            }
            Msg::ControlCommand(Control::Escape) => {
                self.running = false;
                Some(Msg::MenuCommand(MenuMsg::ShowGameMenu))
            }
            Msg::MenuCommand(MenuMsg::ResumeGame) => {
                self.running = true;
                Some(Msg::MenuCommand(MenuMsg::ResumeGame))
            }
            Msg::ButtonPressed(Keycode::Up) => {
                self.player.direction.1 = -1i8;
                None
            }
            Msg::ButtonPressed(Keycode::Down) => {
                self.player.direction.1 = 1i8;
                None
            }
            Msg::ButtonPressed(Keycode::Left) => {
                self.player.direction.0 = -1i8;
                None
            }
            Msg::ButtonPressed(Keycode::Right) => {
                self.player.direction.0 = 1i8;
                None
            }
            Msg::ButtonReleased(Keycode::Up) => {
                self.player.direction.1 = 0;
                None
            }
            Msg::ButtonReleased(Keycode::Down) => {
                self.player.direction.1 = 0;
                None
            }
            Msg::ButtonReleased(Keycode::Left) => {
                self.player.direction.0 = 0;
                None
            }
            Msg::ButtonReleased(Keycode::Right) => {
                self.player.direction.0 = 0;
                None
            }
            Msg::NoOp => None,
            msg => Some(msg),
        }
    }

    fn render(&mut self, r: &mut Renderer, _: &EngineData) {
        r.set_draw_color(RGB(0, 255, 0));
        // get player left upper corner coordinates
        // TODO: proper handling, just player x, y for now
        // this will cause strange behavior, and should be eliminated
        r.fill_rect(Some(self.player.rect))
            .unwrap();
        r.set_draw_color(RGB(255, 128, 0));
        for edible in &self.edibles {
            r.fill_rect(Some(edible.rect))
                .unwrap();
        }
    }

    fn is_fullscreen(&self) -> bool {
        true
    }
}
