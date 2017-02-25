/// Model
/// For now it just holds the message to display and running state of the game
#[derive(Debug)]
pub struct EngineData {
    pub running: bool,
    pub window_size: (u32, u32),
    pub message: String,
}

impl EngineData {
    pub fn new() -> EngineData {
        EngineData {
            running: true,
            window_size: (1024, 768),
            message: "Hello world".to_string(),
        }
    }
}
