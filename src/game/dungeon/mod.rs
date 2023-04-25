use crate::*;

//================================-================================-================================
pub struct DungeonPlugin;
impl Plugin for DungeonPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        
    }
}

//================================-================================-================================
#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub enum GeneratorTile {
    #[default]
    Background, // 0: background tile (stone)
    Room, // 1: open, part of a room
    Hall, // 2: open, not part of a room
    Door, // 3: connection point into room
    Pit,  // 4: dig down large distance
    PitBridge, // 5: block in walking layer, otherwise pit
    Cavern, // 6: no walking layer, dig up and down large distance
    CavernBridge, // 7: block in walking layer, otherwise cavern
    Pillar, // 8: solid up to walking layer, empty above
    DoubleRoom, // 9: 2 high ceiling
    TripleRoom, // 10: 3 high ceiling
    Wall, // 11: 2 block tall
    Roof, // 12: 1 block roof
    PlayerSpawner, // 13: 2 block tall
    WeaponSpawner, // 14: can spawn katana, sword, axe
    PickaxeSpawner, // 15: always spawns pickaxe
    HolyHandgunOfAntinukPedastal, // 16
    Dirt, // 17
    EvilNinjaSpawner, // 18
    EvilWizardSpawner, // 19
    EvilIkthillionSpawner, // 20
    EvilFishSpawner, // 21
    EvilWhelpSpawner, // 22
    EvilChickenSpawner, // 23
    EvilLizardSpawner, // 24
    TownRoom, // 25
    TownDoubleRoom, // 26
    TownTripleRoom, // 27
    EvilGuardKnightSpawner, // 28
    EvilAngelSpawner, // 29
    KatanaSpawner, // 30
}

pub struct GeneratorRoom {
    pub name: &'static str,
    pub dim: IVec2,
    pub tiles: &'static [&'static [u32]],
}

const GENERATOR_ROOMS: &'static [GeneratorRoom] = &[
    GeneratorRoom {
        name: "Ravine",
        dim: IVec2::new(6, 6),
        tiles: &[
            &[8 ,6 ,6 ,6 ,6 ,0 ],
            &[8 ,6 ,6 ,6 ,6 ,0 ],
            &[8 ,6 ,6 ,29,6 ,0 ],
            &[8 ,6 ,6 ,6 ,6 ,0 ],
            &[8 ,6 ,6 ,6 ,6 ,0 ],
            &[1 ,18,1 ,1 ,18,1 ],
        ],
    },
    GeneratorRoom {
        name: "Thin",
        dim: IVec2::new(5, 4),
        tiles: &[
            &[0 ,0 ,1 ,0 ,0 ],
            &[18,18,1 ,18,18],
            &[18,18,1 ,18,18],
            &[0, 0, 1 ,0 ,0 ],
        ],
    },
    GeneratorRoom {
        name: "Tiny",
        dim: IVec2::new(3, 3),
        tiles: &[
            &[19,19,19],
            &[19,14,0 ],
            &[19,19,19],
        ],
    },
    GeneratorRoom {
        name: "Pillar",
        dim: IVec2::new(5, 5),
        tiles: &[
            &[0 ,10,10,10,0 ],
            &[10,10,9 ,10,10],
            &[10,9 ,0 ,9 ,10],
            &[10,10,9 ,10,10],
            &[0 ,10,10,10,0 ],
        ],
    },
    GeneratorRoom {
        name: "Boring",
        dim: IVec2::new(5, 4),
        tiles: &[
            &[1 ,1 ,1 ,1 ,1 ],
            &[1 ,19,19,1 ,1 ],
            &[1 ,19,19,1 ,1 ],
            &[1 ,1 ,1 ,1 ,1 ],
        ],
    },
    GeneratorRoom {
        name: "Tunnel",
        dim: IVec2::new(5, 15),
        tiles: &[
            &[9 ,10,10,10,9 ],
            &[9 ,10,14,10,9 ],
            &[9 ,10,10,10,9 ],
            &[0 ,9 ,9 ,9 ,0 ],
            &[9 ,21,21,21,9 ],
            &[9 ,21,21,21,9 ],
            &[9 ,21,21,21,9 ],
            &[0 ,9 ,9 ,9 ,0 ],
            &[9 ,21,21,21,9 ],
            &[9 ,21,21,21,9 ],
            &[9 ,21,21,21,9 ],
            &[0 ,9 ,9 ,9 ,0 ],
            &[9 ,10,10,10,9 ],
            &[9 ,10,10,10,9 ],
            &[9 ,10,10,10,9 ],
        ],
    },
    GeneratorRoom {
        name: "4 Pillars",
        dim: IVec2::new(7, 7),
        tiles: &[
            &[9 ,9 ,9 ,9 ,9 ,9 ,9 ],
            &[9 ,0 ,10,10,10,0 ,9 ],
            &[9 ,10,10,10,10,10,9 ],
            &[9 ,10,10,14,10,10,9 ],
            &[9 ,10,10,10,10,10,9 ],
            &[9 ,0 ,10,10,10,0 ,9 ],
            &[9 ,9 ,9 ,9 ,9 ,9 ,9 ],
        ],
    },
    GeneratorRoom {
        name: "Hidden",
        dim: IVec2::new(5, 5),
        tiles: &[
            &[0 ,0 ,0 ,0 ,0 ],
            &[0 ,19,1 ,18,0 ],
            &[0 ,1 ,19,1 ,0 ],
            &[0 ,18,1 ,19,0 ],
            &[0 ,0 ,0 ,0 ,0 ],
        ],
    },
    GeneratorRoom {
        name: "Dragon Den",
        dim: IVec2::new(10, 10),
        tiles: &[
            &[0 ,0 ,1 ,1 ,1 ,0 ,0 ,0 ,0 ,0 ],
            &[19,1 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
            &[1 ,0 ,1 ,1 ,1 ,1 ,19,0 ,0 ,0 ],
            &[1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ],
            &[1 ,0 ,0 ,22,22,0 ,1 ,1 ,1 ,1 ],
            &[1 ,1 ,1 ,1 ,30,0 ,1 ,1 ,1 ,1 ],
            &[0 ,1 ,0 ,1 ,22,0 ,0 ,0 ,0 ,0 ],
            &[0 ,1 ,0 ,0 ,0 ,0 ,19,1 ,1 ,0 ],
            &[0 ,1 ,1 ,1 ,1 ,1 ,1 ,0 ,1 ,0 ],
            &[0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ],
        ],
    },
    GeneratorRoom {
        name: "Oval",
        dim: IVec2::new(5, 9),
        tiles: &[
            &[0, 0 ,1 ,0 ,0 ],
            &[0, 0 ,1 ,0 ,0 ],
            &[0, 1 ,9 ,1 ,0 ],
            &[0, 9 ,19,9 ,0 ],
            &[1, 9 ,9 ,9 ,1 ],
            &[0, 9 ,19,9 ,0 ],
            &[0, 1 ,9 ,1 ,0 ],
            &[0, 0 ,1 ,0 ,0 ],
            &[0, 0 ,1 ,0 ,0 ],
        ],
    },
    GeneratorRoom {
        name: "Cavern Bridge",
        dim: IVec2::new(13, 19),
        tiles: &[
            &[6 ,6 ,6 ,6 ,8 ,8 ,8 ,8 ,8 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,8 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[8 ,8 ,28,8 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[8 ,8 ,14,28,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[8 ,8 ,28,8 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,8 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,6 ,6 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ],
            &[6 ,6 ,6 ,6 ,8 ,8 ,8 ,8 ,8 ,6 ,6 ,6 ,6 ],
        ],
    },
];

const IKTHILLIONS_CHAMBERS: GeneratorRoom = GeneratorRoom {
    name: "Ikthillion's Chambers",
    dim: IVec2::new(45, 45),
    tiles: &[
        &[0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,0 ,18,1 ,1 ,1 ,1 ,0 ,1 ,1 ,1 ,1 ,0 ,1 ,1 ,18,0 ,1 ,1 ,1 ,0 ,0 ],
        &[0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,0 ,1 ,0 ,0 ,0 ,18,0 ,1 ,0 ,0 ,1 ,0 ,1 ,0 ,1 ,0 ,1 ,0 ,18,0 ,0 ],
        &[0 ,0 ,0 ,1 ,0 ,0 ,1 ,1 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,0 ,1 ,0 ,1 ,1 ,1 ,1 ,1 ,1 ,0 ,1 ,0 ,1 ,0 ,1 ,0 ,1 ,0 ,1 ,0 ,0 ],
        &[1 ,1 ,1 ,1 ,0 ,0 ,1 ,30,1 ,0 ,0 ,1 ,1 ,1 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,1 ,1 ,0 ,1 ,0 ,0 ,1 ,0 ,1 ,0 ,1 ,18,1 ,0 ,1 ,1 ,1 ,0 ,1 ,1 ,1 ],
        &[0 ,1 ,0 ,1 ,0 ,0 ,1 ,1 ,1 ,0 ,0 ,1 ,0 ,1 ,0 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,0 ,0 ,0 ,1 ,0 ,0 ,1 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,18,0 ,0 ,0 ,0 ],
        &[0 ,1 ,0 ,1 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,1 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,18,1 ,1 ,0 ,1 ,18,0 ,18,0 ,1 ,1 ,1 ,1 ,0 ,0 ,0 ,0 ],
        &[0 ,1 ,0 ,1 ,1 ,1 ,1 ,1 , 1,1 ,1 ,1 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,18,1 ,18,0 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,1 ,0 ,1 ,0 ,1 ,0 ,0 ,0 ,0 ,18,1 ,1 ],
        &[1 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,0 ,9 ,9 ,9 ,0 ,18,1 ,1 ,1 ,0 ,18,1 ,1 ,1 ,0 ,1 ,1 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ],
        &[1 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,10,10,10,0 ,0 ,0 ,0 ,1 ,1 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ],
        &[1 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,1 ,0 ,18,1 ,1 ,1 ,1 ,1 ,18,0 ],
        &[1 ,1 ,0 ,0 ,1 ,28,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ],
        &[1 ,1 ,0 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,18,1 ,1 ,1 ,1 ,1 ,0 ,18,1 ,1 ,0 ,1 ,0 ],
        &[0 ,0 ,0 ,1 ,14,1 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,1 ,0 ,1 ,1 ,1 ,0 ],
        &[0 ,0 ,0 ,28,1 ,1 ,0 ,1 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,1 ,1 ,1 ,1 ,1 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,1 ,14,1 ,0 ,1 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,6 ,6 ,6 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,1 ,1 ,1 ,1 ,1 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,8 ,22,8 ,7 ,7 ,7 ,8 ,22,8 ,7 ,7 ,7 ,8 ,22,8 ,6 ,6 ,6 ,6 ,0 ,18,1 ,1 ,0 ,0 ,1 ,1 ,1 ,18,0 ],
        &[0 ,0 ,0 ,0 ,28,1 ,0 ,1 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,1 ,1 ,1 ,18,0 ,0 ,1 ,0 ],
        &[1 ,1 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ],
        &[0 ,0 ,1 ,0 ,0 ,0 ,0 ,17,0 ,0 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,1 ,0 ,0 ,1 ,1 ,1 ,1 ,0 ],
        &[0 ,0 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,8 ,8 ,1 ,1 ,18,1 ,1 ,1 ,0 ,0 ,1 ,0 ],
        &[0 ,0 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,0 ,6 ,6 ,6 ,6 ,6 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,6 ,6 ,6 ,8 ,6 ,0 ,0 ,1 ,0 ,0 ,1 ,18,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,1 ,0 ,6 ,6 ,6 ,6 ,6 ,8 ,22,8 ,7 ,7 ,7 ,8 ,20,8 ,7 ,7 ,7 ,8 ,22,8 ,6 ,6 ,6 ,8 ,6 ,0 ,0 ,1 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ],
        &[1 ,1 ,1 ,1 ,1 ,1 ,1 ,0 ,1 ,0 ,6 ,6 ,6 ,6 ,6 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,6 ,6 ,8 ,8 ,6 ,0 ,0 ,1 ,0 ,1 ,1 ,1 ,1 ,1 ,1 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,8 ,6 ,6 ,0 ,0 ,1 ,0 ,1 ,0 ,0 ,0 ,1 ,0 ],
        &[0 ,10,22,10,22,10,0 ,0 ,1 ,0 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,8 ,6 ,6 ,0 ,0 ,1 ,1 ,18,0 ,0 ,0 ,1 ,0 ],
        &[0 ,10,0 ,10,0 ,10,0 ,0 ,1 ,0 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,8 ,6 ,6 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,1 ,0 ],
        &[0 ,22,10,10,22,10,0 ,1 ,1 ,0 ,0 ,6 ,6 ,6 ,6 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,6 ,6 ,8 ,8 ,1 ,1 ,1 ,1 ,0 ,1 ,0 ,1 ,1 ,1 ,0 ],
        &[0 ,10,0 ,17,0 ,17,0 ,1 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,8 ,22,8 ,7 ,7 ,7 ,8 ,22,8 ,7 ,7 ,7 ,8 ,22,8 ,6 ,6 ,6 ,6 ,1 ,0 ,0 ,1 ,0 ,1 ,0 ,1 ,0 ,0 ,0 ],
        &[0 ,22,10,17,17,17,0 ,1 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,7 ,7 ,7 ,8 ,8 ,8 ,6 ,6 ,6 ,0 ,1 ,0 ,0 ,1 ,1 ,1 ,0 ,1 ,0 ,0 ,0 ],
        &[0 ,10,0 ,17,0 ,17,0 ,1 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ],
        &[0 ,10,10,17,17,17,17,1 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,0 ,0 ],
        &[0 ,22,0 ,22,0 ,17,17,1 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ],
        &[0 ,10,10,10,10,17,17,1 ,0 ,1 ,1 ,1 ,28,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,10,10,10,10,10,10,10,0 ],
        &[1 ,1 ,0 ,0 ,0 ,17,17,1 ,0 ,1 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,7 ,7 ,7 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,10,11,11,11,12,11,10,0 ],
        &[0 ,1 ,0 ,0 ,0 ,1 ,17,1 ,0 ,1 ,0 ,1 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,10,10,10,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,10,11,12,12,12,11,10,0 ],
        &[0 ,1 ,0 ,0 ,0 ,1 ,0 ,1 ,1 ,1 ,0 ,1 ,0 ,0 ,18,18,18,0 ,0 ,0 ,0 ,9 ,9 ,9 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,1 ,1 ,0 ,10,11,27,27,27,11,10,0 ],
        &[0 ,1 ,0 ,0 ,0 ,1 ,0 ,1 ,0 ,17,0 ,1 ,0 ,0 ,1 ,14,1 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,0 ,0 ,0 ,1 ,1 ,1 ,1 ,1 ,0 ,0 ,0 ,1 ,0 ,10,11,27,27,27,11,10,0 ],
        &[1 ,1 ,1 ,1 ,1 ,1 ,0 ,1 ,0 ,28,0 ,1 ,0 ,0 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,1 ,0 ,10,11,27,27,27,11,10,0 ],
        &[0 ,0 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,28,0 ,1 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,1 ,0 ,10,11,11,11,12,11,10,0 ],
        &[0 ,0 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,17,0 ,1 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,0 ,0 ,0 ,1 ,0 ,10,11,12,12,12,11,10,0 ],
        &[0 ,0 ,1 ,0 ,0 ,0 ,0 ,1 ,1 ,17,1 ,1 ,1 ,1 ,1 ,1 ,0 ,19,19,0 ,0 ,1 ,1 ,1 ,0 ,0 ,1 ,1 ,1 ,0 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,10,12,12,12,12,11,10,0 ],
        &[0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,1 ,19,0 ,0 ,1 ,1 ,1 ,0 ,0 ,18,18,18,0 ,1 ,0 ,0 ,0 ,0 ,1 ,1 ,10,11,11,11,11,11,10,0 ],
        &[0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,1 ,1 ,1 ,0 ,1 ,1 ,1 ,1 ,0 ,0 ,1 ,1 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,10,10,10,10,10,10,10,1 ],
        &[0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,1 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        ],
    };

const HOLY_SHRINE_OF_ANTINUK: GeneratorRoom = GeneratorRoom {
    name: "The Holy Shrine of Antinuk",
    dim: IVec2::new(45, 45),
    tiles: &[
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,9 ,9 ,9 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,10,10,10,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,29,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,6 ,6 ,6 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,29,6 ,6 ,6 ,29,6 ,6 ,6 ,6 ,6 ,8 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,9 ,10,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,8 ,8 ,8 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,10,9 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[1 ,1 ,29,1 ,1 ,1 ,1 ,1 ,9 ,10,7 ,7 ,8 ,7 ,7 ,7 ,8 ,7 ,7 ,7 ,8 ,8 ,16,8 ,8 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,29,10,9 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ,1 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,9 ,10,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,8 ,8 ,8 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,10,9 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,29,6 ,6 ,6 ,29,6 ,6 ,6 ,6 ,6 ,8 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,6 ,6 ,6 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,6 ,6 ,29,6 ,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,10,10,10,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,9 ,9 ,9 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,1 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
        ],
    };

const PLAYER_SPAWN_ROOM: GeneratorRoom = GeneratorRoom {
    name: "Ye Olde Pickaxe Town",
    dim: IVec2::new(21, 38),
    tiles: &[
            //=  -  .  -  =  -  .  -  =  -  .  -  =  -  .  -  =  -  .  -  =
            &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
            &[0 ,25,25,25,0 ,25,25,25,0 ,8 ,8 ,8 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ],
            &[0 ,25,25,25,25,25,25,25,17,8 ,15,8 ,17,25,25,25,25,25,25,25,0 ],
            &[0 ,25,0 ,0 ,0 ,25,25,25,0 ,8 ,8 ,8 ,0 ,0 ,0 ,25,25,25,25,25,0 ],
            &[0 ,25,0 ,0 ,0 ,0 ,25,0 ,0 ,0 ,8 ,0 ,0 ,0 ,0 ,0 ,25,0 ,0 ,0 ,0 ],
            &[0 ,25,25,25,25,25,25,25,25,17,8 ,17,25,25,25,25,25,25,25,25,0 ],
            &[0 ,27,27,27,27,27,27,27,0 ,0 ,8 ,0 ,0 ,27,27,27,27,27,27,27,0 ],
            &[0 ,27,27,27,27,27,27,27,0 ,8 ,8 ,8 ,0 ,27,27,27,27,27,27,27,0 ],
            &[0 ,27,27,27,26,27,27,27,0 ,8 ,13,8 ,0 ,27,27,27,26,27,27,27,0 ],
            &[0 ,27,27,26,0 ,26,27,27,0 ,8 ,8 ,8 ,0 ,27,27,26,0 ,26,27,27,0 ],
            &[0 ,27,27,27,26,27,27,27,0 ,0 ,17 ,0 ,0 ,27,27,27,26,27,27,27,0 ],
            &[0 ,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,0 ],
            &[0 ,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,0 ],
            &[0 ,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,0 ],
            &[0 ,27,27,27,26,27,27,27,27,27,27,27,27,27,27,27,26,27,27,27,0 ],
            &[0 ,27,27,26,0 ,26,27,27,27,27,27,27,27,27,27,26,0 ,26,27,27,0 ],
            &[26,27,27,27,26,27,27,11,12,11,11,11,27,27,27,27,26,27,27,27,26],
            &[26,27,27,27,27,27,27,11,12,12,12,11,27,27,27,27,27,27,27,27,26],
            &[26,27,27,27,27,27,27,11,12,12,12,11,27,27,27,27,27,27,27,27,26],
            &[0 ,27,27,27,27,27,27,11,11,11,11,11,27,27,27,27,27,27,27,27,0 ],
            &[0 ,26,27,27,26,27,27,27,27,27,27,27,27,27,27,27,26,27,27,26,0 ],
            &[0 ,0 ,25,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,0 ,25,0 ],
            &[0 ,26,27,27,26,27,27,27,27,27,27,27,27,27,27,27,26,27,27,26,0 ],
            &[0 ,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,0 ],
            &[26,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,26],
            &[26,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,26],
            &[26,27,27,27,26,27,27,27,27,27,27,27,27,27,27,27,26,27,27,27,26],
            &[0 ,27,27,26,0 ,26,27,27,27,27,27,27,27,27,27,26,0 ,26,27,27,0 ],
            &[0 ,27,27,27,26,27,27,27,27,27,27,27,27,27,27,27,26,27,27,27,0 ],
            &[0 ,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,0 ],
            &[0 ,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,0 ],
            &[0 ,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,27,0 ],
            &[0 ,27,27,27,26,27,27,27,27,27,27,27,27,27,27,27,26,27,27,27,0 ],
            &[0 ,11,12,11,0 ,26,27,27,27,27,27,27,27,27,27,26,0 ,26,27,27,0 ],
            &[0 ,12,12,12,11,27,27,27,0 ,27,27,27,0 ,27,27,27,26,27,27,27,0 ],
            &[0 ,12,12,12,11,27,27,0 ,0 ,26,26,26,0 ,0 ,27,27,26,27,27,27,0 ],
            &[0 ,12,12,12,11,27,27,0 ,6 ,26,26,26,6 ,0 ,27,27,26,27,27,27,0 ],
            &[0 ,14,12,12,0 ,26,0 ,0 ,6 ,26,26,26,6 ,0 ,0 ,26,0 ,26,27,27,0 ],
            &[0 ,0 ,0 ,0 ,0 ,0 ,0 ,6 ,6 ,26,20,26,6 ,6 ,6 ,0 ,0 ,0 ,0 ,0 ,0 ],
        ],
    };

const DUNGEON_DIM: IVec2 = IVec2::new(128, 128);
const DUNGEON_SIZE: usize = (DUNGEON_DIM.x * DUNGEON_DIM.y) as usize;

const PIT_DEPTH: i32 = 5;

const ROOM_ATTEMPTS: u32 = 100;

fn is_room_valid(
    dungeon_tiles: &[u32; DUNGEON_SIZE],
    start_coord: &IVec2,
    room: &GeneratorRoom,
) -> bool {
    for y in 0..room.dim.y {
        for x in 0..room.dim.x {
            let dungeon_index = ((x + start_coord.x) + (y + start_coord.y) * DUNGEON_DIM.x) as usize;
            if dungeon_tiles[dungeon_index] != 0 {
                return false;
            }
        }
    }

    true
}

fn add_room(
    dungeon_tiles: &mut [u32; DUNGEON_SIZE],
    start_coord: &IVec2,
    room: &GeneratorRoom,
) {
    for y in 0..room.dim.y {
        let y_slice = room.tiles[y as usize];
        for x in 0..room.dim.x {
            let dungeon_index = ((x + start_coord.x) + (y + start_coord.y) * DUNGEON_DIM.x) as usize;
            dungeon_tiles[dungeon_index] = y_slice[x as usize];
        }
    }
}

const FULL_SEARCH_DIRECTIONS: [IVec2; 8] = [
    IVec2::NEG_X,
    IVec2::X,
    IVec2::NEG_Y,
    IVec2::Y,
    IVec2::new(-1, -1),
    IVec2::new(-1,  1),
    IVec2::new( 1, -1),
    IVec2::new( 1,  1),
];

fn check_tile_and_neighbors(
    dungeon_tiles: &[u32; DUNGEON_SIZE],
    tile_coord: &IVec2,
    check_for: u32,
) -> bool {
    let tile_index = (tile_coord.x + tile_coord.y * DUNGEON_DIM.x) as usize;
    if dungeon_tiles[tile_index] != check_for {
        return false;
    }

    if tile_coord.x >= 1 && tile_coord.x < DUNGEON_DIM.x - 1 && tile_coord.y >= 1 && tile_coord.y < DUNGEON_DIM.y - 1 {
        for i in 0..8 {
            let search_coord = FULL_SEARCH_DIRECTIONS[i] + *tile_coord;
            if dungeon_tiles[(search_coord.x + search_coord.y * DUNGEON_DIM.x) as usize] != check_for {
                return false;
            }
        }

        return true;
    }

    return false;
}

fn get_neighbors(
    dungeon_tiles: &[u32; DUNGEON_SIZE],
    tile_coord: &IVec2,
) -> [u32; 4] {
    let mut neighbors = [u32::MAX; 4];

    if tile_coord.x >= 1 {
        neighbors[0] = dungeon_tiles[((tile_coord.x - 1) + tile_coord.y * DUNGEON_DIM.x) as usize];
    }
    if tile_coord.x < DUNGEON_DIM.x - 1 {
        neighbors[1] = dungeon_tiles[((tile_coord.x + 1) + tile_coord.y * DUNGEON_DIM.x) as usize];
    }
    if tile_coord.y >= 1 {
        neighbors[2] = dungeon_tiles[(tile_coord.x + (tile_coord.y - 1) * DUNGEON_DIM.x) as usize];
    }
    if tile_coord.y < DUNGEON_DIM.y - 1 {
        neighbors[3] = dungeon_tiles[(tile_coord.x + (tile_coord.y + 1) * DUNGEON_DIM.x) as usize];
    }

    neighbors
}

fn is_tile(
    dungeon_tiles: &[u32; DUNGEON_SIZE],
    tile_coord: &IVec2,
    check_for: u32,
) -> bool {
    dungeon_tiles[(tile_coord.x + tile_coord.y * DUNGEON_DIM.x) as usize] == check_for
}

fn is_border_tile(
    tile_coord: &IVec2,
) -> bool {
    tile_coord.x == 0 || tile_coord.x == DUNGEON_DIM.x - 1 || tile_coord.y == 0 || tile_coord.y == DUNGEON_DIM.y - 1
}

pub fn generate_dungeon(
    commands: &mut Commands,
    root: &mut GridRoot,
    tasks: &mut ResMut<Tasks>,
    asset_map: &Res<AssetMap>,
) -> IVec3 {
    let mut search_tiles = [0; DUNGEON_SIZE];
    let mut dungeon_tiles = [0; DUNGEON_SIZE];
    let mut rng = thread_rng();



    // attempt to add rooms
    let player_spawn_room = &PLAYER_SPAWN_ROOM;
    // let mut player_spawn_room_coord = IVec2::new(rng.gen_range(1..DUNGEON_DIM.x - player_spawn_room.dim.x - 1), rng.gen_range(1..DUNGEON_DIM.y - player_spawn_room.dim.y - 1));
    let mut player_spawn_room_coord = IVec2::new(DUNGEON_DIM.x / 2 - player_spawn_room.dim.x / 2, DUNGEON_DIM.y / 2 - player_spawn_room.dim.y / 2);
    add_room(&mut dungeon_tiles, &player_spawn_room_coord, player_spawn_room);
    for y in -1..player_spawn_room.dim.y + 1 { for x in -1..player_spawn_room.dim.x + 1 {
        search_tiles[((x + player_spawn_room_coord.x) + (y + player_spawn_room_coord.y) * DUNGEON_DIM.x) as usize] = 8;
    }}
    for y in 0..player_spawn_room.dim.y { for x in 0..player_spawn_room.dim.x {
        search_tiles[((x + player_spawn_room_coord.x) + (y + player_spawn_room_coord.y) * DUNGEON_DIM.x) as usize] = 16;
    }}



    let mut shrine_spawned = false;
    let holy_shrine_of_antinuk = &HOLY_SHRINE_OF_ANTINUK;
    while !shrine_spawned {
        let mut holy_shrine_coord = IVec2::new(rng.gen_range(1..DUNGEON_DIM.x - holy_shrine_of_antinuk.dim.x - 1), rng.gen_range(1..DUNGEON_DIM.y - holy_shrine_of_antinuk.dim.y - 1));

        if is_room_valid(&dungeon_tiles, &holy_shrine_coord, holy_shrine_of_antinuk) {
            shrine_spawned = true;
            add_room(&mut dungeon_tiles, &holy_shrine_coord, holy_shrine_of_antinuk);
            for y in -1..holy_shrine_of_antinuk.dim.y + 1 - 8 { for x in -1..holy_shrine_of_antinuk.dim.x + 1 - 8 {
                search_tiles[((x + holy_shrine_coord.x) + (y + holy_shrine_coord.y) * DUNGEON_DIM.x) as usize] = 8;
            }}
            for y in 0..holy_shrine_of_antinuk.dim.y - 8 { for x in 0..holy_shrine_of_antinuk.dim.x - 8 {
                search_tiles[((x + holy_shrine_coord.x) + (y + holy_shrine_coord.y) * DUNGEON_DIM.x) as usize] = 16;
            }}
        }
    }



    let mut chambers_spawned = false;
    let ikthillions_chambers = &IKTHILLIONS_CHAMBERS;
    while !chambers_spawned {
        let mut chambers_coord = IVec2::new(rng.gen_range(1..DUNGEON_DIM.x - ikthillions_chambers.dim.x - 1), rng.gen_range(1..DUNGEON_DIM.y - ikthillions_chambers.dim.y - 1));

        if is_room_valid(&dungeon_tiles, &chambers_coord, ikthillions_chambers) {
            chambers_spawned = true;
            add_room(&mut dungeon_tiles, &chambers_coord, ikthillions_chambers);
            for y in -1..ikthillions_chambers.dim.y + 1 - 8 { for x in -1..ikthillions_chambers.dim.x + 1 - 8 {
                search_tiles[((x + chambers_coord.x) + (y + chambers_coord.y) * DUNGEON_DIM.x) as usize] = 8;
            }}
            for y in 0..ikthillions_chambers.dim.y - 8 { for x in 0..ikthillions_chambers.dim.x - 8 {
                search_tiles[((x + chambers_coord.x) + (y + chambers_coord.y) * DUNGEON_DIM.x) as usize] = 16;
            }}
        }
    }



    // generic rooms
    for attempt in 0..ROOM_ATTEMPTS {
        let room = &GENERATOR_ROOMS[rng.gen_range(0..GENERATOR_ROOMS.len())];
        let mut start_coord = IVec2::new(rng.gen_range(1..DUNGEON_DIM.x - room.dim.x - 1), rng.gen_range(1..DUNGEON_DIM.y - room.dim.y - 1));

        if is_room_valid(&dungeon_tiles, &start_coord, room) {
            add_room(&mut dungeon_tiles, &start_coord, room);
            for y in -1..room.dim.y + 1 { for x in -1..room.dim.x + 1 {
                search_tiles[((x + start_coord.x) + (y + start_coord.y) * DUNGEON_DIM.x) as usize] = 8;
            }}
            for y in 0..room.dim.y { for x in 0..room.dim.x {
                search_tiles[((x + start_coord.x) + (y + start_coord.y) * DUNGEON_DIM.x) as usize] = 16;
            }}
        }
    }

    // collect valid hall start points
    let mut valid_hallway_roots: Vec<IVec2> = vec![];
    for z in 0..DUNGEON_DIM.y { for x in 0..DUNGEON_DIM.x {
        let coord = IVec2::new(x, z);
        if check_tile_and_neighbors(&search_tiles, &coord, 0) {
            valid_hallway_roots.push(coord);
        }
    }}

    let hallway_root = valid_hallway_roots[rng.gen_range(0..valid_hallway_roots.len())];
    
    // depth search hallways
    // 0: unsearched, unbacktracked
    // 1: searched
    // 2: backtracked
    const SEARCH_DIRECTIONS: [IVec2; 4] = [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y];
    let mut hallway_tiles: Vec<IVec2> = vec![hallway_root];
    let mut iter = 0;
    loop {
        if let Some(tile_coord) = hallway_tiles.pop() {
            let neighbors = get_neighbors(&search_tiles, &tile_coord);

            let mut all_new = true;
            for i in 0..4 {
                if neighbors[i] != 0 {
                    all_new = false;
                }
            }

            if all_new {
                hallway_tiles.push(tile_coord + SEARCH_DIRECTIONS[rng.gen_range(0..4)]);
                continue;
            } else {
                if neighbors[0] == 0 {
                    let neighbor_coord = IVec2::new(tile_coord.x - 1, tile_coord.y);
                    if !is_border_tile(&neighbor_coord) {
                        if is_tile(&search_tiles, &(neighbor_coord + IVec2::Y), 0) && is_tile(&search_tiles, &(neighbor_coord - IVec2::Y), 0) {
                            search_tiles[(neighbor_coord.x + neighbor_coord.y * DUNGEON_DIM.x) as usize] = 1;
                            hallway_tiles.push(neighbor_coord);
                            continue;
                        }
                    }

                    search_tiles[(neighbor_coord.x + neighbor_coord.y * DUNGEON_DIM.x) as usize] = 4;
                }
                if neighbors[1] == 0 {
                    let neighbor_coord = IVec2::new(tile_coord.x + 1, tile_coord.y);
                    if !is_border_tile(&neighbor_coord) {
                        if is_tile(&search_tiles, &(neighbor_coord + IVec2::Y), 0) && is_tile(&search_tiles, &(neighbor_coord - IVec2::Y), 0) {
                            search_tiles[(neighbor_coord.x + neighbor_coord.y * DUNGEON_DIM.x) as usize] = 1;
                            hallway_tiles.push(neighbor_coord);
                            continue;
                        }
                    }
                    
                    search_tiles[(neighbor_coord.x + neighbor_coord.y * DUNGEON_DIM.x) as usize] = 4;
                }
                if neighbors[2] == 0 {
                    let neighbor_coord = IVec2::new(tile_coord.x, tile_coord.y - 1);
                    if !is_border_tile(&neighbor_coord) {
                        if is_tile(&search_tiles, &(neighbor_coord + IVec2::X), 0) && is_tile(&search_tiles, &(neighbor_coord - IVec2::X), 0) {
                            search_tiles[(neighbor_coord.x + neighbor_coord.y * DUNGEON_DIM.x) as usize] = 1;
                            hallway_tiles.push(neighbor_coord);
                            continue;
                        }
                    }
                    
                    search_tiles[(neighbor_coord.x + neighbor_coord.y * DUNGEON_DIM.x) as usize] = 4;
                }
                if neighbors[3] == 0 {
                    let neighbor_coord = IVec2::new(tile_coord.x, tile_coord.y + 1);
                    if !is_border_tile(&neighbor_coord) {
                        if is_tile(&search_tiles, &(neighbor_coord + IVec2::X), 0) && is_tile(&search_tiles, &(neighbor_coord - IVec2::X), 0) {
                            search_tiles[(neighbor_coord.x + neighbor_coord.y * DUNGEON_DIM.x) as usize] = 1;
                            hallway_tiles.push(neighbor_coord);
                            continue;
                        }
                    }
                    
                    search_tiles[(neighbor_coord.x + neighbor_coord.y * DUNGEON_DIM.x) as usize] = 4;
                }
            }

            search_tiles[(tile_coord.x + tile_coord.y * DUNGEON_DIM.x) as usize] = 2;
            iter += 1;
            
            if neighbors[0] == 1 {
                hallway_tiles.push(tile_coord + SEARCH_DIRECTIONS[0]);
            }
            if neighbors[1] == 1 {
                hallway_tiles.push(tile_coord + SEARCH_DIRECTIONS[1]);
            }
            if neighbors[2] == 1 {
                hallway_tiles.push(tile_coord + SEARCH_DIRECTIONS[2]);
            }
            if neighbors[3] == 1 {
                hallway_tiles.push(tile_coord + SEARCH_DIRECTIONS[3]);
            }
        } else {
            let mut valid_hallway_roots: Vec<IVec2> = vec![];
            for z in 0..DUNGEON_DIM.y { for x in 0..DUNGEON_DIM.x {
                let coord = IVec2::new(x, z);
                if check_tile_and_neighbors(&search_tiles, &coord, 0) {
                    valid_hallway_roots.push(coord);
                }
            }}
        
            if !valid_hallway_roots.is_empty() {
                let hallway_root = valid_hallway_roots[rng.gen_range(0..valid_hallway_roots.len())];
                hallway_tiles.push(hallway_root);
            } else {
                break;
            }
        }
    }

    // search for doors
    let mut doors_to_place: Vec<IVec2> = vec![];
    for z in 0..DUNGEON_DIM.y { for x in 0..DUNGEON_DIM.x {
        let door_coord = IVec2::new(x, z);
        if !is_border_tile(&door_coord) && (is_tile(&search_tiles, &door_coord, 0) || is_tile(&search_tiles, &door_coord, 4) || is_tile(&search_tiles, &door_coord, 8)) {
            if is_tile(&search_tiles, &(door_coord + SEARCH_DIRECTIONS[0]), 2) || is_tile(&search_tiles, &(door_coord + SEARCH_DIRECTIONS[0]), 16) {
                if is_tile(&search_tiles, &(door_coord + SEARCH_DIRECTIONS[1]), 2) || is_tile(&search_tiles, &(door_coord + SEARCH_DIRECTIONS[1]), 16) {
                    doors_to_place.push(door_coord);

                }
            }
            // if !is_tile(&search_tiles, &(door_coord + SEARCH_DIRECTIONS[2]), 0) && !is_tile(&search_tiles, &(door_coord + SEARCH_DIRECTIONS[3]), 0) {
            //     doors_to_place.push(door_coord);
            // }
        }
    }}

    // place doors
    for door in doors_to_place.iter() {
        root.set_tile_on_at_coord(&IVec3::new(door.x,  0, door.y), &GridTile::new(TileType::Open, TileMatter::Wood));
        root.set_tile_on_at_coord(&IVec3::new(door.x, -1, door.y), &GridTile::new(TileType::Block, TileMatter::Wood));
    }

    const RNG_PASSIVE_CREATURES: &'static [CreatureType] = &[
        CreatureType::EvilChicken,
        CreatureType::EvilCat,
        CreatureType::EvilPidgeon,
        CreatureType::EvilPeasant,
        CreatureType::EvilAnime,
    ];
    const RNG_AGGRO_CREATURES: &'static [CreatureType] = &[
        CreatureType::EvilNinja,
        CreatureType::EvilWizard,
        CreatureType::EvilLizard,
        CreatureType::EvilWhelp,
        CreatureType::EvilFish,
        CreatureType::EvilGuardKnight,
    ];

    let mut player_spawn_point = IVec3::ZERO;
    // place tiles
    for z in 0..DUNGEON_DIM.y {
        for x in 0..DUNGEON_DIM.x {
            let tile_index = (x + z * DUNGEON_DIM.x) as usize;
            let tile_position = IVec3::new(x, 0, z);
            if search_tiles[tile_index] == 2 {
                root.set_tile_on_at_coord(&tile_position, &GridTile::new(TileType::Open, TileMatter::Stone));
                root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::StonePath));
                if rng.gen::<f32>() < 0.05 { tasks.spawn_creature(&tile_position, &IVec3::Z, RNG_AGGRO_CREATURES[rng.gen_range(0..RNG_AGGRO_CREATURES.len())]); }
            }
            match dungeon_tiles[tile_index] {
                0 => { /* Background */ }
                1 => { // Room
                    root.set_tile_on_at_coord(&tile_position, &GridTile::new(TileType::Open, TileMatter::Stone));
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::StonePath));
                    if rng.gen::<f32>() < 0.05 { tasks.spawn_creature(&tile_position, &IVec3::Z, RNG_AGGRO_CREATURES[rng.gen_range(0..RNG_AGGRO_CREATURES.len())]); }
                },
                2 => { // Hall
                    // some kind of hallway material?
                    root.set_tile_on_at_coord(&tile_position, &GridTile::new(TileType::Open, TileMatter::Stone));
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::StonePath));
                    if rng.gen::<f32>() < 0.05 { tasks.spawn_creature(&tile_position, &IVec3::Z, RNG_AGGRO_CREATURES[rng.gen_range(0..RNG_AGGRO_CREATURES.len())]); }
                },
                3 => { // Door
                    // door scene?
                    root.set_tile_on_at_coord(&tile_position, &GridTile::new(TileType::Open, TileMatter::Stone));
                },
                4 => { // Pit
                    for y in -PIT_DEPTH..1 {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    root.set_tile_on_at_coord(&(tile_position + IVec3::Y * (-PIT_DEPTH - 1)), &GridTile::new(TileType::Block, TileMatter::Void));
                },
                5 => { // Pit Bridge
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    for y in -PIT_DEPTH..-1 {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    root.set_tile_on_at_coord(&(tile_position + IVec3::Y * (-PIT_DEPTH - 1)), &GridTile::new(TileType::Block, TileMatter::Void));
                },
                6 => { // Cavern
                    for y in -PIT_DEPTH..=PIT_DEPTH {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    root.set_tile_on_at_coord(&(tile_position + IVec3::Y * (-PIT_DEPTH - 1)), &GridTile::new(TileType::Block, TileMatter::Void));
                },
                7 => { // Cavern Bridge
                    for y in -PIT_DEPTH..-1 {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    for y in 0..=PIT_DEPTH {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                },
                8 => { // Pillar
                    for y in 0..=PIT_DEPTH {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                },
                9 => { // DoubleRoom
                    for y in 0..=1 {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::StonePath));
                    if rng.gen::<f32>() < 0.35 { tasks.spawn_creature(&tile_position, &IVec3::Z, RNG_PASSIVE_CREATURES[rng.gen_range(0..RNG_PASSIVE_CREATURES.len())]); }
                },
                10 => { // TripleRoom
                    for y in 0..=2 {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::StonePath));
                    if rng.gen::<f32>() < 0.35 { tasks.spawn_creature(&tile_position, &IVec3::Z, RNG_PASSIVE_CREATURES[rng.gen_range(0..RNG_PASSIVE_CREATURES.len())]); }
                },
                11 => { // Wall
                    for y in 0..=1 {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Block, TileMatter::Stone));
                    }
                    for y in 2..=2 {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                },
                12 => { // Roof
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    root.set_tile_on_at_coord(&(tile_position + IVec3::Y * 1), &GridTile::new(TileType::Block, TileMatter::Stone));
                    for y in 2..=2 {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::StonePath));
                    if rng.gen::<f32>() < 0.7 { tasks.spawn_creature(&tile_position, &IVec3::Z, RNG_PASSIVE_CREATURES[rng.gen_range(0..RNG_PASSIVE_CREATURES.len())]); }
                },
                13 => { // PlayerSpawner
                    for y in 0..=PIT_DEPTH {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::StonePath));
                    player_spawn_point = tile_position;
                    root.set_tile_on_at_coord(&tile_position, &GridTile::new(TileType::PlayerSpawner, TileMatter::Stone));
                },
                14 => { // WeaponSpawner
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    const WEAPON_SPAWNER_INV_TYPES: [InventoryItemType; 3] = [InventoryItemType::Katana, InventoryItemType::Sword, InventoryItemType::Axe];
                    const WEAPON_SPAWNER_SCENE_TYPES: [SceneAsset; 3] = [SceneAsset::Katana, SceneAsset::Sword, SceneAsset::Axe];
                    let rand = rng.gen_range(0..3);
                    let random_weapon = commands.spawn(GridItem::new(&tile_position, WEAPON_SPAWNER_INV_TYPES[rand]))
                        .insert(SceneBundle {
                            scene: asset_map.get_scene_handle(WEAPON_SPAWNER_SCENE_TYPES[rand]),
                            transform: Transform::from_translation(tile_position.as_vec3()),
                            ..default()
                        })
                        .id();
                    root.set_item_at_coord(&tile_position, &Some(random_weapon));
                },
                15 => { // PickaxeSpawner
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    let pickaxe = commands.spawn(GridItem::new(&tile_position, InventoryItemType::Pickaxe))
                        .insert(SceneBundle {
                            scene: asset_map.get_scene_handle(SceneAsset::Pickaxe),
                            transform: Transform::from_translation(tile_position.as_vec3()),
                            ..default()
                        })
                        .id();
                    root.set_item_at_coord(&tile_position, &Some(pickaxe));
                    
                },
                16 => { // HolyHandgunOfAntinukPedastal
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    let holy_handgun_of_antinuk = commands.spawn(GridItem::new(&tile_position, InventoryItemType::HolyHandgunOfAntinuk))
                        .insert(SceneBundle {
                            scene: asset_map.get_scene_handle(SceneAsset::HolyHandgunOfAntinuk),
                            transform: Transform::from_translation(tile_position.as_vec3()),
                            ..default()
                        })
                        .id();
                    root.set_item_at_coord(&tile_position, &Some(holy_handgun_of_antinuk));
                    
                },
                17 => { // Dirt
                    root.set_tile_on_at_coord(&tile_position, &GridTile::new(TileType::Block, TileMatter::Dirt));
                },
                18 => { // EvilNinjaSpawner
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    tasks.spawn_creature(&tile_position, &IVec3::Z, CreatureType::EvilNinja);
                },
                19 => { // EvilWizardSpawner
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    tasks.spawn_creature(&tile_position, &IVec3::Z, CreatureType::EvilWizard);
                },
                20 => { // EvilIkthillionSpawner
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    tasks.spawn_creature(&tile_position, &IVec3::Z, CreatureType::EvilSkeleton);
                },
                21 => { // EvilFishSpawner
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::Gooey));
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    tasks.spawn_creature(&tile_position, &IVec3::Z, CreatureType::EvilFish);
                },
                22 => { // EvilWhelpSpawner
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    tasks.spawn_creature(&tile_position, &IVec3::Z, CreatureType::EvilWhelp);
                },
                23 => { // EvilChickenSpawner
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    tasks.spawn_creature(&tile_position, &IVec3::Z, CreatureType::EvilChicken);
                },
                24 => { // EvilLizardSpawner
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::Gooey));
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    tasks.spawn_creature(&tile_position, &IVec3::Z, CreatureType::EvilLizard);
                },
                25 => { // 
                    root.set_tile_on_at_coord(&tile_position, &GridTile::new(TileType::Open, TileMatter::Stone));
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::Wood));
                    if rng.gen::<f32>() < 0.1 { tasks.spawn_creature(&tile_position, &IVec3::Z, RNG_PASSIVE_CREATURES[rng.gen_range(0..RNG_PASSIVE_CREATURES.len())]); }
                },
                26 => { // 
                    for y in 0..=1 {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::Wood));
                    if rng.gen::<f32>() < 0.1 { tasks.spawn_creature(&tile_position, &IVec3::Z, RNG_PASSIVE_CREATURES[rng.gen_range(0..RNG_PASSIVE_CREATURES.len())]); }
                },
                27 => { // 
                    for y in 0..=2 {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    root.set_tile_on_at_coord(&(tile_position - IVec3::Y), &GridTile::new(TileType::Block, TileMatter::Wood));
                    if rng.gen::<f32>() < 0.1 { tasks.spawn_creature(&tile_position, &IVec3::Z, RNG_PASSIVE_CREATURES[rng.gen_range(0..RNG_PASSIVE_CREATURES.len())]); }
                },
                28 => { // 
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    tasks.spawn_creature(&tile_position, &IVec3::Z, CreatureType::EvilGuardKnight);
                },
                29 => { // 
                    for y in 0..=PIT_DEPTH {
                        root.set_tile_on_at_coord(&(tile_position + IVec3::Y * y), &GridTile::new(TileType::Open, TileMatter::Stone));
                    }
                    tasks.spawn_creature(&tile_position, &IVec3::Z, CreatureType::EvilAngel);
                },
                30 => { // 
                    root.set_tile_on_at_coord(&(tile_position), &GridTile::new(TileType::Open, TileMatter::Stone));
                    let random_weapon = commands.spawn(GridItem::new(&tile_position, InventoryItemType::Katana))
                        .insert(SceneBundle {
                            scene: asset_map.get_scene_handle(SceneAsset::Katana),
                            transform: Transform::from_translation(tile_position.as_vec3()),
                            ..default()
                        })
                        .id();
                    root.set_item_at_coord(&tile_position, &Some(random_weapon));
                },
                _ => { println!("null"); },
            };
        }
    }

    player_spawn_point
}