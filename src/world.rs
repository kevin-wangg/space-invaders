use crate::sprite::Sprite;

pub struct World {
    player: Sprite,
}

impl World {
    // Takes a mutable pixel frame buffer and sets it to black or white
    // depending on whether that pixel is occupied by an object in the world
    pub fn set_pixel(&self, x: u32, y: u32, pixel: &mut [u8]) {
        if self.player.in_bounds(x, y) {
            // Set pixel white
            pixel[0] = 255;
            pixel[1] = 255;
            pixel[2] = 255;
            pixel[3] = 255;
        } else {
            // Set pixel black
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
            pixel[3] = 255;
        }
    }
}

pub struct WorldBuilder {
    player: Option<Sprite>,
}

impl WorldBuilder {
    pub fn new() -> Self {
        Self {
            player: None,
        }
    }

    pub fn with_player(mut self, player: Sprite) -> Self {
        self.player = Some(player);
        self
    }

    pub fn build(self) -> Result<World, String> {
        let player = self.player.ok_or("Player is required")?;
        Ok(
            World {
                player
            }
        )
    }
}
