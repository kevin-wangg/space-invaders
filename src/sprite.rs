use crate::app::WIDTH;

pub struct Sprite {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Sprite {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn move_by(&mut self, dx: i32) {
        self.x = (self.x as i32 + dx)
            .max(0)
            .min(WIDTH as i32 - self.width as i32) as u32;
    }
}
