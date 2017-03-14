use std::collections::HashMap;
use std::path::PathBuf;

use sdl2::ttf::{Sdl2TtfContext, Font};

pub struct FontCache<'ttf, 'b> {
    pub context: &'ttf Sdl2TtfContext,  // TODO: remove public quantifier from context field when transition to new planned scheme is complete
    pub cache: HashMap<String, Font<'ttf, 'b>>
}

impl<'ttf, 'b> FontCache<'ttf, 'b> {
    pub fn new(context: &'ttf mut Sdl2TtfContext) -> FontCache<'ttf, 'b> {
        FontCache {
            context: context,
            cache: HashMap::new(),
        }
    }

    pub fn load_font<T: Into<String>>(&mut self, key: T, path: PathBuf, size: u16) {
        let font: Font<'ttf, 'b> = self.context.load_font::<'ttf>(path, size).unwrap();
        self.cache.insert(key.into(), font);
    }
}
