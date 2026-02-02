use bevy::prelude::*;


#[derive(Component, Debug, Deref, DerefMut)]
struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.003, 0.002))
    }
}

#[derive(Component)]
pub struct PlayerCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(20., 50., 0.),
        PlayerCamera,
        CameraSensitivity::default(),
    ));
}


// fn rotate_camera {

//     let delta = mouse_motion.delta;

//     if delta != Vec2::ZERO {
//         let delta_yaw = -delta.x * camera_sensitivity.x;
//         let delta_pitch = -delta.y * camera_sensitivity.y;

//         let (yaw, pitch, roll) = c_transform.rotation.to_euler(EulerRot::YXZ);
//         let yaw = yaw + delta_yaw;

//         const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
//         let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

//         c_transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
// }
