mod chunk;
mod gen;
mod perlin;

use crate::world::gen::create_chunks;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_chunks);
    }
}
