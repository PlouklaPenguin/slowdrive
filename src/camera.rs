use bevy::camera_controller::free_camera::{FreeCamera, FreeCameraPlugin, FreeCameraState};
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
        app.add_plugins(FreeCameraPlugin)
            .add_systems(Startup, setup_camera)
            .add_systems(Update, update_camera_state);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1.0, 0.0).looking_to(Vec3::ZERO, Vec3::Y),
        FreeCamera {
            sensitivity: 0.2,
            friction: 25.0,
            walk_speed: 3.0,
            run_speed: 9.0,
            ..default()
        },
        PlayerCamera,
    ));
}

fn update_camera_state(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    fcq: Single<&mut FreeCameraState>,
) {
    let mut fcs = fcq.into_inner();
    if input.just_pressed(KeyCode::KeyB) {
        fcs.enabled = !fcs.enabled;
    }
}
