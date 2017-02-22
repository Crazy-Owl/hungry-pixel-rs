use sdl2::rect::Rect;

pub struct Edible {
    pub rect: Rect,
    pub nutrition: f32,
}

impl Edible {
    pub fn new(x: i32, y: i32, nutrition: f32) -> Edible {
        Edible {
            rect: Rect::new(x, y, nutrition as u32, nutrition as u32),
            nutrition: nutrition,
        }
    }

    pub fn deteriorate(&mut self, x: f32) {
        self.nutrition -= x;
        self.rect.resize(self.nutrition as u32, self.nutrition as u32);
    }
}
