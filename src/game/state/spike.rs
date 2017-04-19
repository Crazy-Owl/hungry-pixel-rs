use rand;
use rand::Rng;
use sdl2::rect::Rect;

#[derive(Debug)]
pub struct Spike {
    pub x: f32,
    pub y: f32,
    speed: f32,
    direction: (i32, i32),
    pub rect: Rect,
    dimensions: (u32, u32),
}

impl Spike {
    pub fn new(x: i32, y: i32, w: u32, h: u32, direction: (i32, i32), speed: f32) -> Spike {
        Spike {
            x: x as f32,
            y: y as f32,
            speed: speed,
            direction: direction,
            rect: Rect::new(x, y, w, h),
            dimensions: (w, h),
        }
    }

    pub fn set_speed(&mut self, speed: f32) {
        self.speed = speed;
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

        let direction = match alignment {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => unimplemented!(),
        };

        let speed: f32 = rng.gen_range(0.0, 20.0);

        Spike::new(x, y, size_x, size_y, direction, speed)
    }

    pub fn update(&mut self, dt: f32, bounds: (f32, f32)) {
        self.x += self.speed * (self.direction.0 as f32) * dt;
        if self.x + (self.dimensions.0 as f32) >= bounds.0 {
            self.x = bounds.0 - self.dimensions.0 as f32;
            self.direction.0 = -self.direction.0;
        }
        if self.x < 0.0 {
            self.x = 0.0;
            self.direction.0 = -self.direction.0;
        }
        self.y += self.speed * (self.direction.1 as f32) * dt;
        if self.y + (self.dimensions.1 as f32) >= bounds.1 {
            self.y = bounds.1 - self.dimensions.1 as f32;
            self.direction.1 = -self.direction.1;
        }
        if self.y < 0.0 {
            self.y = 0.0;
            self.direction.1 = -self.direction.1;
        }
        self.update_rect();
    }

    pub fn update_rect(&mut self) {
        self.rect.reposition((self.x as i32, self.y as i32));
    }
}
