mod generation;
mod perlin;
mod heightmap;

use crate::world::generation::{create_chunks, destroy_chunks};
use crate::world::heightmap::HeightMap;

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<HeightMap>()
            .add_systems(Update, create_chunks)
            .add_systems(Update, destroy_chunks);
    }
}
