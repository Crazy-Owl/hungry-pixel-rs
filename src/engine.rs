use std::collections::linked_list::LinkedList;
use std::collections::hash_set::HashSet;
use sdl2::{EventPump, VideoSubsystem, TimerSubsystem};
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::ttf::Font;
use sdl2::pixels::Color::RGB;
use sdl2::keyboard::Keycode;

use super::model::Model;
use super::msg::{Msg, ControlCommand};
use super::SDL2Context;
use super::state::StateT;
use super::state::game::GameState;
use super::resources;

const FPS_LOCK: u32 = 1000 / 64;
/// Game Engine

/// Holds all the data relevant to establishing the main game loop, to process SDL events
/// (keyboard and mouse) etc.
pub struct Engine<'m> {
    pub model: Model,
    pub context: &'m SDL2Context,
    /// LinkedList for in-game messages
    pub messages: LinkedList<Msg>,
    pub event_pump: EventPump,
    /// Renderer with static runtime since it corresponds to the window
    pub renderer: Renderer<'static>,
    pub timer: TimerSubsystem,
    pub font: Font<'m, 'static>, // TODO: provide a font cache (just like image cache)
    /// last update timestamp in SDL2 internal milliseconds
    pub last_update: u32,
    pub current_state: Option<Box<StateT<Model = Model, Message = Msg>>>,
    marked_events: HashSet<Keycode>
}

/// Basic trait for all game engines.
/// Has two associated types - message and model
pub trait TEngine {
    /// Message type is what we basically send around
    type Message;
    /// Model is what holds all the game state inside
    type Model;

    /// Update takes a single message, processes it and then optionally puts another one in queue
    fn update(&mut self, msg: Self::Message) -> Option<Self::Message>;
    /// Render the model on screen
    fn render(&mut self);
    /// This is where we handle all the "subscriptions" like external events:
    /// for example, tick should occur here, SDL2 event processing should occur here.
    /// Returned bool indicates whether the game should stop running.
    fn process(&mut self) -> bool;
}

impl<'a> Engine<'a> {
    pub fn new(sdl_context: &'a mut SDL2Context) -> Engine<'a> {
        let event_pump: EventPump = sdl_context.sdl2.event_pump().unwrap();
        let video_subsystem: VideoSubsystem = sdl_context.sdl2.video().unwrap();
        let mut timer: TimerSubsystem = sdl_context.sdl2.timer().unwrap();
        let font: Font = sdl_context.ttf
            .load_font(resources::get_resource_path("PressStart2P-Regular.ttf"), 14)
            .unwrap();
        let window: Window = video_subsystem.window("SDL2 game", 800, 600)
            .position_centered()
            .opengl()
            .allow_highdpi()
            .build()
            .expect("Could not create window!");

        let renderer: Renderer<'static> = window.renderer()
            .accelerated()
            .build()
            .expect("Could not aquire renderer");

        let ticks = timer.ticks();

        let game_state = GameState::new();

        Engine {
            model: Model::new(),
            context: sdl_context,
            messages: LinkedList::new(),
            event_pump: event_pump,
            renderer: renderer,
            timer: timer,
            font: font,
            last_update: ticks,
            current_state: Some(Box::new(game_state)),
            marked_events: HashSet::new(),
        }
    }
}

impl<'a> TEngine for Engine<'a> {
    type Message = Msg;
    type Model = Model;

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        let state_result: Option<Msg> = match self.current_state {
            // TODO: actually process the result of `process_message` fn
            Some(ref mut state) => state.as_mut().process_message(&mut self.model, msg),
            None => None,
        };
        match state_result {
            Some(Msg::NoOp) => None,
            Some(Msg::Exit) => {
                self.model.running = false;
                None
            }
            _ => None,
        }
    }

    fn render(&mut self) {
        self.renderer.set_draw_color(RGB(0, 0, 0));
        self.renderer.clear();
        if let Some(ref mut state) = self.current_state {
            state.render(&mut self.renderer);
        }
        self.renderer.present();
    }

    fn process(&mut self) -> bool {
        let ticks_at_start = self.timer.ticks();

        self.marked_events.drain();

        for event in self.event_pump.poll_iter() {
            use sdl2::event::Event::*;

            if let KeyDown {keycode: x, ..} = event {
                if self.marked_events.contains(&(x.unwrap())) {
                    continue;
                } else {
                    self.marked_events.insert(x.unwrap());
                }
            }

            match event {
                Quit { .. } |
                KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    println!("Escape pressed");
                    self.messages.push_back(Msg::ButtonPressed(ControlCommand::Escape))
                }
                KeyDown { keycode: Some(Keycode::Up), .. } => {
                    self.messages.push_back(Msg::ButtonPressed(ControlCommand::Up))
                }
                KeyDown { keycode: Some(Keycode::Down), .. } => {
                    self.messages.push_back(Msg::ButtonPressed(ControlCommand::Down))
                }
                KeyDown { keycode: Some(Keycode::Left), .. } => {
                    self.messages.push_back(Msg::ButtonPressed(ControlCommand::Left))
                }
                KeyDown { keycode: Some(Keycode::Right), .. } => {
                    self.messages.push_back(Msg::ButtonPressed(ControlCommand::Right))
                }
                KeyUp { keycode: Some(Keycode::Up), .. } => {
                    self.messages.push_back(Msg::ButtonReleased(ControlCommand::Up))
                }
                KeyUp { keycode: Some(Keycode::Down), .. } => {
                    self.messages.push_back(Msg::ButtonReleased(ControlCommand::Down))
                }
                KeyUp { keycode: Some(Keycode::Left), .. } => {
                    self.messages.push_back(Msg::ButtonReleased(ControlCommand::Left))
                }
                KeyUp { keycode: Some(Keycode::Right), .. } => {
                    self.messages.push_back(Msg::ButtonReleased(ControlCommand::Right))
                }
                _ => {}
            }
        }

        self.render();
        if let Some(msg) = self.messages.pop_front() {
            self.update(msg).map(|m| self.messages.push_back(m));
        }
        let ticks_at_finish = self.timer.ticks();
        if ticks_at_finish - ticks_at_start < FPS_LOCK {
            self.timer.delay(FPS_LOCK - (ticks_at_finish - ticks_at_start));
        }
        self.messages.push_back(Msg::Tick(self.timer.ticks() - ticks_at_start));
        self.model.running
    }
}
