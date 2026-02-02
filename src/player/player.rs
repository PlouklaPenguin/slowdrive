use crate::player::camera::PlayerCamera;
use crate::CHUNK_SIZE;


use bevy::color::palettes::css::SILVER;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier3d::prelude::*;


pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    speed: f32,
    sprint: bool,
    pub chunk_pos: IVec2,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(Update, move_player_impulse);
    }
}

fn spawn_player(
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
            sprint: false,
            chunk_pos: ivec2(
              0, 0
            ),
    },
    RigidBody::Dynamic,
    Collider::cuboid(1., 1., 1.),
    GravityScale(0.5),
    Sleeping::disabled(),
    Ccd::enabled(),
    ExternalImpulse::default(),
  ));
}

fn move_player_impulse(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  timer: Res<Time>,
  mut player_q: Query<(&mut ExternalImpulse, &mut Player, &Transform), Without<PlayerCamera>>,
  mut camera_q: Query<&mut Transform, With<PlayerCamera>>
) {

    let mut c_t = camera_q.single_mut().expect("could not find a single camera");
    let (mut p_i, mut p_d, transform) =
        player_q.single_mut().expect("Could not find a single player");

    let t_old = transform.clone();


    if keyboard_input.pressed(KeyCode::Space) {
        p_i.impulse += Vec3::Y * p_d.speed * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::ControlLeft) {
        p_i.impulse -= Vec3::Y * p_d.speed * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        p_i.impulse -= vec3(transform.local_z().x, 0., transform.local_z().z)
            * p_d.speed
            * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        p_i.impulse += vec3(transform.local_z().x, 0., transform.local_z().z)
            * p_d.speed
            * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        p_i.impulse -= vec3(transform.local_x().x, 0., transform.local_x().z)
            * p_d.speed
            * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        p_i.impulse += vec3(transform.local_x().x, 0., transform.local_x().z)
            * p_d.speed
            * timer.delta_secs();
    }

    // if transform.translation != t_old.translation {
        p_d.chunk_pos = ivec2(
            (transform.translation.x / CHUNK_SIZE as f32) as i32,
            (transform.translation.z / CHUNK_SIZE as f32) as i32,
        );
    // }

    c_t.translation = transform.translation + vec3(transform.local_z().x, 1., transform.local_z().z) * 10.;
    
    c_t.look_at(transform.translation, Vec3::Y);   
}

fn _move_player_pos(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
    mut player_q: Query<(&mut Transform, &mut Player), Without<PlayerCamera>>,
    mut camera_q: Query<&mut Transform, With<PlayerCamera>>
) {
    let mut c_t = camera_q.single_mut().expect("could not find a single camera");
    let (mut p_t, mut p_d) =
        player_q.single_mut().expect("Could not find a single player");
    let immut_trans = p_t.clone();


    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        p_d.sprint = true;
    } else {
        p_d.sprint = false;
    }

    if p_d.sprint {
        p_d.speed = 15.;
    } else {
        p_d.speed = 10.;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        p_t.translation += Vec3::Y * p_d.speed * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::ControlLeft) {
        p_t.translation -= Vec3::Y * p_d.speed * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        p_t.translation -= vec3(immut_trans.local_z().x, 0., immut_trans.local_z().z)
            * p_d.speed
            * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        p_t.translation += vec3(immut_trans.local_z().x, 0., immut_trans.local_z().z)
            * p_d.speed
            * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        p_t.translation -= vec3(immut_trans.local_x().x, 0., immut_trans.local_x().z)
            * p_d.speed
            * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        p_t.translation += vec3(immut_trans.local_x().x, 0., immut_trans.local_x().z)
            * p_d.speed
            * timer.delta_secs();
    }

    if immut_trans.translation != p_t.translation {
        p_d.chunk_pos = ivec2(
            (p_t.translation.x / CHUNK_SIZE as f32) as i32,
            (p_t.translation.z / CHUNK_SIZE as f32) as i32,
        );
    }

    c_t.translation = p_t.translation + vec3(immut_trans.local_z().x, 1., immut_trans.local_z().z) * 10.;
    
    c_t.look_at(p_t.translation, Vec3::Y);   


}