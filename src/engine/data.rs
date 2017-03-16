use engine::font::FontCache;

/// Model
/// For now it just holds the message to display and running state of the game
//#[derive(Debug)]
pub struct EngineData {
    pub running: bool,
    pub window_size: (u32, u32),
    pub font_cache: FontCache,
}

impl EngineData {
    pub fn new(font_cache: FontCache) -> EngineData {
        EngineData {
            running: true,
            window_size: (1024, 768),
            font_cache: font_cache,
        }
    }
}
