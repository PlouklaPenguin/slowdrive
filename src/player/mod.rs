use input::InputPlugin;
use movement::*;
use player::*;

use bevy::prelude::*;

mod input;
mod movement;
pub mod player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin)
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}
