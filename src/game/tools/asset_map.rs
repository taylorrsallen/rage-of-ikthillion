use crate::*;
use bevy::{
    sprite::*,
    render::texture::*,
    utils::HashMap,
};
use std::env::*;
use std::fs::*;
use std::io::*;

//================================-================================-================================
const FONTS: &'static [&str] = &[
    "Hack/Hack-Regular",
    "Hack/Hack-Italic",
    "Hack/Hack-Bold",
    "Hack/Hack-BoldItalic",
];
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FontAsset {
    HackRegular,
    HackItalic,
    HackBold,
    HackBoldItalic,
}

const IMAGES: &'static [&str] = &[
    "inventory_tile_01",
    "inventory_tile_02",
    "inventory_tile_03",
    "inventory_tile_04",
    "inventory_tile_05",
    "inventory_tile_06",

    "ramen",
    "hammer",
    "sword",
    "axe",
    "pickaxe",
    "katana",
    "holy_handgun_of_antinuk",

    "vitality",
    "endurance",
    "strength",
    "dexterity",
    "luck",
    "backpack",

    "ikthillion_marker",
];
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ImageAsset {
    InventoryTile01,
    InventoryTile02,
    InventoryTile03,
    InventoryTile04,
    InventoryTile05,
    InventoryTile06,

    Ramen,
    Hammer,
    Sword,
    Axe,
    Pickaxe,
    Katana,
    HolyHandgunOfAntinuk,

    Vitality,
    Endurance,
    Strength,
    Dexterity,
    Luck,
    Backpack,

    IkthillionMarker,
}

const SCENES: &'static [&str] = &[
    "hammer",
    "sword",
    "axe",
    "pickaxe",
    "katana",
    "holy_handgun_of_antinuk",
    
    "evil_chicken",
    "evil_lizard",
    "evil_angel",
    "evil_pidgeon",
    "evil_cat",
    "evil_guard_knight",
    "evil_wizard",
    "evil_peasant",
    "evil_ninja",
    "evil_fish",
    "evil_anime",
    "evil_skeleton",
    "evil_whelp",
];
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SceneAsset {
    Hammer,
    Sword,
    Axe,
    Pickaxe,
    Katana,
    HolyHandgunOfAntinuk,

    EvilChicken,
    EvilLizard,
    EvilAngel,
    EvilPidgeon,
    EvilCat,
    EvilGuardKnight,
    EvilWizard,
    EvilPeasant,
    EvilNinja,
    EvilFish,
    EvilAnime,
    EvilSkeleton,
    EvilWhelp,

    Null,
}

const SOUNDS: &'static [&str] = &[
    "step_boot_stone_00",
    "step_boot_stone_01",
    "step_boot_dirt_00",
    "step_boot_dirt_01",
    "step_boot_dirt_02",
    "step_boot_grass",
    "gooey_step_00",
    "gooey_step_01",
    "cat_death_00",
    "cat_death_01",
    "cat_death_02",
    "cat_death_03",
    "cat_hurt_00",
    "cat_hurt_01",
    "goblin_death_00",
    "female_death_00",
    "female_death_01",
    "female_death_02",
    "female_death_03",
    "female_death_04",
    "female_death_05",
    "male_death_00",
    "male_death_01",
    "male_death_02",
    "male_death_03",
    "male_death_04",
    "male_death_05",
    "male_death_06",
    "male_death_07",
    "male_death_08",
    "male_death_09",
    "male_death_10",
    "male_death_11",
    "male_death_12",
    "male_death_13",
    "male_hurt_00",
    "male_hurt_01",
    "male_hurt_02",
    "male_hurt_03",
    "male_hurt_04",
    "monster_death_00",
    "monster_death_01",
    "monster_death_02",
    "monster_hurt_00",
    "monster_hurt_01",
    "monster_hurt_02",
    "spooky_death_00",
    "spooky_death_01",
    "chicken_death_00",
    "chicken_death_01",
    "chicken_death_02",
    "chicken_hurt_00",
    "chicken_hurt_01",
    "pidgeon_death_00",
    "japanese_hurt_00",
    "japanese_hurt_01",
    "japanese_hurt_02",
    "japanese_hurt_03",
    "ikthillion_death_00",
    "ikthillion_attack_00",
    "whelp_death_00",
    "pop_00",
    "pop_01",
    "pop_02",
    "hit_00",
    "hit_01",
    "hit_critical_00",
    "hit_sword_metal_00",
    "swing_00",
    "swing_01",
    "swing_02",
    "fast_swing_00",
    "tick_00",
    "tick_01",
    "cloth_00",
    "cloth_01",
    "cloth_02",
    "stone_break_00",
    "stone_break_01",
    "cave_ambience_00",
    "level_up_00",
    "level_up_01",
    "katana_00",
    "holy_handgun_of_antinuk_00",
    "holy_handgun_of_antinuk_01",
    "holy_handgun_of_antinuk_02",
    "holy_handgun_of_antinuk_03",
    "death",
    "saving_all_ikthia",
];
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SoundAsset {
    StepBootStone00,
    StepBootStone01,
    StepBootDirt00,
    StepBootDirt01,
    StepBootDirt02,
    StepBootGrass,
    GooeyStep00,
    GooeyStep01,
    CatDeath00,
    CatDeath01,
    CatDeath02,
    CatDeath03,
    CatHurt00,
    CatHurt01,
    GoblinDeath00,
    FemaleDeath00,
    FemaleDeath01,
    FemaleDeath02,
    FemaleDeath03,
    FemaleDeath04,
    FemaleDeath05,
    MaleDeath00,
    MaleDeath01,
    MaleDeath02,
    MaleDeath03,
    MaleDeath04,
    MaleDeath05,
    MaleDeath06,
    MaleDeath07,
    MaleDeath08,
    MaleDeath09,
    MaleDeath10,
    MaleDeath11,
    MaleDeath12,
    MaleDeath13,
    MaleHurt00,
    MaleHurt01,
    MaleHurt02,
    MaleHurt03,
    MaleHurt04,
    MonsterDeath00,
    MonsterDeath01,
    MonsterDeath02,
    MonsterHurt00,
    MonsterHurt01,
    MonsterHurt02,
    SpookyDeath00,
    SpookyDeath01,
    ChickenDeath00,
    ChickenDeath01,
    ChickenDeath02,
    ChickenHurt00,
    ChickenHurt01,
    PidgeonDeath00,
    JapaneseHurt00,
    JapaneseHurt01,
    JapaneseHurt02,
    JapaneseHurt03,
    IkthillionDeath00,
    IkthillionAttack00,
    WhelpDeath00,
    Pop00,
    Pop01,
    Pop02,
    Hit00,
    Hit01,
    HitCritical00,
    HitSwordMetal00,
    Swing00,
    Swing01,
    Swing02,
    FastSwing00,
    Tick00,
    Tick01,
    Cloth00,
    Cloth01,
    Cloth02,
    StoneBreak00,
    StoneBreak01,
    CaveAmbience00,
    LevelUp00,
    LevelUp01,
    Katana00,
    HolyHandgunOfAntinuk00,
    HolyHandgunOfAntinuk01,
    HolyHandgunOfAntinuk02,
    HolyHandgunOfAntinuk03,
    Death,
    SavingAllIkthia,
    Null,
}

pub const STONE_BREAKS: [SoundAsset; 2] = [
    SoundAsset::StoneBreak00,
    SoundAsset::StoneBreak01,
];

pub const POPS: [SoundAsset; 3] = [
    SoundAsset::Pop00,
    SoundAsset::Pop01,
    SoundAsset::Pop02,
];

pub const ATTACK_SWINGS: [SoundAsset; 3] = [
    SoundAsset::Swing00,
    SoundAsset::Swing01,
    SoundAsset::Swing02,
];

//================================-================================-================================
pub struct AssetMapPlugin;
impl Plugin for AssetMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_systems((
                startup_asset_map.in_base_set(StartupSet::PreStartup),
            ))
            .add_system(startup_assets);
    }
}

//================================-================================-================================
#[derive(Default, Resource)]
pub struct AssetMap {
    pub texture_atlas: Handle<Image>,
    pub fonts: Vec<Handle<Font>>,
    pub images: Vec<Handle<Image>>,
    // pub generator_templates: Vec<GeneratorTemplate>,
    pub scenes: Vec<Handle<Scene>>,
    pub sounds: Vec<Handle<AudioSource>>,
}

impl AssetMap {
    fn load_font(
        &mut self,
        name: &str,
        asset_server: &Res<AssetServer>,
    ) {
        println!("asset_map: loading font: {}", "fonts/".to_string() + name + ".ttf");
        self.fonts.push(asset_server.load("fonts/".to_string() + name + ".ttf"));
    }

    pub fn get_font(
        &self,
        font: FontAsset,
    ) -> Handle<Font> {
        self.fonts[font as usize].clone()
    }



    fn load_image(
        &mut self,
        name: &str,
        asset_server: &Res<AssetServer>,
    ) {
        println!("asset_map: loading image: {}", "images/".to_string() + name + ".png");
        self.images.push(asset_server.load("images/".to_string() + name + ".png"));
    }

    pub fn get_image(
        &self,
        image: ImageAsset,
    ) -> Handle<Image> {
        self.images[image as usize].clone()
    }



    fn load_scene(
        &mut self,
        name: &str,
        asset_server: &Res<AssetServer>,
    ) {
        println!("asset_map: loading scene: {}", "models/".to_string() + name + ".glb#Scene0");
        self.scenes.push(asset_server.load("models/".to_string() + name + ".glb#Scene0"));
    }

    pub fn get_scene_handle(
        &self,
        scene: SceneAsset,
    ) -> Handle<Scene> {
        self.scenes[scene as usize].clone()
    }



    fn load_sound(
        &mut self,
        name: &str,
        asset_server: &Res<AssetServer>,
    ) {
        println!("asset_map: loading sound: {}", "sounds/".to_string() + name + ".ogg");
        self.sounds.push(asset_server.load("sounds/".to_string() + name + ".ogg"));
    }

    pub fn get_sound(
        &self,
        sound: SoundAsset,
    ) -> Handle<AudioSource> {
        self.sounds[sound as usize].clone()
    }
}

//================================-================================-================================
fn startup_asset_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut asset_map = AssetMap::default();
    asset_map.texture_atlas = asset_server.load("texture_atlas.png");

    for font_name in FONTS {
        asset_map.load_font(font_name, &asset_server);
    }
    for image_name in IMAGES {
        asset_map.load_image(image_name, &asset_server);
    }
    for scene_name in SCENES {
        asset_map.load_scene(scene_name, &asset_server);
    }
    for sound_name in SOUNDS {
        asset_map.load_sound(sound_name, &asset_server);
    }

    commands.insert_resource(asset_map);
}

fn startup_assets(
    mut images: ResMut<Assets<Image>>,
) {
    for (_, mut image) in images.iter_mut() {
        image.sampler_descriptor = ImageSampler::Descriptor(
            SamplerDescriptor {
                address_mode_u: AddressMode::Repeat,
                address_mode_v: AddressMode::Repeat,
                address_mode_w: AddressMode::Repeat,
                mag_filter: FilterMode::Nearest,
                min_filter: FilterMode::Nearest,
                ..default()
            }
        )
    }
}