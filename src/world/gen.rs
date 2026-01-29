use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

use crate::world::perlin;

const VIEW_RADIUS: i32 = 8;
const CHUNK_SIZE: i32 = 32;
const SCALE: f32 = 25.0;

// of size u*v;
#[derive(Component)]
struct WorldMap(Vec<f32>);

// #[derive(Component)]
// struct ChunkComponent { pos: [i32; 2], }

pub fn create_chunks(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    for z in -VIEW_RADIUS..VIEW_RADIUS {
        for x in -VIEW_RADIUS..VIEW_RADIUS {
            let (c_x, c_z) = ((x*CHUNK_SIZE) as f32, (z*CHUNK_SIZE) as f32);
            // println!("c_x:{}, c_y:{}, x: {}, z: {}", c_x, c_z, x, z);
            // println!("WHAT");

            commands.spawn((
                Mesh3d(meshes.add(chunk_builder(c_x, c_z))),
                MeshMaterial3d(materials.add(if x % 2 == 0 {Color::BLACK} else {Color::WHITE})),
                Transform::from_xyz(c_x, 0., c_z)
            ));
        }
    }
}


fn chunk_builder(s_x: f32, s_z: f32) -> Mesh {
    let z_vertex_count = 32;
    let x_vertex_count = 32;
    let num_vertices = (z_vertex_count * x_vertex_count) as usize;

    let mut positions: Vec<Vec3> = Vec::with_capacity(num_vertices);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
    let mut indices: Vec<u32> = Vec::with_capacity(num_vertices);

    let rotation = Quat::from_rotation_arc(Vec3::Y, Vec3::Y);
    let size = Vec2::new(CHUNK_SIZE as f32, CHUNK_SIZE as f32);

    for z in 0..z_vertex_count {
        for x in 0..x_vertex_count {
            let tx = x as f32 / (x_vertex_count - 1) as f32;
            let tz = z as f32 / (z_vertex_count - 1) as f32;
            // let ty = perlin::noise(tx, 0., tz);
            // println!("{}", ty);
            // let pos = Vec3::new((-0.5 + tx) * size.x, ty * SCALE, (-0.5 + tz) * size.y);
            let mut pos = rotation * Vec3::new((-0.5 + tx) * size.x, 0., (-0.5 + tz) * size.y);
            pos.y = perlin::noise(
                (pos.x - s_x) * 1./64.,
                0.,
                (pos.z - s_z) * 1./64.
                ) * SCALE;


            // the normal is the grad at the point!!!
            positions.push(pos);
            // let norm = calc_normal(tx*5., tz*5., 1.);
            // normals.push(norm.to_array());

            // commands.spawn((
            //     Mesh3d(meshes.add(Segment3d::from_direction_and_length(norm, 2.))),
            //     MeshMaterial3d(materials.add(Color::WHITE)),
            //     Transform::from_xyz(pos.x + norm.x, pos.y + norm.y, pos.z + norm.z),
            // ));

            uvs.push([tx, tz]);
        }
    }

    for z in 0..z_vertex_count - 1 {
        for x in 0..x_vertex_count - 1 {
            let quad = z * x_vertex_count + x;
            indices.push(quad + x_vertex_count + 1);
            indices.push(quad + 1);
            indices.push(quad + x_vertex_count);
            indices.push(quad);
            indices.push(quad + x_vertex_count);
            indices.push(quad + 1);
        }
    }

    let mut world_mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_indices(Indices::U32(indices))
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    // .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    world_mesh.duplicate_vertices();
    world_mesh.compute_flat_normals();

    world_mesh
}

// pub fn create_world() {}

// pub fn spawn_world(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let z_vertex_count = 128;
//     let x_vertex_count = 128;
//     let num_vertices = (z_vertex_count * x_vertex_count) as usize;
//     let num_indices = ((z_vertex_count - 1) * (x_vertex_count - 1) * 6) as usize;

//     let mut positions: Vec<Vec3> = Vec::with_capacity(num_vertices);
//     let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
//     let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

//     let rotation = Quat::from_rotation_arc(Vec3::Y, Vec3::Y);
//     let size = Vec2::new(256., 256.);

//     for z in 0..z_vertex_count {
//         for x in 0..x_vertex_count {
//             let tx = x as f32 / (x_vertex_count - 1) as f32;
//             let tz = z as f32 / (z_vertex_count - 1) as f32;
//             let ty = perlin::noise(tx, 0., tz);
//             // println!("{}", ty);
//             // let pos = Vec3::new((-0.5 + tx) * size.x, ty * SCALE, (-0.5 + tz) * size.y);
//             let pos = rotation * Vec3::new((-0.5 + tx) * size.x, ty * SCALE, (-0.5 + tz) * size.y);

//             // the normal is the grad at the point!!!
//             positions.push(pos);
//             // let norm = calc_normal(tx*5., tz*5., 1.);
//             // normals.push(norm.to_array());

//             // commands.spawn((
//             //     Mesh3d(meshes.add(Segment3d::from_direction_and_length(norm, 2.))),
//             //     MeshMaterial3d(materials.add(Color::WHITE)),
//             //     Transform::from_xyz(pos.x + norm.x, pos.y + norm.y, pos.z + norm.z),
//             // ));

//             uvs.push([tx, tz]);
//         }
//     }

//     for z in 0..z_vertex_count - 1 {
//         for x in 0..x_vertex_count - 1 {
//             let quad = z * x_vertex_count + x;
//             indices.push(quad + x_vertex_count + 1);
//             indices.push(quad + 1);
//             indices.push(quad + x_vertex_count);
//             indices.push(quad);
//             indices.push(quad + x_vertex_count);
//             indices.push(quad + 1);
//         }
//     }

//     let mut world_mesh = Mesh::new(
//         PrimitiveTopology::TriangleList,
//         RenderAssetUsages::default(),
//     )
//     .with_inserted_indices(Indices::U32(indices))
//     .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
//     // .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
//     .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

//     world_mesh.duplicate_vertices();
//     world_mesh.compute_flat_normals();

//     commands.spawn((
//         Mesh3d(meshes.add(world_mesh)),
//         MeshMaterial3d(materials.add(Color::BLACK)),
//     ));
// }

// fn produce_perlin_mesh() -> Mesh {
//     let width = 1;
//     let height = 1;
//     let sub_with = 1;
//     let sub_height = 1;
// }

fn r(s: f32, t: f32) -> Vec3 {
    return Vec3::new(s, perlin::noise(s, 0., t), t);
}

// fn calc_normal(x: f32, z: f32, h: f32) -> Dir3 {
//     // R is (s, noise(s, t), t)
//     let drdx = Vec3::new(
//         1.,
//         (perlin::noise(x + h, 0., z) - perlin::noise(x, 0., z)) / h,
//         0.,
//     );
//     let drdz = Vec3::new(
//         0.,
//         (perlin::noise(x, 0., z + h) - perlin::noise(x, 0., z)) / h,
//         1.,
//     );
//     //    let z =  (perlin::noise(Vec3::new(s - 1., 0., t)) * SCALE - perlin::noise(Vec3::new(s + 1., 0., t)) * SCALE) / 2.;
//     //    println!("{}", z);
//     if let Some(norm) = drdz.cross(drdx).try_normalize() {
//         // println!("{}", norm);
//         return Dir3::new_unchecked(norm);
//     } else {
//         panic!("{}", drdx.cross(drdz));
//     }
// }
