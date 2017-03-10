use sdl2::rect::Rect;

use engine::data::EngineData;
use super::pixel::GameSettings;

#[derive(Debug)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub speed: (f32, f32),
    pub direction: (i8, i8),
    pub rect: Rect,
    pub size: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            x: 0.0,
            y: 0.0,
            speed: (0.0, 0.0),
            direction: (0, 0),
            rect: Rect::new(0, 0, 20, 20),
            size: 20.0,
        }
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        self.rect.set_x(x as i32);
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
        self.rect.set_y(y as i32);
    }

    pub fn offset(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
        self.rect.reposition((self.x as i32, self.y as i32));
    }

    pub fn resize(&mut self, d_size: f32) {
        self.size += d_size;
        self.rect.resize(self.size as u32, self.size as u32);
    }

    // Boolean result of this fn tells whether the player has not yet lost the game
    // (`true` means "continue")
    pub fn process(&mut self,
                   x: f32,
                   engine_data: &mut EngineData,
                   settings: &GameSettings)
                   -> bool {
        let offset_args = (self.speed.0 * x / 1000.0, self.speed.1 * x / 1000.0);
        self.offset(offset_args.0, offset_args.1);
        self.resize(-settings.deterioration_rate * x / 1000.0);
        if self.size <= 1.0 {
            return false;
        }
        self.speed.0 += (self.direction.0 as f32) * settings.acceleration_rate * x / 1000.0;
        self.speed.1 += (self.direction.1 as f32) * settings.acceleration_rate * x / 1000.0;

        if self.x < 0.0 {
            self.set_x(0.0);
            self.speed.0 = -self.speed.0;
        }

        if self.x > (engine_data.window_size.0 as f32) - self.size {
            let new_x = engine_data.window_size.0 as f32 - self.size;
            self.set_x(new_x);
            self.speed.0 = -self.speed.0;
        }

        if self.y < 0.0 {
            self.set_y(0.0);
            self.speed.1 = -self.speed.1;
        }

        if self.y > (engine_data.window_size.1 as f32) - self.size {
            let new_y = engine_data.window_size.1 as f32 - self.size;
            self.set_y(new_y);
            self.speed.1 = -self.speed.1;
        }
        true
    }
}
