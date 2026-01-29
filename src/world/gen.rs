use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;

use crate::world::perlin;

const VIEW_RADIUS: i32 = 8;
const CHUNK_SIZE: i32 = 32;
const SCALE: f32 = 25.0;


#[derive(Component)]
struct ChunkComponent {
    pos: [i32; 2],
}

pub fn create_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for z in -VIEW_RADIUS..VIEW_RADIUS {
        for x in -VIEW_RADIUS..VIEW_RADIUS {
            let (c_x, c_z) = ((x * CHUNK_SIZE) as f32, (z * CHUNK_SIZE) as f32);

            commands.spawn((
                Mesh3d(meshes.add(chunk_builder(c_x, c_z))),
                MeshMaterial3d(materials.add(if (x % 2 == 0) ^ (z % 2 == 0) {
                    Color::BLACK
                } else {
                    Color::WHITE
                })),
                Transform::from_xyz(c_x, 0., c_z),
                ChunkComponent { pos: [x, z] },
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
            let mut pos = rotation * Vec3::new((-0.5 + tx) * size.x, 0., (-0.5 + tz) * size.y);
            pos.y = perlin::noise(Vec3::new(
                (pos.x + s_x) * 1. / 64.,
                0.,
                (pos.z + s_z) * 1. / 64.,
            )) * SCALE;

            positions.push(pos);
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
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    world_mesh.duplicate_vertices();
    world_mesh.compute_flat_normals();

    world_mesh
}















