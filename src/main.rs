use std::f32::consts::PI;

use slowdrive::{DepsPlugin, GamePlugin};

use bevy::asset::AssetMetaCheck;
use bevy::{
    color::palettes::basic::SILVER,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

fn main() -> AppExit {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Game".to_string(),
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
        )
        .add_plugins(DepsPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, setup_world)
        .add_systems(Update, mouse_lock)
        .run()
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
    //     MeshMaterial3d(materials.add(Color::Srgba(SILVER))),
    //     Transform::from_xyz(0.0, 0., 0.0),
    // ));
    commands.spawn((
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 5., 0.),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));
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
