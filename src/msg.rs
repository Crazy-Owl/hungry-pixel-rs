/// Message type
#[derive(Debug)]
pub enum Msg {
    NoOp,
    Exit,
    Tick(u32),
    StartGame,
    ButtonPressed(ControlCommand),
    ButtonReleased(ControlCommand),
}

#[derive(Debug)]
pub enum ControlCommand {
    Up,
    Down,
    Left,
    Right,
    Escape,
}
