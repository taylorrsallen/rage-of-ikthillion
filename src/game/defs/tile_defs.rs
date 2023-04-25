use crate::*;

//================================-================================-================================
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub enum TileType {
    #[default]
    Open,
    Block,
    Door,
    PlayerSpawner,
}

#[derive(Default, Copy, Clone, Eq, PartialEq)]
pub enum TileMatter {
    #[default]
    Stone,
    DarkStone,
    DarkerStone,
    DarkestStone,
    VoidStone,
    Void,
    Wood,
    Gooey,
    Dirt,
    Grass,
    StonePath,
}

//================================-================================-================================
pub struct TileTypeDef {
    pub name: &'static str,
    pub scene: SceneAsset,
}

pub struct TileMatterDef {
    pub block_texture_ids: [u32; 6],
    pub step_sounds: &'static [SoundAsset],
}

//================================-================================-================================
pub const TILE_TYPE_DEFS: &'static [TileTypeDef] = &[
];

pub const TILE_MATTER_DEFS: &'static [TileMatterDef] = &[
    TileMatterDef { // Stone
        block_texture_ids: [6, 6, 7, 8, 6, 6],
        step_sounds: &[
            SoundAsset::StepBootStone00,
        ],
    },
    TileMatterDef { // DarkStone
        block_texture_ids: [9, 9, 10, 11, 9, 9],
        step_sounds: &[
            SoundAsset::StepBootStone00,
        ],
    },
    TileMatterDef { // DarkerStone
        block_texture_ids: [12, 12, 13, 14, 12, 12],
        step_sounds: &[
            SoundAsset::StepBootStone00,
        ],
    },
    TileMatterDef { // DarkestStone
        block_texture_ids: [15, 15, 16, 17, 15, 15],
        step_sounds: &[
            SoundAsset::StepBootStone00,
        ],
    },
    TileMatterDef { // VoidStone
        block_texture_ids: [18, 18, 19, 20, 18, 18],
        step_sounds: &[
            SoundAsset::StepBootStone00,
        ],
    },
    TileMatterDef { // Void
        block_texture_ids: [21, 21, 22, 23, 21, 21],
        step_sounds: &[
            SoundAsset::Cloth02,
        ],
    },
    TileMatterDef { // Wood
        block_texture_ids: [24, 24, 25, 26, 24, 24],
        step_sounds: &[
            SoundAsset::Cloth02,
        ],
    },
    TileMatterDef { // Gooey
        block_texture_ids: [27, 27, 28, 29, 27, 27],
        step_sounds: &[
            SoundAsset::GooeyStep00,
            SoundAsset::GooeyStep01,
        ],
    },
    TileMatterDef { // Dirt
        block_texture_ids: [3, 3, 4, 5, 3, 3],
        step_sounds: &[
            SoundAsset::StepBootDirt00,
            SoundAsset::StepBootDirt01,
            SoundAsset::StepBootDirt02,
        ],
    },
    TileMatterDef { // Grass
        block_texture_ids: [0, 0, 1, 2, 0, 0],
        step_sounds: &[
            SoundAsset::StepBootDirt00,
            SoundAsset::StepBootDirt01,
            SoundAsset::StepBootDirt02,
        ],
    },
    TileMatterDef { // StonePath
        block_texture_ids: [32, 32, 33, 34, 32, 32],
        step_sounds: &[
            SoundAsset::StepBootStone00,
        ],
    },
];