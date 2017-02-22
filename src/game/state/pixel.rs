use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color::*;

use engine::state::StateT;
use rand;

use msg::{Msg, ControlCommand};
use model::Model;
use super::player::Player;

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


pub struct Edible {
    rect: Rect,
    nutrition: f32,
}

impl Edible {
    pub fn new(x: i32, y: i32, nutrition: f32) -> Edible {
        Edible {
            rect: Rect::new(x, y, nutrition as u32, nutrition as u32),
            nutrition: nutrition,
        }
    }

    pub fn deteriorate(&mut self, x: f32) {
        self.nutrition -= x;
        self.rect.resize(self.nutrition as u32, self.nutrition as u32);
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
    type Model = Model;

    fn process_message(&mut self,
                       model: &mut Self::Model,
                       msg: Self::Message)
                       -> Option<Self::Message> {
        match msg {
            Msg::Tick(x) => {
                if self.running {

                    if !self.player.process(x as f32, model, &self.settings) {
                        return Some(Msg::Exit);
                    }
                    self.edible_eta -= (x as f32) / 1000.0;
                    if self.edible_eta <= 0.0 {
                        self.spawn_edible(model.window_size.0 - 15, model.window_size.1 - 15);
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
            Msg::ButtonPressed(ControlCommand::Escape) => Some(Msg::Exit),
            Msg::ButtonPressed(ControlCommand::Up) => {
                self.player.direction.1 = -1i8;
                None
            }
            Msg::ButtonPressed(ControlCommand::Down) => {
                self.player.direction.1 = 1i8;
                None
            }
            Msg::ButtonPressed(ControlCommand::Left) => {
                self.player.direction.0 = -1i8;
                None
            }
            Msg::ButtonPressed(ControlCommand::Right) => {
                self.player.direction.0 = 1i8;
                None
            }
            Msg::ButtonReleased(ControlCommand::Up) => {
                self.player.direction.1 = 0;
                None
            }
            Msg::ButtonReleased(ControlCommand::Down) => {
                self.player.direction.1 = 0;
                None
            }
            Msg::ButtonReleased(ControlCommand::Left) => {
                self.player.direction.0 = 0;
                None
            }
            Msg::ButtonReleased(ControlCommand::Right) => {
                self.player.direction.0 = 0;
                None
            }
            _ => None,
        }
    }

    fn render(&mut self, r: &mut Renderer) {
        r.set_draw_color(RGB(0, 255, 0));
        // get player left upper corner coordinates
        // TODO: proper handling, just player x, y for now
        // this will cause strange behavior, and should be eliminated
        r.draw_rect(self.player.rect)
            .unwrap();
        r.set_draw_color(RGB(255, 128, 0));
        for edible in &self.edibles {
            r.draw_rect(edible.rect)
                .unwrap();
        }
    }
}
