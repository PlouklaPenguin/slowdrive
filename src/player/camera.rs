use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*, window::PrimaryWindow};

use std::f32::consts::FRAC_PI_2;

#[derive(Component, Debug, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

#[derive(Component)]
struct PlayerCamera {
    speed: f32,
    sprint: bool,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, move_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 12., 16.).looking_at(Vec3::ZERO, Vec3::Y),
        PlayerCamera {
            speed: 10.0,
            sprint: false,
        },
        CameraSensitivity::default(),
    ));
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
    } else {
        camera.sprint = false;
    }

    if camera.sprint {
        camera.speed = 15.;
    } else {
        camera.speed = 10.
    }

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
