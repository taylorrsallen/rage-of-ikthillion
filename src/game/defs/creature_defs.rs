use crate::*;

//================================-================================-================================
#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum CreatureType {
    #[default]
    EvilNinja,
    EvilWizard,
    EvilLizard,
    // EvilWolfKnight,
    EvilWhelp,
    EvilSkeleton,
    EvilFish,
    EvilChicken,
    EvilCat,
    EvilPidgeon,
    EvilPeasant,
    EvilAnime,
    EvilAngel,
    EvilGuardKnight,
}

impl CreatureType {
    pub fn get_def(
        &self,
    ) -> &CreatureTypeDef {
        &CREATURE_TYPE_DEFS[*self as usize]
    }
}

pub struct CreatureTypeDef {
    pub name: &'static str,
    pub scene: SceneAsset,

    pub weapon: WeaponType,
    pub aggressive: bool,

    pub base_vitality: u32,
    pub base_endurance: u32,
    pub base_strength: u32,
    pub base_dexterity: u32,
    pub base_luck: u32,

    pub base_level: u32,
    pub base_experience: f32,

    pub move_cd: f32,
    death_sounds: &'static [SoundAsset],
    hurt_sounds: &'static [SoundAsset],
}

impl CreatureTypeDef {
    pub fn get_death_sound(
        &self,
    ) -> SoundAsset {
        self.death_sounds[thread_rng().gen_range(0..self.death_sounds.len())]
    }

    pub fn get_hurt_sound(
        &self,
    ) -> SoundAsset {
        self.hurt_sounds[thread_rng().gen_range(0..self.hurt_sounds.len())]
    }
}

pub const CREATURE_TYPE_DEFS: &'static [CreatureTypeDef] = &[
    CreatureTypeDef {
        name: "Evil Ninja",
        scene: SceneAsset::EvilNinja,

        weapon: WeaponType::RapidMonsterClaw,
        aggressive: true,

        base_vitality: 10,
        base_endurance: 10,
        base_strength: 15,
        base_dexterity: 45,
        base_luck: 35,
    
        base_level: 10,
        base_experience: 1_000.0,

        move_cd: 0.6,
        death_sounds: &[
            SoundAsset::MaleDeath00,
            SoundAsset::MaleDeath02,
            SoundAsset::MaleDeath05,
        ],
        hurt_sounds: &[
            SoundAsset::MaleHurt00,
            SoundAsset::MaleHurt01,
            SoundAsset::MaleHurt02,
        ],
    },
    CreatureTypeDef {
        name: "Evil Wizard",
        scene: SceneAsset::EvilWizard,

        weapon: WeaponType::WeakMonsterClaw,
        aggressive: true,

        base_vitality: 1,
        base_endurance: 1,
        base_strength: 1,
        base_dexterity: 1,
        base_luck: 1,
    
        base_level: 15,
        base_experience: 1_000.0,

        move_cd: 1.5,
        death_sounds: &[
            SoundAsset::GoblinDeath00,
        ],
        hurt_sounds: &[
            SoundAsset::MaleHurt03,
            SoundAsset::MaleHurt04,
        ],
    },
    CreatureTypeDef {
        name: "Evil Lizard",
        scene: SceneAsset::EvilLizard,
        
        weapon: WeaponType::StandardMonsterClaw,
        aggressive: true,

        base_vitality: 50,
        base_endurance: 50,
        base_strength: 50,
        base_dexterity: 50,
        base_luck: 50,
    
        base_level: 40,
        base_experience: 25.0,
        
        move_cd: 2.5,
        death_sounds: &[
            SoundAsset::MonsterDeath00,
            SoundAsset::MonsterDeath01,
            SoundAsset::MonsterDeath02,
        ],
        hurt_sounds: &[
            SoundAsset::MonsterHurt00,
            SoundAsset::MonsterHurt01,
            SoundAsset::MonsterHurt02,
        ],
    },
    CreatureTypeDef {
        name: "Evil Whelp",
        scene: SceneAsset::EvilWhelp,
        
        weapon: WeaponType::RapidMonsterClaw,
        aggressive: true,

        base_vitality: 100,
        base_endurance: 100,
        base_strength: 100,
        base_dexterity: 100,
        base_luck: 100,
    
        base_level: 50,
        base_experience: 50.0,
        
        move_cd: 0.5,
        death_sounds: &[
            SoundAsset::WhelpDeath00,
        ],
        hurt_sounds: &[
            SoundAsset::MonsterHurt02,
        ],
    },
    CreatureTypeDef {
        name: "Evil Skeleton",
        scene: SceneAsset::EvilSkeleton,
        
        weapon: WeaponType::IkthillionsMonsterClaw,
        aggressive: true,

        base_vitality: 750,
        base_endurance: 600,
        base_strength: 750,
        base_dexterity: 75,
        base_luck: 1,
    
        base_level: 2_000,
        base_experience: 1_000.0,
        
        move_cd: 1.0,
        death_sounds: &[
            SoundAsset::IkthillionDeath00,
        ],
        hurt_sounds: &[
            SoundAsset::MonsterHurt01,
            SoundAsset::MaleHurt03,
        ],
    },
    CreatureTypeDef {
        name: "Evil Fish",
        scene: SceneAsset::EvilFish,
        
        weapon: WeaponType::LargeMonsterClaw,
        aggressive: true,

        base_vitality: 75,
        base_endurance: 75,
        base_strength: 75,
        base_dexterity: 75,
        base_luck: 75,
    
        base_level: 35,
        base_experience: 25.0,
        
        move_cd: 1.0,
        death_sounds: &[
            SoundAsset::MonsterDeath00,
            SoundAsset::MonsterDeath01,
            SoundAsset::MonsterDeath02,
        ],
        hurt_sounds: &[
            SoundAsset::MonsterHurt00,
            SoundAsset::MonsterHurt01,
            SoundAsset::MonsterHurt02,
        ],
    },
    CreatureTypeDef {
        name: "Evil Chicken",
        scene: SceneAsset::EvilChicken,
        
        weapon: WeaponType::WeakMonsterClaw,
        aggressive: false,

        base_vitality: 1,
        base_endurance: 1,
        base_strength: 1,
        base_dexterity: 10,
        base_luck: 10,
    
        base_level: 10,
        base_experience: 1.0,
        
        move_cd: 1.0,
        death_sounds: &[
            SoundAsset::ChickenDeath00,
            SoundAsset::ChickenDeath01,
            SoundAsset::ChickenDeath02,
        ],
        hurt_sounds: &[
            SoundAsset::ChickenHurt00,
            SoundAsset::ChickenHurt01,
        ],
    },
    CreatureTypeDef {
        name: "Evil Cat",
        scene: SceneAsset::EvilCat,
        
        weapon: WeaponType::WeakMonsterClaw,
        aggressive: false,

        base_vitality: 1,
        base_endurance: 1,
        base_strength: 1,
        base_dexterity: 20,
        base_luck: 20,
    
        base_level: 15,
        base_experience: 1.0,
        
        move_cd: 1.0,
        death_sounds: &[
            SoundAsset::CatDeath00,
            SoundAsset::CatDeath01,
            SoundAsset::CatDeath02,
            SoundAsset::CatDeath03,
        ],
        hurt_sounds: &[
            SoundAsset::CatHurt00,
            SoundAsset::CatHurt01,
        ],
    },
    CreatureTypeDef {
        name: "Evil Pidgeon",
        scene: SceneAsset::EvilPidgeon,
        
        weapon: WeaponType::WeakMonsterClaw,
        aggressive: false,

        base_vitality: 1,
        base_endurance: 1,
        base_strength: 1,
        base_dexterity: 30,
        base_luck: 30,
    
        base_level: 10,
        base_experience: 1.0,
        
        move_cd: 1.0,
        death_sounds: &[
            SoundAsset::PidgeonDeath00,
        ],
        hurt_sounds: &[
            SoundAsset::ChickenHurt00,
            SoundAsset::ChickenHurt01,
            SoundAsset::CatHurt01,
        ],
    },
    CreatureTypeDef {
        name: "Evil Peasant",
        scene: SceneAsset::EvilPeasant,
        
        weapon: WeaponType::WeakMonsterClaw,
        aggressive: false,

        base_vitality: 10,
        base_endurance: 10,
        base_strength: 10,
        base_dexterity: 40,
        base_luck: 40,
    
        base_level: 30,
        base_experience: 1.0,
        
        move_cd: 4.0,
        death_sounds: &[
            SoundAsset::MaleDeath01,
            SoundAsset::MaleDeath03,
            SoundAsset::MaleDeath04,
            SoundAsset::MaleDeath06,
            SoundAsset::MaleDeath07,
            SoundAsset::MaleDeath08,
            SoundAsset::MaleDeath09,
            SoundAsset::MaleDeath10,
            SoundAsset::MaleDeath11,
            SoundAsset::MaleDeath12,
            SoundAsset::MaleDeath13,
        ],
        hurt_sounds: &[
            SoundAsset::MaleHurt00,
            SoundAsset::MaleHurt01,
            SoundAsset::MaleHurt02,
            SoundAsset::MaleHurt03,
            SoundAsset::MaleHurt04,
        ],
    },
    CreatureTypeDef {
        name: "Evil Anime",
        scene: SceneAsset::EvilAnime,
        
        weapon: WeaponType::WeakMonsterClaw,
        aggressive: false,

        base_vitality: 5,
        base_endurance: 5,
        base_strength: 5,
        base_dexterity: 60,
        base_luck: 60,
    
        base_level: 25,
        base_experience: 1.0,
        
        move_cd: 4.0,
        death_sounds: &[
            SoundAsset::FemaleDeath00,
            SoundAsset::FemaleDeath01,
            SoundAsset::FemaleDeath02,
            SoundAsset::FemaleDeath03,
            SoundAsset::FemaleDeath04,
            SoundAsset::FemaleDeath05,
        ],
        hurt_sounds: &[
            SoundAsset::JapaneseHurt00,
            SoundAsset::JapaneseHurt01,
            SoundAsset::JapaneseHurt02,
            SoundAsset::JapaneseHurt03,
        ],
    },
    CreatureTypeDef {
        name: "Evil Angel",
        scene: SceneAsset::EvilAngel,
        
        weapon: WeaponType::HeavyMonsterClaw,
        aggressive: true,

        base_vitality: 100,
        base_endurance: 100,
        base_strength: 150,
        base_dexterity: 30,
        base_luck: 2,
    
        base_level: 50,
        base_experience: 1.0,
        
        move_cd: 3.0,
        death_sounds: &[
            SoundAsset::SpookyDeath00,
            SoundAsset::SpookyDeath01
        ],
        hurt_sounds: &[
            SoundAsset::MonsterHurt02,
        ],
    },
    CreatureTypeDef {
        name: "Evil Knight",
        scene: SceneAsset::EvilGuardKnight,
        
        weapon: WeaponType::HeavyMonsterClaw,
        aggressive: true,

        base_vitality: 250,
        base_endurance: 250,
        base_strength: 175,
        base_dexterity: 10,
        base_luck: 10,
    
        base_level: 30,
        base_experience: 1.0,
        
        move_cd: 1.0,
        death_sounds: &[
            SoundAsset::MaleDeath01,
            SoundAsset::MaleDeath03,
            SoundAsset::MaleDeath04,
            SoundAsset::MaleDeath06,
            SoundAsset::MaleDeath07,
            SoundAsset::MaleDeath08,
            SoundAsset::MaleDeath09,
            SoundAsset::MaleDeath10,
            SoundAsset::MaleDeath11,
            SoundAsset::MaleDeath12,
            SoundAsset::MaleDeath13,
        ],
        hurt_sounds: &[
            SoundAsset::MaleHurt00,
            SoundAsset::MaleHurt01,
            SoundAsset::MaleHurt02,
            SoundAsset::MaleHurt03,
            SoundAsset::MaleHurt04,
        ],
    },
];