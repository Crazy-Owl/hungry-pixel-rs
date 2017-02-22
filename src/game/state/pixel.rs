use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color::*;

use engine::state::StateT;
use rand;

use msg::{Msg, ControlCommand};
use model::Model;

pub struct GameSettings {
    max_velocity: f32,
    deterioration_rate: f32,
    edible_deterioration_rate: f32,
    acceleration_rate: f32,
    edibles_spawn_rate: f32,
    edible_bounds: (u32, u32),
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

#[derive(Debug)]
pub struct Player {
    x: f32,
    y: f32,
    rect: Rect,
    size: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 0.0,
            y: 0.0,
            rect: Rect::new(0, 0, 20, 20),
            size: 20.0,
        }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        self.rect.set_x(x as i32);
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
        self.rect.set_y(y as i32);
    }

    pub fn offset(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
        self.rect.reposition((self.x as i32, self.y as i32));
    }

    pub fn resize(&mut self, d_size: f32) {
        self.size += d_size;
        self.rect.resize(self.size as u32, self.size as u32);
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
    player_speed: (f32, f32),
    player_direction: (i8, i8),
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
            player_speed: (0.0, 0.0),
            player_direction: (0, 0),
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
                    self.player.offset(self.player_speed.0 * (x as f32) / 1000.0,
                                       self.player_speed.1 * (x as f32) / 1000.0);
                    self.player.resize(-self.settings.deterioration_rate * (x as f32) / 1000.0);
                    if self.player.size <= 1.0 {
                        println!("Sorry, you've lost!");
                        return Some(Msg::Exit);
                    }
                    self.player_speed.0 +=
                        (self.player_direction.0 as f32) * self.settings.acceleration_rate *
                        (x as f32) / 1000.0;
                    self.player_speed.1 +=
                        (self.player_direction.1 as f32) * self.settings.acceleration_rate *
                        (x as f32) / 1000.0;

                    if self.player.x < 0.0 {
                        self.player.set_x(0.0);
                        self.player_speed.0 = -self.player_speed.0;
                    }

                    if self.player.x > (model.window_size.0 as f32) - self.player.size {
                        let new_x = model.window_size.0 as f32 - self.player.size;
                        self.player
                            .set_x(new_x);
                        self.player_speed.0 = -self.player_speed.0;
                    }

                    if self.player.y < 0.0 {
                        self.player.set_y(0.0);
                        self.player_speed.1 = -self.player_speed.1;
                    }

                    if self.player.y > (model.window_size.1 as f32) - self.player.size {
                        let new_y = model.window_size.1 as f32 - self.player.size;
                        self.player
                            .set_y(new_y);
                        self.player_speed.1 = -self.player_speed.1;
                    }

                    self.edible_eta -= (x as f32) / 1000.0;
                    if self.edible_eta <= 0.0 {
                        self.spawn_edible(model.window_size.0 - 15, model.window_size.1 - 15);
                        self.edible_eta = self.settings.edibles_spawn_rate;
                    }
                    let mut collisions = Vec::<usize>::new();
                    for edible_idx in 0..self.edibles.len() {
                        let edible = &mut self.edibles[edible_idx];
                        edible.deteriorate(self.settings.edible_deterioration_rate * (x as f32) / 1000.0);
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
        r.draw_rect(self.player.rect)
            .unwrap();
        r.set_draw_color(RGB(255, 128, 0));
        for edible in &self.edibles {
            r.draw_rect(edible.rect)
                .unwrap();
        }
    }
}
