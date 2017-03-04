/// Message type
#[derive(Debug, Clone, Copy)]
pub enum Msg {
    NoOp,
    Exit,
    Tick(u32),
    StartGame,
    ResumeGame,
    ToMainMenu,
    ShowGameMenu,
    ButtonPressed(ControlCommand),
    ButtonReleased(ControlCommand),
    PopState(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum ControlCommand {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Enter,
    Pause,
}
