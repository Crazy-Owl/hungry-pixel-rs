/// Message type
#[derive(Debug)]
pub enum Msg {
    NoOp,
    Exit,
    Tick(u32),
    StartGame,
    ButtonPressed(ControlCommand)
}

#[derive(Debug)]
pub enum ControlCommand {
    Up,
    Down,
    Left,
    Right,
    Escape
}