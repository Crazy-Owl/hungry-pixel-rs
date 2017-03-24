use rand;
use rand::Rng;
use sdl2::rect::Rect;

#[derive(Debug)]
pub struct Spike {
    pub x: f32,
    pub y: f32,
    speed: (f32, f32),
    direction: (i8, i8),
    pub rect: Rect,
    dimensions: (u32, u32),
}

impl Spike {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Spike {
        Spike {
            x: x as f32,
            y: y as f32,
            speed: (0.0, 0.0),
            direction: (0, 0),
            rect: Rect::new(x, y, w, h),
            dimensions: (w, h),
        }
    }

    pub fn set_speed(&mut self, x_spd: f32, y_spd: f32) {
        self.speed = (x_spd, y_spd);
    }

    pub fn random(max_x: i32, max_y: i32, min_size: u32, max_size: u32) -> Spike {
        let mut rng = rand::thread_rng();
        let alignment: usize = rng.gen_range(0, 4);
        let (size_x, size_y): (u32, u32) = (rng.gen_range(min_size, max_size), rng.gen_range(min_size, max_size));
        let (x, y): (i32, i32) = match alignment {
            0 => (rng.gen_range(0, max_x - size_x as i32), 0),
            1 => (max_x - size_x as i32, rng.gen_range(0, max_y - size_y as i32)),
            2 => (rng.gen_range(0, max_x - size_x as i32), max_y - size_y as i32),
            3 => (0, rng.gen_range(0, max_y - size_y as i32)),
            _ => unimplemented!(),
        };

        Spike::new(x, y, size_x, size_y)
    }
}
