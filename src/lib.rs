mod camera;
mod player;
mod world;

use crate::{camera::CameraPlugin, player::PlayerPlugin, world::WorldPlugin};

use avian3d::prelude::*;
use bevy::app::Plugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub const CHUNK_VIEW_DISTANCE: i32 = 32;
pub const CHUNK_SIZE: i32 = 32;
pub const CHUNK_RES: u32 = 8;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(WorldPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(PlayerPlugin);
    }
}

pub struct DepsPlugin;

impl Plugin for DepsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(PhysicsPlugins::default())
            // .add_plugins(PhysicsDebugPlugin)
            .add_plugins(EguiPlugin::default())
            .add_plugins(WorldInspectorPlugin::new());
    }
}
