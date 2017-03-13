use std::collections::HashMap;
use std::sync::Mutex;
use sdl2::render::Renderer;
use sdl2::pixels::Color::*;
use sdl2::keyboard::Keycode;

use engine::state::StateT;
use rand;

use msg::{Msg, Movement, GameCommand, MenuMsg};
use engine::data::EngineData;
use super::player::Player;
use super::edible::Edible;

lazy_static! {
    pub static ref MOVEMENT_MAPPING: Mutex<HashMap<Keycode, Movement>> = {
        let mut hm = HashMap::new();
        // TODO: do something with these invocations, probably a macro use?
        hm.insert(Keycode::Up, Movement::Up);
        hm.insert(Keycode::Down, Movement::Down);
        hm.insert(Keycode::Left, Movement::Left);
        hm.insert(Keycode::Right, Movement::Right);
        Mutex::new(hm)
    };
}

pub struct GameSettings {
    pub max_velocity: f32,
    pub deterioration_rate: f32,
    pub edible_deterioration_rate: f32,
    pub acceleration_rate: f32,
    pub edibles_spawn_rate: f32,
    pub edible_bounds: (u8, u8),
}

impl GameSettings {
    fn new() -> GameSettings {
        GameSettings {
            max_velocity: 30.0,
            deterioration_rate: 0.5,
            edible_deterioration_rate: 2.0,
            acceleration_rate: 75.0,
            edibles_spawn_rate: 3.0,
            edible_bounds: (15, 25),
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
        let coords = rand::random::<(u32, u32)>();
        let size = rand::random::<u8>() %
                   (self.settings.edible_bounds.1 - self.settings.edible_bounds.0);
        let x = (coords.0 % max_x) as i32;
        let y = (coords.1 % max_y) as i32;
        let edible = Edible::new(x, y, (self.settings.edible_bounds.0 + size) as f32);
        self.edibles.push(edible);
    }

    pub fn process_game_command(&mut self, c: GameCommand) -> Option<Msg> {
        match c {
            GameCommand::StartMovement(direction) => {
                match direction {
                    Movement::Up => {
                        self.player.direction.1 = -1;
                    }
                    Movement::Down => {
                        self.player.direction.1 = 1;
                    }
                    Movement::Left => {
                        self.player.direction.0 = -1;
                    }
                    Movement::Right => {
                        self.player.direction.0 = 1;
                    }
                }
                None
            }
            GameCommand::StopMovement(direction) => {
                match direction {
                    Movement::Up | Movement::Down => self.player.direction.1 = 0,
                    Movement::Left | Movement::Right => self.player.direction.0 = 0,
                }
                None
            }
            GameCommand::Pause => {
                self.running = false;
                None
            }
            GameCommand::Resume => {
                self.running = true;
                None
            }
            GameCommand::Menu => {
                self.running = false;
                Some(Msg::MenuCommand(MenuMsg::ShowGameMenu))
            }
        }
    }

    pub fn process_button_press(&mut self, k: Keycode) -> Option<Msg> {
        if let Some(direction) = MOVEMENT_MAPPING.lock().unwrap().get(&k) {
            self.process_game_command(GameCommand::StartMovement(*direction))
        } else {
            match k {
                Keycode::Escape => self.process_game_command(GameCommand::Menu),
                Keycode::P | Keycode::Pause => {
                    let is_running = self.running;
                    self.process_game_command({
                        if is_running {
                            GameCommand::Pause
                        } else {
                            GameCommand::Resume
                        }
                    })
                }
                _ => None,
            }
        }
    }

    pub fn process_button_release(&mut self, k: Keycode) -> Option<Msg> {
        if let Some(direction) = MOVEMENT_MAPPING.lock().unwrap().get(&k) {
            self.process_game_command(GameCommand::StopMovement(*direction))
        } else {
            None
        }
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
                        return Some(Msg::ShowGameOver);
                    }
                    self.edible_eta -= (x as f32) / 1000.0;
                    if self.edible_eta <= 0.0 {
                        self.spawn_edible(engine_data.window_size.0 - 25,
                                          engine_data.window_size.1 - 25);
                        self.edible_eta = self.settings.edibles_spawn_rate;
                    }
                    let mut to_remove = Vec::<usize>::new();
                    for edible_idx in 0..self.edibles.len() {
                        let edible = &mut self.edibles[edible_idx];
                        edible.deteriorate(self.settings.edible_deterioration_rate * (x as f32) /
                                           1000.0);
                        if edible.nutrition <= 0.0 {
                            to_remove.push(edible_idx);
                            continue;
                        }
                        if self.player.rect.intersection(edible.rect).is_some() {
                            self.player.size += edible.nutrition;
                            to_remove.push(edible_idx);
                        }
                    }
                    to_remove.sort();
                    to_remove.reverse();
                    for removing_idx in to_remove {
                        self.edibles.swap_remove(removing_idx);
                    }

                    if self.player.size >= (engine_data.window_size.1 as f32 / 2.0) {
                        return Some(Msg::ShowWinScreen);
                    }
                }
                Some(Msg::Tick(x))
            }
            Msg::MenuCommand(MenuMsg::ResumeGame) => {
                self.running = true;
                Some(Msg::MenuCommand(MenuMsg::ResumeGame))
            }
            Msg::Command(x) => self.process_game_command(x),
            // Buttons
            Msg::ButtonPressed(x) => self.process_button_press(x),
            Msg::ButtonReleased(x) => self.process_button_release(x),
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
