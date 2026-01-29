mod player;
mod world;

use crate::world::WorldPlugin;
use crate::player::camera::CameraPlugin;

use bevy::app::Plugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::app::App) {
      app.add_plugins(WorldPlugin)
      .add_plugins(CameraPlugin);
  }
}