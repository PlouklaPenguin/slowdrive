use bevy::asset::RenderAssetUsages;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::{Vec3, ops::floor, *};
// use bevy::reflect::Enum;
use avian3d::prelude::*;
use std::collections::HashMap;

use crate::player::player::Player;
use crate::world::perlin;

use crate::{CHUNK_RES, CHUNK_SIZE, CHUNK_VIEW_DISTANCE};

const SCALE: f32 = 25.0;




#[derive(Component)]
pub struct ChunkComponent {
    loc: IVec2,
}

impl ChunkComponent {
    fn new(loc: IVec2) -> Self {
        ChunkComponent { loc }
    }
}

// When we move into a new chunk, send an event on_player_chunk_change.
// Then we check, p

// Generate a global heightmap from a seed (MUCH LATER)
// To generate a chunk, acess this heightmap and attach an operation that sections
// off the hightmap into the chunk's heightmap
// If a chunk is within x from player, attach a collider. OR we could just enable sleeping
// If thats the thing that takes up the performance.


pub fn create_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    hm: Res<HeightMap>,
    chunk_query: Query<(Entity, &ChunkComponent, Option<&Collider>)>,
    player: Single<&Player>,
) {
    let chunks: HashMap<IVec2, (Entity, Option<&Collider>)> = chunk_query
        .iter()
        .map(|x| -> (IVec2, (Entity, Option<&Collider>)) { (x.1.loc, (x.0, x.2)) })
        .collect();
    let p_t = player.chunk_pos;

    for z in (-CHUNK_VIEW_DISTANCE)..CHUNK_VIEW_DISTANCE {
        for x in (-CHUNK_VIEW_DISTANCE)..CHUNK_VIEW_DISTANCE {
            let (c_x, c_z) = (((x + p_t[0]) * (CHUNK_SIZE)), ((z + p_t[1]) * (CHUNK_SIZE)));
            let index = ivec2(x + p_t[0], z + p_t[1]);

            // if x >= (-CHUNK_VIEW_DISTANCE / 2) && x < CHUNK_VIEW_DISTANCE / 2 {
            //     if z >= (-CHUNK_VIEW_DISTANCE / 2) && z < CHUNK_VIEW_DISTANCE / 2 {
            //         todo!();
            //     }
            // }

            if !chunks.contains_key(&index) {
                let (indices, p, uv) = chunk_builder(c_x as f32, c_z as f32, &hm);

                let world_mesh = Mesh::new(
                    PrimitiveTopology::TriangleList,
                    RenderAssetUsages::default(),
                )
                .with_inserted_indices(Indices::U32(indices.clone()))
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, p.clone())
                .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
                .with_computed_normals();

                commands.spawn((
                    Mesh3d(meshes.add(world_mesh)),
                    MeshMaterial3d(materials.add(
                        if ((x + p_t[0]) % 2 == 0) ^ ((z + p_t[1]) % 2 == 0) {
                            Color::BLACK
                        } else {
                            Color::WHITE
                        },
                    )),
                    Transform::from_xyz(c_x as f32, 0., c_z as f32),
                    ChunkComponent::new(ivec2(x + p_t[0], z + p_t[1])),
                ));
            };
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
        let t = comp.loc;
        if i32::abs(t.x - p_t.x) > CHUNK_VIEW_DISTANCE
            || i32::abs(t.y - p_t.y) > CHUNK_VIEW_DISTANCE
        {
            commands.entity(chunk).despawn();
        };
    }
}

fn chunk_builder(s_x: f32, s_z: f32, hm: &Res<HeightMap>) -> (Vec<u32>, Vec<Vec3>, Vec<[f32; 2]>) {
    let z_vertex_count = CHUNK_RES;
    let x_vertex_count = CHUNK_RES;
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
            // let n = perlin::noised(Vec3::new(
            //     (pos.x + s_x) * 1. / 128.,
            //     0.,
            //     (pos.z + s_z) * 1. / 128.,
            // )) * SCALE;

            let n = hm.0[(pos.x + s_x) as usize][(pos.z + s_z) as usize] * SCALE;

            pos.y = n;
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
