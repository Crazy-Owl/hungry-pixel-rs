use std::fmt::{Display, Formatter, Result};

use sdl2::keyboard::Keycode;
/// Message type
#[derive(Debug, Clone, Copy)]
pub enum Msg {
    NoOp,
    Exit,
    Tick(u32),
    StartGame,
    MenuCommand(MenuMsg),
    ButtonPressed(Keycode),
    ButtonReleased(Keycode),
    Command(GameCommand),
    PopState(usize),
    ShowGameOver,
    ShowWinScreen,
    ShowCredits,
    ShowOptions,
    OptionsSelect(Movement),
    OptionsSet(Keycode),
}

#[derive(Debug, Clone, Copy)]
pub enum MenuMsg {
    ToMainMenu,
    ShowGameMenu,
    ResumeGame,
}

#[derive(Debug, Clone, Copy)]
pub enum GameCommand {
    StartMovement(Movement),
    StopMovement(Movement),
    Pause,
    Resume,
    Menu,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl Display for Movement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", {
            match *self {
                Movement::Up => "Up",
                Movement::Down => "Down",
                Movement::Left => "Left",
                Movement::Right => "Right",
            }
        })
    }
}
