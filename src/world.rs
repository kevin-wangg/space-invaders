
struct Sprite {
    body: Vec<(u32, u32)>
}

pub struct World {
    player: Sprite,
}

struct WorldBuilder {
    player: Option<Sprite>,
}

impl WorldBuilder {
    fn new() -> Self {
        Self {
            player: None,
        }
    }

    fn with_player(mut self, player: Sprite) -> Self {
        self.player = Some(player);
        self
    }

    fn build(self) -> Result<World, String> {
        let player = self.player.ok_or("Player is required")?;
        Ok(
            World {
                player
            }
        )
    }
}
