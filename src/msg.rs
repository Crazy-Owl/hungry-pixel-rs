/// Message type
#[derive(Debug, Clone, Copy)]
pub enum Msg {
    NoOp,
    Exit,
    Tick(u32),
    StartGame,
    ButtonPressed(ControlCommand),
    ButtonReleased(ControlCommand),
}

#[derive(Debug, Clone, Copy)]
pub enum ControlCommand {
    Up,
    Down,
    Left,
    Right,
    Escape,
    Enter,
}
