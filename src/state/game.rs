use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color::*;

use super::StateT;

use msg::{Msg, ControlCommand};
use model::Model;

pub struct Edible(f32, f32, f32);

pub struct GameSettings {
    max_velocity: f32,
    deterioration_rate: f32,
    acceleration_rate: f32,
    edibles_spawn_rate: f32,
}

impl GameSettings {
    fn new() -> GameSettings {
        GameSettings {
            max_velocity: 30.0,
            deterioration_rate: 0.5,
            acceleration_rate: 50.0,
            edibles_spawn_rate: 5.0,
        }
    }
}

pub struct GameState {
    running: bool,
    player: (f32, f32, f32),
    player_speed: (f32, f32),
    player_direction: (i8, i8),
    edible_eta: f32,
    items: Vec<Edible>,
    settings: GameSettings,
}

impl GameState {
    pub fn new() -> GameState {
        let settings = GameSettings::new();
        GameState {
            running: true,
            player: (0.0, 0.0, 20.0),
            player_speed: (0.0, 0.0),
            player_direction: (0, 0),
            edible_eta: settings.edibles_spawn_rate,
            items: Vec::new(),
            settings: settings,
        }
    }

    pub fn spawn_edible(&mut self) {}
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
                    self.player.0 += self.player_speed.0 * (x as f32) / 1000.0;
                    self.player.1 += self.player_speed.1 * (x as f32) / 1000.0;
                    self.player.2 -= self.settings.deterioration_rate * (x as f32) / 1000.0;
                    self.player_speed.0 +=
                        (self.player_direction.0 as f32) * self.settings.acceleration_rate *
                        (x as f32) / 1000.0;
                    self.player_speed.1 +=
                        (self.player_direction.1 as f32) * self.settings.acceleration_rate *
                        (x as f32) / 1000.0;
                    println!("{:?}", self.player);
                }
                Some(Msg::Tick(x))
            }
            Msg::ButtonPressed(ControlCommand::Escape) => Some(Msg::Exit),
            Msg::ButtonPressed(ControlCommand::Up) => {
                self.player_direction.1 = -1i8;
                None
            }
            Msg::ButtonPressed(ControlCommand::Down) => {
                self.player_direction.1 = 1i8;
                None
            }
            Msg::ButtonPressed(ControlCommand::Left) => {
                self.player_direction.0 = -1i8;
                None
            }
            Msg::ButtonPressed(ControlCommand::Right) => {
                self.player_direction.0 = 1i8;
                None
            }
            Msg::ButtonReleased(ControlCommand::Up) => {
                self.player_direction.1 = 0;
                None
            }
            Msg::ButtonReleased(ControlCommand::Down) => {
                self.player_direction.1 = 0;
                None
            }
            Msg::ButtonReleased(ControlCommand::Left) => {
                self.player_direction.0 = 0;
                None
            }
            Msg::ButtonReleased(ControlCommand::Right) => {
                self.player_direction.0 = 0;
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
        r.draw_rect(Rect::new(self.player.0 as i32,
                                 self.player.1 as i32,
                                 self.player.2 as u32,
                                 self.player.2 as u32))
            .unwrap()
    }
}
