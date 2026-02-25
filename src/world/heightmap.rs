use crate::world::perlin::noised;


use bevy::prelude::*;

#[derive(Resource)]
pub struct WorldHeightMap(Vec<Vec<f32>>);

pub struct ChunkIndex {
  start: (u32, u32),
  size: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HeightMap{
  img: Vec<Vec<f32>>,
  res: u32,
}

impl Default for HeightMap {
    fn default() -> Self {   
        let res: u32 = 1024;
        let img = vec![vec![0.; res as usize]; res as usize];
        HeightMap {
          img,
          res,
        }
    }
}

impl Meshable for HeightMap {
  type Output = HeightMapMeshBuilder;
      
  fn mesh(&self) -> Self::Output {
    todo!();
  }
}

#[derive(Clone, Debug, Default, Reflect)]
#[reflect(Default, Debug, Clone)]
pub struct HeightMapMeshBuilder {
  pub height_map: HeightMap,
  pub half_size: u32,
  pub chunk: []
};

