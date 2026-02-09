use crate::CHUNK_SIZE;
use crate::camera::PlayerCamera;
use crate::player::{Player, input::PlayerActions};

use avian3d::prelude::*;
use bevy::camera_controller::free_camera::FreeCameraState;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn move_player(
    mut commands: Commands,
    pq: Single<(Forces, &Transform, &ActionState<PlayerActions>, &mut Player)>,
    cs: Single<&FreeCameraState>,
) {
    let (mut forces, transform, action_state, mut player) = pq.into_inner();

    if !cs.enabled {
        for input_direction in PlayerActions::DIRECTIONS {
            if action_state.pressed(&input_direction) {
                if let Some(direction) = input_direction.direction() {
                    forces.apply_force(direction * 2000.);
                }
            }
        }
    }

    player.chunk_pos = ivec2(
        (transform.translation.x / CHUNK_SIZE as f32) as i32,
        (transform.translation.z / CHUNK_SIZE as f32) as i32,
    );
}
