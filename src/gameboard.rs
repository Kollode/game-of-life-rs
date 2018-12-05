use graphics::types::Color;
use graphics::{Context, Graphics};

use piston::input::UpdateArgs;

/// Stores gameboard view settings.
pub struct GameboardSettings {
    /// Text color.
    pub text_color: Color,
}

impl GameboardSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardSettings {
        GameboardSettings {
            text_color: [0.0, 0.0, 0.1, 1.0],
        }
    }
}

/// Stores visual information about a gameboard.
pub struct Gameboard {
    /// Stores gameboard view settings.
    pub settings: GameboardSettings,
    updates: i64,
}

impl Gameboard {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardSettings) -> Gameboard {
        Gameboard {
            settings,
            updates: 0,
        }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, c: &Context, g: &mut G) {}

    pub fn update(&mut self, args: &UpdateArgs) {
        self.updates += 1
    }
}
