use rand::Rng;
use rand;
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
        if self.nutrition < x {
            self.nutrition = 0.0;
        } else {
            self.nutrition -= x;
        }
        self.rect.resize(self.nutrition as u32, self.nutrition as u32);
    }

    pub fn random(x_max: u32, y_max: u32, nut_range_min: f32, nut_range_max: f32) -> Edible {

        let mut rng = rand::thread_rng();

        let nutrition = rng.gen_range(nut_range_min, nut_range_max);

        Edible {
            rect: Rect::new(rng.gen_range(0, x_max as i32 - nutrition as i32),
                            rng.gen_range(0, y_max as i32 - nutrition as i32),
                            nutrition as u32,
                            nutrition as u32),
            nutrition: nutrition,
        }
    }
}
