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

#[derive(Debug, Clone, Copy)]
pub enum Movement {
    Up,
    Down,
    Left,
    Right,
}
