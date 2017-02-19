/// Model
/// For now it just holds the message to display and running state of the game
#[derive(Debug)]
pub struct Model {
    pub running: bool,
    pub window_size: (u32, u32),
    pub message: String,
}

impl Model {
    pub fn new() -> Model {
        Model {
            running: true,
            window_size: (1024, 768),
            message: "Hello world".to_string(),
        }
    }
}
