use std::collections::HashMap;
use std::sync::Mutex;
use sdl2::render::Renderer;
use sdl2::pixels::Color::*;
use sdl2::keyboard::Keycode;

use engine::state::StateT;
use msg::{Msg, Movement, GameCommand, MenuMsg};
use engine::data::EngineData;
use super::player::Player;
use super::edible::Edible;
use super::spike::Spike;

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
    pub spikes_spawn_rate: f32,
    pub spikes_bounds: (u32, u32),
}

impl GameSettings {
    fn new() -> GameSettings {
        GameSettings {
            max_velocity: 30.0,
            deterioration_rate: 0.75,
            edible_deterioration_rate: 2.0,
            acceleration_rate: 100.0,
            edibles_spawn_rate: 3.0,
            edible_bounds: (15, 25),
            spikes_spawn_rate: 10.0,
            spikes_bounds: (15, 55),
        }
    }
}

pub struct GameState {
    running: bool,
    player: Player,
    edible_eta: f32,
    edibles: Vec<Edible>,
    settings: GameSettings,
    spike_eta: f32,
    spikes: Vec<Spike>,
}

impl GameState {
    pub fn new() -> GameState {
        let settings = GameSettings::new();
        GameState {
            running: true,
            player: Player::new(),
            edible_eta: settings.edibles_spawn_rate,
            edibles: Vec::new(),
            spike_eta: settings.spikes_spawn_rate,
            settings: settings,
            spikes: Vec::new(),
        }
    }

    pub fn spawn_edible(&mut self, max_x: u32, max_y: u32) {
        self.edibles.push(Edible::random(max_x,
                                         max_y,
                                         self.settings.edible_bounds.0 as f32,
                                         self.settings.edible_bounds.1 as f32));
    }

    pub fn spawn_spike(&mut self, max_x: i32, max_y: i32, min_size: u32, max_size: u32) {
        self.spikes.push(Spike::random(max_x, max_y, min_size, max_size));
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
                    self.spike_eta -= (x as f32) / 1000.0;
                    if self.spike_eta <= 0.0 {
                        let spikes_bounds = self.settings.spikes_bounds;
                        self.spawn_spike(engine_data.window_size.0 as i32,
                                         engine_data.window_size.1 as i32,
                                         spikes_bounds.0,
                                         spikes_bounds.1);
                        self.spike_eta = self.settings.spikes_spawn_rate;
                    }
                    let mut to_remove_edibles = Vec::<usize>::new();
                    for edible_idx in 0..self.edibles.len() {
                        let edible = &mut self.edibles[edible_idx];
                        edible.deteriorate(self.settings.edible_deterioration_rate * (x as f32) /
                                           1000.0);
                        if edible.nutrition <= 0.0 {
                            to_remove_edibles.push(edible_idx);
                            continue;
                        }
                        if self.player.rect.intersection(edible.rect).is_some() {
                            self.player.size += edible.nutrition;
                            to_remove_edibles.push(edible_idx);
                        }
                    }

                    let mut to_remove_spikes = Vec::<usize>::new();
                    for spike_idx in 0..self.spikes.len() {
                        let spike = &mut self.spikes[spike_idx];
                        spike.update(x as f32 / 1000.0, (engine_data.window_size.0 as f32, engine_data.window_size.1 as f32));
                        if self.player.rect.intersection(spike.rect).is_some() {
                            if self.player.size >= 40.0 {
                                self.player.size -= 20.0;
                            } else {
                                self.player.size -= 0.5 * self.player.size;
                            }
                            to_remove_spikes.push(spike_idx);
                        }
                    }
                    to_remove_edibles.sort();
                    to_remove_edibles.reverse();
                    for removing_idx in to_remove_edibles {
                        self.edibles.swap_remove(removing_idx);
                    }

                    to_remove_spikes.sort();
                    to_remove_spikes.reverse();
                    for removing_idx in to_remove_spikes {
                        self.spikes.swap_remove(removing_idx);
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

    fn render(&mut self, r: &mut Renderer, _: &mut EngineData) {
        r.set_draw_color(RGB(0, 255, 0));
        // get player left upper corner coordinates
        // TODO: proper handling, just player x, y for now
        // this will cause strange behavior, and should be eliminated
        r.fill_rect(Some(self.player.rect))
            .unwrap();
        r.set_draw_color(RGB(255, 128, 0));
        for edible in &self.edibles {
            r.fill_rect(Some(edible.rect)).unwrap();
        }
        r.set_draw_color(RGB(255, 0, 0));
        for spike in &self.spikes {
            r.fill_rect(Some(spike.rect)).unwrap();
        }
    }

    fn is_fullscreen(&self) -> bool {
        true
    }
}
