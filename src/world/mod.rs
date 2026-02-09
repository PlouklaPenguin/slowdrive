mod generation;
mod perlin;

use crate::world::generation::{create_chunks, destroy_chunks};

use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, create_chunks)
            .add_systems(Update, destroy_chunks);
    }
}
