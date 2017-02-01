extern crate sdl2;

use std::collections::linked_list::LinkedList;

// Game Engine

struct Engine {
    model: Model,
    messages: LinkedList<Msg>
}

impl Engine {
    fn update(&mut self, msg : Msg) -> Option<Msg> {
        match msg {
            Msg::NoOp => None,
            Msg::Exit => {
                self.model.state = false;
                None
            },
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
struct Model {
    state: bool,
    message: String
}

// Msg

#[derive(Debug)]
enum Msg { NoOp, Exit, Change(String) }

fn main() {
    let mut model = Model {state: true, message: "Hello world".to_string()};
    let mut engine : Engine = Engine {model: model, messages: LinkedList::new()};
    engine.messages.push_back(Msg::Change("Kek".to_string()));
    'running: loop {
        if !engine.process() {
            break 'running;
        }
    }
}
