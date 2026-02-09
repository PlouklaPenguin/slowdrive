use crate::player::input::PlayerActions;

use bevy::color::palettes::css::SILVER;
use bevy::prelude::*;
use avian3d::prelude::*;

#[derive(Component)]
pub struct Player {
    speed: f32,
    // sprint: bool,
    pub chunk_pos: IVec2,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(2.))),
        MeshMaterial3d(materials.add(Color::Srgba(SILVER))),
        Transform::from_xyz(0., 20., 0.),
        Player {
            speed: 100.0,
            // sprint: false,
            chunk_pos: ivec2(0, 0),
        },
        PlayerActions::default_input_map(),
        RigidBody::Dynamic,
        Collider::cuboid(2., 2., 2.),
        GravityScale(2.0),
        Friction::new(0.9)
    ));
}
