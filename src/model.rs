/// Model
/// For now it just holds the message to display and running state of the game
#[derive(Debug)]
pub struct Model {
    pub running: bool,
    pub message: String,
}

impl Model {
    pub fn new() -> Model {
        Model {
            running: true,
            message: "Hello world".to_string(),
        }
    }
}
