use super::{PolarGame, GameSetup};
use super::object::Point;

#[derive(Default)]
pub struct PolarGameBuilder {
    setup: GameSetup
}

impl PolarGameBuilder {
    pub fn with_tunnel_length<'a> (&'a mut self, length: f64) -> &'a mut Self {
        self.setup.radial_max = length;
        self
    }

    pub fn with_player_start<'a> (&'a mut self, start: Point) -> &'a mut Self {
        self.setup.player_start = start;
        self
    }

    pub fn with_player_width<'a> (&'a mut self, width: Point) -> &'a mut Self {
        self.setup.player_width = width;
        self
    }
    
    pub fn build_game(&mut self) -> PolarGame {
        PolarGame::new(self.setup)
    }
}
