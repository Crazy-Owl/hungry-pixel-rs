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
    ControlCommand(Control),
    PopState(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum MenuMsg {
    ToMainMenu,
    ShowGameMenu,
    ResumeGame,
}

#[derive(Debug, Clone, Copy)]
pub enum Control {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Enter,
    Pause,
    Space,
}
