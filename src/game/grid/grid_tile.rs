use crate::*;
use serde::*;

//================================-================================-================================
#[derive(Default, Copy, Clone)]
pub struct GridTile {
    pub tile_type: TileType,
    pub matter: TileMatter,
    pub state: u32,
}

impl GridTile {
    pub fn open() -> Self {
        Self {
            tile_type: TileType::Open,
            matter: TileMatter::Stone,
            state: 0,
        }
    }

    pub fn new(
        tile_type: TileType,
        matter: TileMatter,
    ) -> Self {
        Self {
            tile_type,
            matter,
            state: 0,
        }
    }

    pub fn get_scene(
        &self,
    ) -> SceneAsset {
        TILE_TYPE_DEFS[0].scene
    }

    pub fn get_step_sound(
        &self,
    ) -> SoundAsset {
        TILE_MATTER_DEFS[self.matter as usize].step_sounds[thread_rng().gen_range(0..TILE_MATTER_DEFS[self.matter as usize].step_sounds.len())]
    }

    pub fn is_solid(
        &self,
    ) -> bool {
        match self.tile_type {
            TileType::Block => { true } 
            _ => { false }
        }
    }
}