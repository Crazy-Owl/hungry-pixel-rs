pub mod state;
pub mod data;
pub mod context;
pub mod font;

use std::collections::{HashSet, VecDeque};
use sdl2::{EventPump, VideoSubsystem, TimerSubsystem};
use sdl2::render::{Renderer, Texture};
use sdl2::video::Window;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::pixels::Color::RGB;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use self::data::EngineData;
use super::msg::{Msg, MenuMsg};
use engine::context::SDL2Context;
use self::state::StateT;
use game::state::pixel::GameState;
use game::state::menu::{MenuState, MenuPosition};
use game::state::static_string::StaticState;
use game::state::options::OptionsState;
use self::font::FontCache;
use super::resources;


const FPS_LOCK: u32 = 1000 / 64;

/// Game Engine

/// Holds all the data relevant to establishing the main game loop, to process SDL events
/// (keyboard and mouse) etc.
pub struct Engine {
    pub engine_data: EngineData,
    pub context: SDL2Context,
    /// LinkedList for in-game messages
    pub messages: VecDeque<Msg>,
    pub event_pump: EventPump,
    /// Renderer with static runtime since it corresponds to the window
    pub renderer: Renderer<'static>,
    pub timer: TimerSubsystem,
    /// last update timestamp in SDL2 internal milliseconds
    pub last_update: u32,
    pub states_stack: Vec<Box<StateT<EngineData = EngineData, Message = Msg>>>,
    marked_events: HashSet<Keycode>,
}

/// Basic trait for all game engines.
/// Has two associated types - message and model
pub trait TEngine {
    /// Message type is what we basically send around
    type Message;
    /// Model is what holds all the game state inside
    type EngineData;

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
    pub fn new(sdl_context: SDL2Context, ttf_context: Sdl2TtfContext) -> Engine {
        let event_pump: EventPump = sdl_context.sdl2.event_pump().unwrap();
        let video_subsystem: VideoSubsystem = sdl_context.sdl2.video().unwrap();
        let mut timer: TimerSubsystem = sdl_context.sdl2.timer().unwrap();
        let font_cache = FontCache::new(ttf_context);
        let mut engine_data = EngineData::new(font_cache);
        let window: Window = video_subsystem.window("SDL2 game",
                    engine_data.window_size.0,
                    engine_data.window_size.1)
            .position_centered()
            .resizable()
            .opengl()
            .allow_highdpi()
            .build()
            .expect("Could not create window!");

        let mut renderer: Renderer<'static> = window.renderer()
            .accelerated()
            .build()
            .expect("Could not aquire renderer");

        engine_data.font_cache.load_font(&mut renderer,
                                         "default",
                                         resources::get_resource_path("PressStart2P-Regular.ttf"),
                                         14);
        engine_data.font_cache.load_font(&mut renderer,
                                         "default-large",
                                         resources::get_resource_path("PressStart2P-Regular.ttf"),
                                         24);

        renderer.set_logical_size(engine_data.window_size.0, engine_data.window_size.1)
            .expect("Could not set logical size of renderer!");

        let ticks = timer.ticks();

        Engine {
            engine_data: engine_data,
            context: sdl_context,
            messages: VecDeque::new(),
            event_pump: event_pump,
            renderer: renderer,
            timer: timer,
            last_update: ticks,
            states_stack: vec![],
            marked_events: HashSet::new(),
        }
    }

    fn in_game_menu(&mut self) -> Box<MenuState> {
        let choices: Vec<(Texture, Msg)> =
            vec![(self.engine_data
                      .font_cache
                      .render_texture(&mut self.renderer, "default", "Resume", None)
                      .unwrap(),
                  Msg::MenuCommand(MenuMsg::ResumeGame)),
                 (self.engine_data
                      .font_cache
                      .render_texture(&mut self.renderer, "default", "Exit to main Menu", None)
                      .unwrap(),
                  Msg::MenuCommand(MenuMsg::ToMainMenu))];

        let pause_texture = self.engine_data
            .font_cache
            .render_texture(&mut self.renderer, "default-large", "PAUSE", None)
            .unwrap();

        Box::new(MenuState::new(&mut self.renderer,
                                choices,
                                Some(Msg::MenuCommand(MenuMsg::ResumeGame)),
                                MenuPosition::Centered,
                                Some((pause_texture, "PAUSE".to_string())),
                                false))
    }

    fn main_menu(&mut self) -> Box<MenuState> {
        let choices: Vec<(Texture, Msg)> =
            vec![(self.engine_data
                      .font_cache
                      .render_texture(&mut self.renderer, "default", "New Game", None)
                      .unwrap(),
                  Msg::StartGame),
                 (self.engine_data
                      .font_cache
                      .render_texture(&mut self.renderer, "default", "Controls", None)
                      .unwrap(),
                  Msg::ShowOptions),
                 (self.engine_data
                      .font_cache
                      .render_texture(&mut self.renderer, "default", "Credits", None)
                      .unwrap(),
                  Msg::ShowCredits),
                 (self.engine_data
                      .font_cache
                      .render_texture(&mut self.renderer, "default", "Exit Game", None)
                      .unwrap(),
                  Msg::Exit)];

        let title_texture = self.engine_data
            .font_cache
            .render_texture(&mut self.renderer, "default-large", "HUNGRY PIXEL", None)
            .unwrap();

        Box::new(MenuState::new(&mut self.renderer,
                                choices,
                                None,
                                MenuPosition::Centered,
                                Some((title_texture, "HUNGRY PIXEL".to_string())),
                                true))
    }

    fn intro_screen(&mut self) -> Box<StaticState> {
        self.engine_data
            .font_cache
            .render_texture(&mut self.renderer,
                            "default",
                            "This is a game about a pixel who is very hungry.",
                            None)
            .unwrap();
        let textures: Vec<Texture> =
            vec![self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer,
                                     "default",
                                     "This is a game about a pixel who is very hungry.",
                                     None)
                     .unwrap(),
                 self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer, "default", "So he eats...", None)
                     .unwrap(),
                 self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer, "default", "And eats...", None)
                     .unwrap(),
                 self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer,
                                     "default",
                                     "He eats so much that he grows into a square!..", None)
                     .unwrap()];
        Box::new(StaticState::new(textures, 1000, Msg::MenuCommand(MenuMsg::ToMainMenu)))
    }

    fn gameover_screen(&mut self) -> Box<StaticState> {
        let textures: Vec<Texture> =
            vec![self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer, "default-large", "GAME OVER", None)
                     .unwrap(),
                 self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer, "default-large", "Unfortunately.", None)
                     .unwrap()];
        Box::new(StaticState::new(textures, 1000, Msg::MenuCommand(MenuMsg::ToMainMenu)))
    }

    fn winning_screen(&mut self) -> Box<StaticState> {
        let textures: Vec<Texture> =
            vec![self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer, "default-large", "Congratulations!", None)
                     .unwrap(),
                 self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer, "default-large", "You've won!", None)
                     .unwrap()];
        Box::new(StaticState::new(textures, 1000, Msg::ShowCredits))
    }

    fn credits(&mut self) -> Box<StaticState> {
        let textures: Vec<Texture> =
            vec![self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer, "default-large", "Author:", None)
                     .unwrap(),
                 self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer, "default-large", "Crazy-Owl", None)
                     .unwrap(),
                 self.engine_data
                     .font_cache
                     .render_texture(&mut self.renderer,
                                     "default-large",
                                     "http://GitHub.com/Crazy-Owl", Some((0, 255, 0, 0)))
                     .unwrap()];
        Box::new(StaticState::new(textures, 1500, Msg::MenuCommand(MenuMsg::ToMainMenu)))
    }

    fn options(&mut self) -> Box<OptionsState> {
        Box::new(OptionsState::new(&mut self.engine_data.font_cache, &mut self.renderer))
    }

    pub fn start_game(&mut self) {
        let intro_screen = self.intro_screen();
        self.states_stack.push(intro_screen);
    }
}

impl TEngine for Engine {
    type Message = Msg;
    type EngineData = EngineData;

    fn update(&mut self, msg: Msg) -> Option<Msg> {
        let mut current_msg = Some(msg);
        'stack_propagation: for index in (0..self.states_stack.len()).rev() {
            match (&mut self.states_stack[index])
                .process_message(&mut self.engine_data, current_msg.unwrap()) {
                None => {
                    current_msg = None;
                    break 'stack_propagation;
                }
                Some(message) => {
                    current_msg = Some(message);
                    continue 'stack_propagation;
                }
            }
        }
        match current_msg {
            Some(Msg::StartGame) => {
                let game_state = GameState::new();
                self.states_stack.push(Box::new(game_state));
                None
            }
            Some(Msg::MenuCommand(MenuMsg::ToMainMenu)) => {
                let menu = self.main_menu();
                let drain_range = ..self.states_stack.len();
                self.states_stack.drain(drain_range);
                self.states_stack.push(menu);
                None
            }
            Some(Msg::MenuCommand(MenuMsg::ShowGameMenu)) => {
                let menu = self.in_game_menu();
                self.states_stack.push(menu);
                None
            }
            Some(Msg::PopState(x)) => {
                for _ in 0..x {
                    self.states_stack.pop();
                }
                None
            }
            Some(Msg::ShowGameOver) => {
                let drain_range = ..self.states_stack.len();
                self.states_stack.drain(drain_range);
                let gameover_screen = self.gameover_screen();
                self.states_stack.push(gameover_screen);
                None
            }
            Some(Msg::ShowOptions) => {
                let options = self.options();
                self.states_stack.push(options);
                None
            }
            Some(Msg::MenuCommand(MenuMsg::ResumeGame)) => {
                self.states_stack.pop();
                None
            }
            Some(Msg::Exit) => {
                self.engine_data.running = false;
                None
            }
            Some(Msg::ShowWinScreen) => {
                let drain_range = ..self.states_stack.len();
                self.states_stack.drain(drain_range);
                let win_screen = self.winning_screen();
                self.states_stack.push(win_screen);
                None
            }
            Some(Msg::ShowCredits) => {
                let drain_range = ..self.states_stack.len();
                self.states_stack.drain(drain_range);
                let credits_screen = self.credits();
                self.states_stack.push(credits_screen);
                None
            }
            Some(x) => {
                self.messages.push_back(x);
                None
            }
            _ => None,
        }
    }

    fn render(&mut self) {
        self.renderer.set_draw_color(RGB(0, 0, 0));
        self.renderer.clear();
        {
            self.renderer.set_draw_color(RGB(150, 150, 150));
            self.renderer
                .draw_rect(Rect::new(0,
                                     0,
                                     self.engine_data.window_size.0,
                                     self.engine_data.window_size.1))
                .unwrap();
        }
        if !self.states_stack.is_empty() {
            let mut last_drawable_index = self.states_stack.len();
            'fullscreen: for index in (0..self.states_stack.len()).rev() {
                if self.states_stack[index].is_fullscreen() {
                    last_drawable_index = index;
                    break 'fullscreen;
                }
            }
            for index in last_drawable_index..self.states_stack.len() {
                self.states_stack[index].render(&mut self.renderer, &self.engine_data);
            }
        }
        // if let Some(state) = self.states_stack.last_mut() {
        //     state.render(&mut self.renderer, &self.engine_data);
        // }
        self.renderer.present();
    }

    fn process(&mut self) -> bool {
        let ticks_at_start = self.timer.ticks();

        self.marked_events.drain();

        for event in self.event_pump.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::event::WindowEvent;

            if let KeyDown { keycode: x, .. } = event {
                if self.marked_events.contains(&(x.unwrap())) {
                    continue;
                } else {
                    self.marked_events.insert(x.unwrap());
                }
            }

            match event {
                Quit { .. } => self.messages.push_back(Msg::Exit),
                KeyDown { keycode: Some(x), .. } => {
                    self.messages.push_back(Msg::ButtonPressed(x));
                }
                KeyUp { keycode: Some(x), .. } => {
                    self.messages.push_back(Msg::ButtonReleased(x));
                }
                Window { win_event: WindowEvent::Resized(x, y), .. } => {
                    println!("Window resized, {} {}", x, y);
                }
                _ => {}
            }
        }

        self.render();
        while let Some(msg) = self.messages.pop_front() {
            self.update(msg).map(|m| self.messages.push_back(m));
        }

        let ticks_at_finish = self.timer.ticks();
        if ticks_at_finish - ticks_at_start < FPS_LOCK {
            self.timer.delay(FPS_LOCK - (ticks_at_finish - ticks_at_start));
        }
        self.messages.push_back(Msg::Tick(self.timer.ticks() - ticks_at_start));
        self.engine_data.running
    }
}
