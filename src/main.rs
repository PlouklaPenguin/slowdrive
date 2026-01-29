// use std::f32::consts::FRAC_PI_2;
use std::f32::consts::PI;

use slowdrive::GamePlugin;

use bevy::asset::AssetMetaCheck;
use bevy::{
    // asset::RenderAssetUsages,
    color::palettes::basic::SILVER,
    // input::mouse::AccumulatedMouseMotion,
    // math::vec3,
    prelude::*,
    // mesh::PrimitiveTopology,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

// use std::hash::{DefaultHasher, Hash, Hasher};

fn main() {
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
        .add_plugins(GamePlugin)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_lock)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // asset_server: Res<AssetServer>
) {
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

// fn perlin(x: u32, y: u32) -> u32 {
//     let hash = calculate_hash(&(x, y));
// }

// fn calculate_hash<T: Hash>(t: &T) -> u64 {
//     let mut s = DefaultHasher::new();
//     t.hash(&mut s);
//     s.finish()
// }
