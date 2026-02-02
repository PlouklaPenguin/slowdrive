use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::{ops::abs, Vec3, *};
use bevy::reflect::Enum;
use bevy_rapier3d::prelude::shape_views::TriangleView;
use bevy_rapier3d::prelude::*;
use std::collections::HashSet;

use crate::player::player::Player;
use crate::world::perlin;

use crate::CHUNK_SIZE;
use crate::CHUNK_VIEW_DISTANCE;

const SCALE: f32 = 25.0;

#[derive(Component)]
pub struct ChunkComponent(IVec2);

// When we move into a new chunk, send an event on_player_chunk_change.
// Then we check, p

pub fn create_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    chunk_query: Query<&ChunkComponent>,
    player_query: Query<&Player>,
) {
    let chunks: HashSet<IVec2> = chunk_query.iter().map(|x| -> IVec2 { x.0 }).collect();
    let p_t = player_query
        .single()
        .expect("could not find single player")
        .chunk_pos;

    for z in -CHUNK_VIEW_DISTANCE..CHUNK_VIEW_DISTANCE {
        for x in -CHUNK_VIEW_DISTANCE..CHUNK_VIEW_DISTANCE {
            let (c_x, c_z) = (((x + p_t[0]) * (CHUNK_SIZE)), ((z + p_t[1]) * (CHUNK_SIZE)));
            let index = ivec2(x + p_t[0], z + p_t[1]);

            let (indices, vertices, uvs) = chunk_builder(c_x as f32, c_z as f32);

            let world_mesh = Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::default(),
            )
            .with_inserted_indices(Indices::U32(indices.clone()))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

            if !chunks.contains(&index) {
                commands
                    .spawn((
                        Mesh3d(meshes.add(world_mesh)),
                        MeshMaterial3d(materials.add(
                            if ((x + p_t[0]) % 2 == 0) ^ ((z + p_t[1]) % 2 == 0) {
                                Color::BLACK
                            } else {
                                Color::WHITE
                            },
                        )),
                        Transform::from_xyz(c_x as f32, 0., c_z as f32),
                        ChunkComponent(ivec2(x + p_t[0], z + p_t[1])),
                    ))
                    .insert(
                        Collider::trimesh(
                            vertices,
                            indices
                                .chunks(3)
                                .map(|v| v.try_into().expect("length too long!!!"))
                                .collect(),
                        )
                        .expect("failed to build trimesh"),
                    );
            }
        }
    }
}

pub fn destroy_chunks(
    mut commands: Commands,
    chunk_query: Query<(&ChunkComponent, Entity)>,
    // CHANGE LATER
    player_query: Query<&Player>,
) {
    let chunks = chunk_query.iter();
    let p_t = player_query
        .single()
        .expect("Couldn't identitfy a single player")
        .chunk_pos;
    for (comp, chunk) in chunks {
        let t = comp.0;
        if i32::abs(t.x - p_t.x) > CHUNK_VIEW_DISTANCE
            || i32::abs(t.y - p_t.y) > CHUNK_VIEW_DISTANCE
        {
            commands.entity(chunk).despawn();
        };
    }
}

fn chunk_builder(s_x: f32, s_z: f32) -> (Vec<u32>, Vec<Vec3>, Vec<[f32; 2]>) {
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
                (pos.x + s_x) * 1. / 128.,
                0.,
                (pos.z + s_z) * 1. / 128.,
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
    return (indices, positions, uvs);

    // let mut world_mesh = Mesh::new(
    //     PrimitiveTopology::TriangleList,
    //     RenderAssetUsages::default(),
    // )
    // .with_inserted_indices(Indices::U32(indices))
    // .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    // .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    // world_mesh.duplicate_vertices();
    // world_mesh.compute_flat_normals();

    // (world_mesh, positions, indices)
}
