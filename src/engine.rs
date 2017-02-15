use std::collections::linked_list::LinkedList;
use sdl2::{EventPump, VideoSubsystem, TimerSubsystem};
use sdl2::render::Renderer;
use sdl2::video::Window;
use sdl2::ttf::Font;

use super::model::Model;
use super::Msg;
use super::SDL2Context;
use super::state;
use super::resources;

const FPS_LOCK: u32 = 1000 / 64;
/// Game Engine

/// Holds all the data relevant to establishing the main game loop, to process SDL events
/// (keyboard and mouse) etc.
pub struct Engine<'a> {
    pub model: Model,
    pub context: &'a SDL2Context,
    /// LinkedList for in-game messages
    pub messages: LinkedList<Msg>,
    pub event_pump: EventPump,
    /// Renderer with static runtime since it corresponds to the window
    pub renderer: Renderer<'static>,
    pub timer: TimerSubsystem,
    pub font: Font<'a>, // TODO: provide a font cache (just like image cache)
    /// last update timestamp in SDL2 internal milliseconds
    pub last_update: u32,
    pub current_state: Option<Box<state::StateT<Model = Model, Message = Msg>>>,
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

        Engine {
            model: Model::new(),
            context: sdl_context,
            messages: LinkedList::new(),
            event_pump: event_pump,
            renderer: renderer,
            timer: timer,
            font: font,
            last_update: ticks,
            current_state: None,
        }
    }
}

impl<'a> TEngine for Engine<'a> {
    type Message = Msg;
    type Model = Model;

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        match msg {
            Msg::NoOp => None,
            Msg::Exit => {
                self.model.running = false;
                None
            }
            Msg::Change(x) => {
                self.model.message = x;
                Some(Msg::Exit)
            }
            Msg::Tick(x) => {
                println!("{}", x);
                None
            }
        }
    }

    fn render(&mut self) {
        self.renderer.clear();
        self.renderer.present();
    }

    fn process(&mut self) -> bool {
        let ticks_at_start = self.timer.ticks();

        for event in self.event_pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode::*;

            match event {
                Quit { .. } |
                KeyDown { keycode: Some(Escape), .. } => self.messages.push_back(Msg::Exit),
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
