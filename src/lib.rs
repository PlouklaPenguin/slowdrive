mod player;
mod world;

use crate::player::camera::CameraPlugin;
use crate::player::player::PlayerPlugin;
use crate::world::WorldPlugin;

use bevy::app::Plugin;

pub const CHUNK_VIEW_DISTANCE: i32 = 2;
pub const CHUNK_SIZE: i32 = 32;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(WorldPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(PlayerPlugin);
    }
}
