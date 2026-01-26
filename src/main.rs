use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

mod world;

use crate::world::gen::WorldPlugin;

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::basic::SILVER,
    input::mouse::AccumulatedMouseMotion,
    math::vec3,
    prelude::*,
    mesh::PrimitiveTopology,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .add_systems(Startup, (setup))
        .add_systems(Update, (move_camera, mouse_lock))
        .run();
}


#[derive(Component)]
struct World {
    seed: f32,
}

#[derive(Component)]
struct PlayerCamera {
    speed: f32,
    sprint: bool,
}

#[derive(Component, Debug, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 12.0, 16.0).looking_at(Vec3::ZERO, Vec3::Y),
        PlayerCamera { speed: 10.0, sprint: false },
        CameraSensitivity::default(),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
        MeshMaterial3d(materials.add(Color::Srgba(SILVER))),
        Transform::from_xyz(0.0, 0., 0.0),
    ));

    // commands.spawn((
    //     Mesh3d(meshes.add(Circle::new(4.0))),
    //     MeshMaterial3d(materials.add(Color::BLACK)),
    //     Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    // ));

    // commands.spawn((
    //     Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
    //     MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))), 
    // ));
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 2., 0.),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        }
    ));


    // commands.spawn((
    //     PointLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     Transform::from_xyz(4.0, 20.0, 4.0),
    // ));
}

fn move_camera(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut Transform, &mut PlayerCamera, &CameraSensitivity)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (mut c_transform, mut camera, camera_sensitivity) =
        query.single_mut().expect("Could not find a single camera");
    let immut_trans = c_transform.clone();

    let p_window = window_query.single();

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        camera.sprint = true;
    } else { camera.sprint = false; }


    if camera.sprint {
        camera.speed = 15.;
    } else {camera.speed = 10.}
    

    if keyboard_input.pressed(KeyCode::Space) {
        c_transform.translation += Vec3::Y * camera.speed * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::ControlLeft) {
        c_transform.translation -= Vec3::Y * camera.speed * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyW) {
        c_transform.translation -= vec3(immut_trans.local_z().x, 0., immut_trans.local_z().z)
            * camera.speed
            * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        c_transform.translation += vec3(immut_trans.local_z().x, 0., immut_trans.local_z().z)
            * camera.speed
            * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        c_transform.translation -= vec3(immut_trans.local_x().x, 0., immut_trans.local_x().z)
            * camera.speed
            * timer.delta_secs();
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        c_transform.translation += vec3(immut_trans.local_x().x, 0., immut_trans.local_x().z)
            * camera.speed
            * timer.delta_secs();
    }

    let delta = mouse_motion.delta;

    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = c_transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        c_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

fn mouse_lock(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_windows: Query<&mut CursorOptions, With<PrimaryWindow>>,
) {
    let mut cursor_options = q_windows
        .single_mut()
        .expect("could not find single window!!! WHAT");

    if keyboard_input.just_pressed(KeyCode::Escape) {
        if cursor_options.grab_mode == CursorGrabMode::Locked {
            cursor_options.grab_mode = CursorGrabMode::None;
            cursor_options.visible = true;
        } else {
            cursor_options.grab_mode = CursorGrabMode::Locked;
            cursor_options.visible = false;
        }
    }
}

// fn perlin(x: u32, y: u32) -> u32 {
//     let hash = calculate_hash(&(x, y));
// }

// fn calculate_hash<T: Hash>(t: &T) -> u64 {
//     let mut s = DefaultHasher::new();
//     t.hash(&mut s);
//     s.finish()
// }
