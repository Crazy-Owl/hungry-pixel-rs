extern crate sdl2;

use std::collections::linked_list::LinkedList;
use sdl2::{EventPump, Sdl, VideoSubsystem, TimerSubsystem};
use sdl2::render::Renderer;
use sdl2::video::Window;

pub mod resources;

/// Game Engine
/// Holds all the data relevant to establishing the main game loop, to process SDL events
/// (keyboard and mouse) etc.
pub struct Engine {
    pub model: Model,
    /// LinkedList for in-game messages
    pub messages: LinkedList<Msg>,
    pub event_pump: EventPump,
    /// Renderer with static runtime since it corresponds to the window
    pub renderer: Renderer<'static>,
    pub timer: TimerSubsystem,
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

impl Engine {
    pub fn new() -> Engine {
        let sdl_context: Sdl = sdl2::init().expect("Could not initialize SDL!");
        let event_pump: EventPump = sdl_context.event_pump().unwrap();
        let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();
        let timer: TimerSubsystem = sdl_context.timer().unwrap();
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

        Engine {
            model: Model::new(),
            messages: LinkedList::new(),
            event_pump: event_pump,
            renderer: renderer,
            timer: timer,
        }
    }
}

impl TEngine for Engine {
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
        }
    }

    fn render(&mut self) {
        self.renderer.clear();
        self.renderer.present();
    }

    fn process(&mut self) -> bool {

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
        self.model.running
    }
}

/// Model
/// For now it just holds the message to display and running state of the game
#[derive(Debug)]
pub struct Model {
    pub running: bool,
    pub message: String,
}

impl Model {
    pub fn new() -> Model {
        Model {
            running: true,
            message: "Hello world".to_string(),
        }
    }
}

// Msg

#[derive(Debug)]
pub enum Msg {
    NoOp,
    Exit,
    Change(String),
}
