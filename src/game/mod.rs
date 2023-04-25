use crate::*;
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        clear_color::ClearColorConfig,
    },
};

pub use rand::*;
pub use leafwing_input_manager::prelude::*;

mod defs;
mod dungeon;
mod grid;
mod grid_entity;
mod grid_player;
mod tools;
mod ui;
pub use defs::*;
pub use dungeon::*;
pub use grid::*;
pub use grid_entity::*;
pub use grid_player::*;
pub use tools::*;
pub use ui::*;

//================================-================================-================================
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugin(ToolsPlugin)
            .add_plugin(GridEntityPlugin)
            .add_plugin(GridPlayerPlugin)
            .add_plugin(GridPlugin)
            .add_plugin(UIPlugin)
            .add_system(on_enter_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(on_exit_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(on_enter_gameplay.in_schedule(OnEnter(AppState::Gameplay)))
            .add_system(on_exit_gameplay.in_schedule(OnExit(AppState::Gameplay)));
    }
}

//================================-================================-================================
pub fn on_enter_main_menu(
    mut commands: Commands,
    mut settings: ResMut<Settings>,
    mut bgm_events: EventWriter<BGMEvent>,
    mut death_root_query: Query<Entity, With<DeathMenuRoot>>,
    asset_map: Res<AssetMap>,
) {
    spawn_main_menu(&mut commands, &asset_map);
    if settings.player_died {
        if let Ok(death_root) = death_root_query.get_single() {
            commands.entity(death_root).despawn_recursive();
        }
        settings.player_died = false;
        bgm_events.send(BGMEvent::new(SoundAsset::Null, 1.0, BGMType::Music));
    }
}

pub fn on_exit_main_menu(
    mut commands: Commands,
    mut main_menu_query: Query<Entity, With<MainMenuRoot>>,
    mut main_menu_camera_query: Query<Entity, With<MainMenuCamera>>,
) {
    commands.entity(main_menu_query.single_mut()).despawn_recursive();
    commands.entity(main_menu_camera_query.single_mut()).despawn_recursive();
}

pub fn on_enter_gameplay(
    mut commands: Commands,
    mut grid_root_query: Query<&mut GridRoot>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut tasks: ResMut<Tasks>,
    mut bgm_events: EventWriter<BGMEvent>,
    asset_map: Res<AssetMap>,
    mut settings: ResMut<Settings>,
    mut victory_events: EventWriter<VictoryEvent>,
) {
    if let Ok(mut root) = grid_root_query.get_single_mut() {
        spawn_player(&settings.player_spawn, &IVec3::Z, &settings.inventory, &mut commands, &mut root, &asset_map);
    } else {
        let mut root = GridRoot::new(GridTile::new(TileType::Block, TileMatter::Stone));
        // generate_debug_room(&mut tasks, &mut root);
        let player_spawn_point = generate_dungeon(&mut commands, &mut root, &mut tasks, &asset_map);
        settings.player_spawn = player_spawn_point;
        spawn_player(&player_spawn_point, &IVec3::Z, &settings.inventory, &mut commands, &mut root, &asset_map);
        commands.spawn(root);
    }

    bgm_events.send(BGMEvent::new(SoundAsset::CaveAmbience00, 0.5, BGMType::Ambience));
}

pub fn on_exit_gameplay(
    mut commands: Commands,
    mut grid_root_query: Query<(Entity, &mut GridRoot)>,
    mut player_query: Query<(Entity, &GridEntity), With<Player>>,
    mut camera_query: Query<Entity, With<Camera>>,
    mut window_query: Query<&mut Window>,
    asset_map: Res<AssetMap>,
) {
    let (grid_root_entity, mut grid_root) = grid_root_query.single_mut();
    let (player_entity, player_grid_entity) = player_query.single();
    let mut window = window_query.single_mut();
    window.cursor.visible = true;
    window.cursor.grab_mode = CursorGrabMode::None;

    grid_root.set_creature_at_coord(&player_grid_entity.coord, &None);

    commands.entity(player_entity).despawn_recursive();
    commands.entity(camera_query.single_mut()).despawn_recursive();
}

pub fn on_enter_death(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
) {
    next_state.set(AppState::Gameplay);

    commands.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::Z, Vec3::Y),
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::DARK_GRAY),
                    ..default()
                },
                ..default()
            }, 
            BloomSettings::default(),
        ))
        .insert(MainMenuCamera)
        .insert(Name::new("Main Menu Camera"));
}

//================================-================================-================================
fn generate_debug_room(
    tasks: &mut ResMut<Tasks>,
    grid_root: &mut GridRoot,
) {
    let room_0_size = 3;
    let room_1_size = 2;
    let pit_depth = 3;

    let open_tile = GridTile::open();
    let dirt_tile = GridTile::new(TileType::Block, TileMatter::Grass);
    let stone_tile = GridTile::new(TileType::Block, TileMatter::Stone);

    // room 0
    for x in -room_0_size..=room_0_size { for z in -room_0_size..=room_0_size {
        grid_root.set_tile_on_at_coord(&IVec3::new(x, 0, z), &open_tile);
        grid_root.set_tile_on_at_coord(&IVec3::new(x, 1, z), &open_tile);
        grid_root.set_tile_on_at_coord(&IVec3::new(x, 2, z), &open_tile);
    }}
    for y in 0..3 {
        grid_root.set_tile_on_at_coord(&IVec3::new(-room_0_size + 1, y, -room_0_size + 1), &stone_tile);
        grid_root.set_tile_on_at_coord(&IVec3::new( room_0_size - 1, y, -room_0_size + 1), &stone_tile);
        grid_root.set_tile_on_at_coord(&IVec3::new(-room_0_size + 1, y,  room_0_size - 1), &stone_tile);
        grid_root.set_tile_on_at_coord(&IVec3::new( room_0_size - 1, y,  room_0_size - 1), &stone_tile);
    }
    
    // dirt cube
    let dirt_cube_size = room_1_size + 2;
    for x in -dirt_cube_size..=dirt_cube_size { for y in -dirt_cube_size..=dirt_cube_size { for z in -dirt_cube_size..=dirt_cube_size {
        grid_root.set_tile_on_at_coord(&IVec3::new(x, y, z + room_0_size + room_1_size + 2), &dirt_tile);
    }}}

    // hall: room 0 to room 1
    grid_root.set_tile_on_at_coord(&IVec3::new(0, 0, room_0_size), &open_tile);
    grid_root.set_tile_on_at_coord(&IVec3::new(0, 0, room_0_size + 1), &open_tile);
    
    // room 1
    for x in -room_1_size..=room_1_size { for z in -room_1_size..=room_1_size {
        grid_root.set_tile_on_at_coord(&IVec3::new(x, 0, z + room_0_size + room_1_size + 2), &open_tile);
    }}
    
    // tasks.spawn_creature(&IVec3::new(0, 0, room_0_size), &IVec3::NEG_Z, CreatureType::Skreleton);
    // tasks.spawn_creature(&IVec3::new(0, 0, room_0_size + 1), &IVec3::NEG_Z, CreatureType::Skreleton);
    // tasks.spawn_creature(&IVec3::new(0, 0, room_0_size + 2), &IVec3::NEG_Z, CreatureType::Skreleton);
    // tasks.spawn_creature(&IVec3::new(0, 0, room_0_size + 3), &IVec3::NEG_Z, CreatureType::Skreleton);
    // tasks.spawn_creature(&IVec3::new(0, 0, room_0_size + 4), &IVec3::NEG_Z, CreatureType::Skreleton);
}