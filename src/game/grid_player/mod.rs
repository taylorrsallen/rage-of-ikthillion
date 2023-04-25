use crate::*;
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        clear_color::ClearColorConfig,
    },
};

mod fps_camera;
pub use fps_camera::*;

//================================-================================-================================
pub struct GridPlayerPlugin;
impl Plugin for GridPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FpsCameraPlugin)
            .add_systems((
                player_input,
                update_player_weapon,
                // move_debug_player,
                update_player_inventory,
            ).in_set(OnUpdate(AppState::Gameplay)));
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct Player {
    camera: Entity,
}

impl Player {
    pub fn new(
        camera: Entity,
    ) -> Self {
        Self {
            camera,
        }
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct PlayerWeapon(WeaponType);

//================================-================================-================================
pub fn spawn_player(
    coord: &IVec3,
    facing: &IVec3,
    inventory: &Inventory,
    commands: &mut Commands,
    root: &mut GridRoot,
    asset_map: &Res<AssetMap>,
) {

    let camera = commands.spawn((
            Camera3dBundle {
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: 60.0 * 0.01745329,
                    ..default()
                }),
                transform: Transform::from_translation(coord.as_vec3()).looking_at(Vec3::Z, Vec3::Y),
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                    ..default()
                },
                ..default()
            }, 
            BloomSettings::default(),
        ))
        .insert(PointLightBundle {
            point_light: PointLight {
                color: Color::rgb(0.95, 0.6, 0.1),
                intensity: 80.0,
                ..default()
            },
            ..default()
        })
        .insert(FpsCameraBundle::new(FpsCameraController::default(), coord.as_vec3(), facing.as_vec3(), Vec3::Y))
        .insert(Name::new("Camera"))
        .with_children(|child_builder| {
            child_builder.spawn(SceneBundle {
                    scene: asset_map.get_scene_handle(SceneAsset::Katana),
                    transform: Transform::from_translation(Vec3::new(0.3, -0.5, -0.5)),
                    ..default()
                })
                .insert(MoveTo::new(&Vec3::NEG_Y, &Vec3::new(0.3, -0.5, -0.5), 5.0).with_look_axis(&Vec3::Z))
                .insert(PathAnim::new(vec![]))
                .insert(PlayerWeapon(WeaponType::Katana));
        })
        .id();

    let mut player_inventory = inventory.clone();
    if inventory.dim == IVec2::ONE {
        player_inventory = Inventory::new(&IVec2::new(8, 6));
        let hammer = InventoryItem::new(&IVec2::new(0, 0), &IVec2::new(1, 2), InventoryItemType::Hammer);
        player_inventory.add_inventory_item(&hammer);

        // let handgun = InventoryItem::new(&IVec2::new(0, 2), &IVec2::new(1, 2), InventoryItemType::HolyHandgunOfAntinuk);
        // inventory.add_inventory_item(&handgun);
        // let ramen = InventoryItem::new(&IVec2::new(4, 0), &IVec2::new(2, 2), InventoryItemType::Ramen);
        //             inventory.add_inventory_item(&ramen);
    }

    let player_entity = commands.spawn(Player::new(camera))
        .insert(GridEntity::new(WeaponType::Hammer, CreatureType::EvilNinja, *coord, *facing, 0.2))
        .insert(Stats {
            health: f32::MAX,
            stamina: f32::MAX,
            vitality: 1,
            endurance: 1,
            strength: 1,
            dexterity: 1,
            luck: 1,
            level: 1,
            experience: 0.0,
            bullets: 10,
            reload_timer: Timer::from_seconds(2.0, TimerMode::Once),
            reloading: false,
        })
        .insert(player_inventory.clone())
        .insert(InventoryViewer::default())
        .insert(InputManagerBundle {
            input_map: get_default_input_map(),
            ..default()
        })
        .insert(MoveTo::new(&facing.as_vec3(), &coord.as_vec3(), 5.0))
        .insert(PathAnim::new(vec![]))
        .insert(TransformBundle {
            local: Transform::from_translation(coord.as_vec3()),
            ..default()
        })
        .insert(Name::new("Player"))
        .id();

    root.set_creature_at_coord(coord, &Some(player_entity));
}



fn player_input(
    mut player_query: Query<(Entity, &Player, &ActionState<InputAction>, &mut Transform, &mut GridEntity), Without<LookTransform>>,
    mut camera_query: Query<(&mut LookTransform, &Transform)>,
    mut grid_root_query: Query<&mut GridRoot>,
    mouse_buttons: Res<Input<MouseButton>>,
    menu_states: Res<MenuStates>,
    asset_map: Res<AssetMap>,
    time: Res<Time>,
) {
    if let Ok((player_entity, player, action_state, mut player_transform, mut player_grid_entity)) = player_query.get_single_mut() {
        let (mut look_transform, camera_transform) = camera_query.single_mut();
        let mut grid_root = grid_root_query.single_mut();
    
        let mut forward = camera_transform.forward();
        forward.y = 0.0;
        forward = forward.normalize();
    
        let mut right = camera_transform.right();
        right.y = 0.0;
        right = right.normalize();
    
        let look_target_offset = look_transform.target - look_transform.eye;
    
        let mut action_input = false;
        if !menu_states.menu_open {
            if mouse_buttons.pressed(MouseButton::Left) {
                action_input = true;
                player_grid_entity.set_next_action_with_direction(&forward);
            }
        }
    
        if !action_input && !player_grid_entity.actioning {
            player_grid_entity.set_next_action_with_grid_step(&IVec3::ZERO);
        }
    
        let mut movement_input = false;
        let mut combined_move = Vec3::ZERO;
        for (input, direction) in [
            (InputAction::MoveLeft, -right),
            (InputAction::MoveRight, right),
            (InputAction::MoveBack, -forward),
            (InputAction::MoveForward, forward),
            (InputAction::Crouch, Vec3::NEG_Y),
            (InputAction::Jump, Vec3::Y),
        ].iter().cloned() {
            if action_state.pressed(input) {
                movement_input = true;
                player_grid_entity.set_next_move_with_direction(&direction);
                break;
            }
        }
    
        if !movement_input {
            player_grid_entity.set_next_move_with_grid_step(&IVec3::ZERO);
        }
        
        look_transform.eye = player_transform.translation;
        look_transform.target = look_transform.eye + look_target_offset;
    }
}

fn update_player_weapon(
    mut commands: Commands,
    mut camera_query: Query<Entity, With<FpsCameraController>>,
    mut player_weapon_query: Query<(Entity, &mut Transform, &mut PathAnim, &PlayerWeapon)>,
    mut player_query: Query<(&mut GridEntity, &mut Stats), With<Player>>,
    mut bgm_events: EventWriter<BGMEvent>,
    asset_map: Res<AssetMap>,
) {
    let camera = camera_query.single();
    let (weapon_entity, mut weapon_transform, mut weapon_path_anim, weapon) = player_weapon_query.single_mut();
    if let Ok((mut player_grid_entity, mut player_stats)) = player_query.get_single_mut() {
        let weapon_def = player_grid_entity.weapon.get_def();
        
        if player_grid_entity.weapon != weapon.0 {
            if player_grid_entity.weapon == WeaponType::HolyHandgunOfAntinuk {
                bgm_events.send(BGMEvent::new(SoundAsset::HolyHandgunOfAntinuk00, 0.5, BGMType::HolyHandgunOfAntinuk));
            } else {
                bgm_events.send(BGMEvent::new(SoundAsset::Null, 0.5, BGMType::HolyHandgunOfAntinuk));
            }
    
            commands.entity(weapon_entity).despawn_recursive();
            commands.entity(camera).with_children(|child_builder| {
                child_builder.spawn(SceneBundle {
                        scene: asset_map.get_scene_handle(weapon_def.scene),
                        transform: Transform::from_translation(weapon_def.equip_offset),
                        ..default()
                    })
                    .insert(MoveTo::new(&weapon_def.equip_rotation.0, &weapon_def.equip_offset, 5.0).with_look_axis(&weapon_def.equip_rotation.1))
                    .insert(PathAnim::new(vec![]))
                    .insert(PlayerWeapon(player_grid_entity.weapon));
            });
        }
    
        if player_grid_entity.actioning && !player_grid_entity.action_anim_started && !weapon_path_anim.is_active() {
            player_grid_entity.action_anim_started = true;
            weapon_path_anim.set_path(vec![
                (weapon_transform.translation + weapon_def.attack_offset, weapon_def.attack_anim_speed),
                (weapon_transform.translation, 5.0),
            ]);
        }
    }
}

fn update_player_inventory(
    mut commands: Commands,
    mut root_query: Query<&mut GridRoot>,
    mut player_query: Query<(&mut Inventory, &GridEntity), With<Player>>,
    mut ui_sound_events: EventWriter<UISoundEvent>,
    mut settings: ResMut<Settings>,
    grid_item_query: Query<&GridItem>,
) {
    let mut removals: Vec<IVec3> = vec![];
    let mut root = root_query.single_mut();
    if let Ok((mut inventory, grid_entity)) = player_query.get_single_mut() {
        if let Some(item_entity) = root.get_item_from_coord(&grid_entity.coord) {
            if let Ok(grid_item) = grid_item_query.get(*item_entity) {
                // let ramen = InventoryItem::new(&IVec2::new(4, 0), &IVec2::new(2, 2), InventoryItemType::Ramen);
                // let hammer = InventoryItem::new(&IVec2::new(0, 0), &IVec2::new(1, 2), InventoryItemType::Hammer);
                // inventory.add_inventory_item(&ramen);
                // inventory.add_inventory_item(&hammer);
                let mut inventory_item: InventoryItem;
                match grid_item.inventory_item_type {
                    InventoryItemType::Katana => {
                        inventory_item = InventoryItem::new(&IVec2::new(0, 5), &IVec2::new(6, 1), InventoryItemType::Katana);
                    },
                    InventoryItemType::Sword => {
                        inventory_item = InventoryItem::new(&IVec2::new(0, 4), &IVec2::new(5, 1), InventoryItemType::Sword);
                    },
                    InventoryItemType::Axe => {
                        inventory_item = InventoryItem::new(&IVec2::new(6, 0), &IVec2::new(2, 6), InventoryItemType::Axe);
                    },
                    InventoryItemType::Pickaxe => {
                        inventory_item = InventoryItem::new(&IVec2::new(1, 0), &IVec2::new(2, 3), InventoryItemType::Pickaxe);
                    },
                    InventoryItemType::HolyHandgunOfAntinuk => {
                        inventory_item = InventoryItem::new(&IVec2::new(0, 2), &IVec2::new(1, 2), InventoryItemType::HolyHandgunOfAntinuk);
                    },
                    _ => {
                        inventory_item = InventoryItem::new(&IVec2::new(0, 0), &IVec2::new(1, 2), InventoryItemType::Hammer);
                    },
                }
    
                inventory.add_inventory_item(&inventory_item);
                ui_sound_events.send(UISoundEvent::new(inventory_item.get_def().pickup_sound, 1.0));
                settings.inventory = inventory.clone();
            }
    
            removals.push(grid_entity.coord);
            commands.entity(*item_entity).despawn_recursive();
        }
    }
    
    for removal in removals.iter() {
        root.set_item_at_coord(&removal, &None);
    }
}

fn move_debug_player(
    mut camera_query: Query<(&mut LookTransform, &Transform), With<FpsCameraController>>,
    mut player_query: Query<&mut GridEntity, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    fixed_time: Res<FixedTime>,
) {
    let mut player_grid_entity = player_query.single_mut();
    let (mut look_transform, transform) = camera_query.single_mut();
    let mut movement_input = Vec3::ZERO;

    let mut forward = transform.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut right = transform.right();
    right.y = 0.0;
    right = right.normalize();

    // let mut up = transform.up();
    // up = up.normalize();

    for (key, direction) in [
        (KeyCode::A, -right),
        (KeyCode::D, right),
        (KeyCode::S, -forward),
        (KeyCode::W, forward),
        (KeyCode::C, Vec3::NEG_Y),
        (KeyCode::Space, Vec3::Y),
    ].iter().cloned() {
        if keyboard.pressed(key) {
            movement_input += direction;
        }
    }

    let look_target_offset = look_transform.target - look_transform.eye;
    if let Some(movement) = movement_input.try_normalize() {
        let speed = 25.0;
        let delta = fixed_time.period.as_secs_f32();

        look_transform.eye += movement * speed * delta;
        look_transform.target = look_transform.eye + look_target_offset;
        player_grid_entity.coord = look_transform.eye.as_ivec3();
    }
}