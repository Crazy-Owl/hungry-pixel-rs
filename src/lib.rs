extern crate sdl2;

use std::collections::linked_list::LinkedList;
use sdl2::{EventPump, Sdl, VideoSubsystem, TimerSubsystem};
use sdl2::render::Renderer;
use sdl2::video::Window;

pub mod resources;

// Game Engine

pub struct Engine {
    pub model: Model,
    pub messages: LinkedList<Msg>,
    pub event_pump: EventPump,
    pub renderer: Renderer<'static>,
    pub timer: TimerSubsystem,
}

/// Basic trait for all game engines.
pub trait TEngine {
    type Message;
    type Model;

    fn update(&mut self, msg: Self::Message) -> Option<Self::Message>;
    fn render(&self);
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
                self.model.state = false;
                None
            }
            Msg::Change(x) => {
                self.model.message = x;
                Some(Msg::Exit)
            }
        }
    }

    fn render(&self) {
        println!("{}", &self.model.message);
    }

    fn process(&mut self) -> bool {
        self.render();
        if let Some(msg) = self.messages.pop_front() {
            self.update(msg).map(|m| self.messages.push_back(m));
        }
        self.model.state
    }
}

// Model

#[derive(Debug)]
pub struct Model {
    pub state: bool,
    pub message: String,
}

impl Model {
    pub fn new() -> Model {
        Model {
            state: true,
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
