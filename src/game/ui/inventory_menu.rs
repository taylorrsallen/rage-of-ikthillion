use std::{rc::Rc, sync::{Arc, RwLock}};

use bevy::ui::RelativeCursorPosition;

use crate::*;

//================================-================================-================================
pub struct InventoryMenuPlugin;
impl Plugin for InventoryMenuPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems((
                    toggle_inventory_menu,
                    update_inventory_menu,
                    update_cursor_follow,
                ).chain().in_set(OnUpdate(AppState::Gameplay)))
            .add_systems((
                    despawn_inventory_menu,
                ).in_schedule(OnExit(AppState::Gameplay)));
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct InventoryMenuRoot;

#[derive(Component)]
pub struct InventoryPanel {
    pub inventory_entity: Entity,
    pub slot_images: Vec<Entity>,
    pub item_images: Vec<(IVec2, Entity)>,
    pub dirty_mask: Bitmask,
}

impl InventoryPanel {
    pub fn new(
        inventory_entity: &Entity,
        dim2d: &IVec2,
    ) -> Self {
        Self {
            inventory_entity: *inventory_entity,
            slot_images: vec![],
            item_images: vec![],
            dirty_mask: Bitmask::from_dim2d(dim2d, false),
        }
    }
}

#[derive(Component)]
pub struct InventorySlotUI;

#[derive(Component)]
pub struct InventoryItemUI;

//================================-================================-================================
#[derive(Component)]
pub struct CursorFollow {
    offset: Vec2,
}

fn update_cursor_follow(
    mut commands: Commands,
    mut cursor_follow_query: Query<(Entity, &mut Style, &mut Visibility, &CursorFollow)>,
    window_query: Query<&Window>,
    action_state_query: Query<&ActionState<InputAction>>,
) {
    if let Ok(action_state) = action_state_query.get_single() {
        let window = window_query.single();
        if let Some(cursor_position) = window.cursor_position() {
            for (entity, mut style, mut visibility, cursor_follow) in cursor_follow_query.iter_mut() {
                if action_state.released(InputAction::PrimaryAction) {
                    commands.entity(entity).despawn_recursive();
                } else {
    
                    style.position = UiRect {
                        left: Val::Px(cursor_position.x + cursor_follow.offset.x),
                        bottom: Val::Px(cursor_position.y + cursor_follow.offset.y),
                        ..default()
                    };
                    *visibility = Visibility::Visible;
                }
            }
        }
    }
}

//================================-================================-================================
pub fn toggle_inventory_menu(
    mut commands: Commands,
    mut menu_states: ResMut<MenuStates>,
    mut ui_sound_events: EventWriter<UISoundEvent>,
    player_inventory_query: Query<(Entity, &Inventory), With<Player>>,
    action_state_query: Query<&ActionState<InputAction>>,
    asset_map: Res<AssetMap>,
) {
    if let Ok(action_state) = action_state_query.get_single() {
        if action_state.just_pressed(InputAction::MenuInventory) && menu_states.escape == None {
            if let Some(inventory_menu) = menu_states.inventory {
                ui_sound_events.send(UISoundEvent::new(SoundAsset::Cloth01, 1.0));
                commands.entity(inventory_menu).despawn_recursive();
                menu_states.inventory = None;
            } else {
                ui_sound_events.send(UISoundEvent::new(SoundAsset::Cloth00, 1.0));
                
                let (player_entity, player_inventory) = player_inventory_query.single();
                menu_states.inventory = Some(spawn_inventory_menu(&player_entity, &player_inventory, &mut commands, &asset_map));
            }
        }
    }
}

pub fn despawn_inventory_menu(
    mut commands: Commands,
    mut menu_states: ResMut<MenuStates>,
    inventory_menu_query: Query<Entity, With<InventoryMenuRoot>>,
) {
    if let Ok(inventory_menu_entity) = inventory_menu_query.get_single() {
        commands.entity(inventory_menu_entity).despawn_recursive();
        menu_states.inventory = None;
    }
}

pub fn update_inventory_menu(
    mut commands: Commands,
    mut inventory_slot_query: Query<&mut UiImage, With<InventorySlotUI>>,
    mut inventory_item_query: Query<(Entity, &GlobalTransform), With<InventoryItemUI>>,
    mut inventory_panel_query: Query<(&mut InventoryPanel, &RelativeCursorPosition)>,
    mut ui_sound_events: EventWriter<UISoundEvent>,
    mut player_query: Query<(&mut GridEntity, &ActionState<InputAction>), With<Player>>,
    mouse_buttons: Res<Input<MouseButton>>,
    inventory_query: Query<&Inventory>,
    window_query: Query<&Window>,
    asset_map: Res<AssetMap>,
) {
    if let Ok((mut player_grid_entity, action_state)) = player_query.get_single_mut() {
        let window = window_query.single();
    
        for (mut inventory_panel, relative_cursor_position) in inventory_panel_query.iter_mut() {
            let inventory = inventory_query.get(inventory_panel.inventory_entity).ok().unwrap();
    
            for dirty_index in OnMaskIter::new(0, &inventory_panel.dirty_mask) {
                let mut inventory_slot_ui = inventory_slot_query.get_mut(inventory_panel.slot_images[dirty_index]).ok().unwrap();
                if inventory.active_mask.is_bit_on(dirty_index) {
                    inventory_slot_ui.texture = asset_map.get_image(ImageAsset::InventoryTile02);
                } else {
                    inventory_slot_ui.texture = asset_map.get_image(ImageAsset::InventoryTile01);
                }
            }
    
            inventory_panel.dirty_mask.set_off();
    
            if let Some(cursor_position) = relative_cursor_position.normalized {
                if cursor_position.x >= 0.0 && cursor_position.x <= 1.0 && cursor_position.y >= 0.0 && cursor_position.y <= 1.0 {
                    let index = inventory.index_from_coord(&(Vec2::new(cursor_position.x, 1.0 - cursor_position.y) * inventory.dim.as_vec2()).as_ivec2());
    
                    if inventory.active_mask.is_bit_on(index) {
                        let item = inventory.items[index];
                        let item_def = item.get_def();
    
                        if mouse_buttons.just_released(MouseButton::Left) {
                            if item_def.weapon_type != WeaponType::Null {
                                player_grid_entity.weapon = item_def.weapon_type;
                                ui_sound_events.send(UISoundEvent::new(item_def.equip_sound, 1.0));
                            }
    
                            println!("{}", item_def.name);
                        }
                        
                        for y in 0..item.dim.y { for x in 0..item.dim.x {
                            let item_index = inventory.index_from_coord(&(item.origin + IVec2::new(x, y)));
                            let mut inventory_slot_ui = inventory_slot_query.get_mut(inventory_panel.slot_images[item_index]).ok().unwrap();
                            inventory_slot_ui.texture = asset_map.get_image(ImageAsset::InventoryTile05);
                            inventory_panel.dirty_mask.set_bit_on(item_index);
                        }}
                    } else {
                        let mut inventory_slot_ui = inventory_slot_query.get_mut(inventory_panel.slot_images[index]).ok().unwrap();
                        inventory_slot_ui.texture = asset_map.get_image(ImageAsset::InventoryTile04);
                        inventory_panel.dirty_mask.set_bit_on(index);
                    }
                }
            }
        }
    }
}

//================================-================================-================================
pub fn spawn_inventory_menu(
    inventory_entity: &Entity,
    inventory: &Inventory,
    commands: &mut Commands,
    asset_map: &Res<AssetMap>,
) -> Entity {
    commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .insert(InventoryMenuRoot)
        .insert(Name::new("Inventory Root"))
        .with_children(|child_builder| {
            spawn_inventory_panel(inventory_entity, inventory, child_builder, &asset_map);
        })
        .id()
}

pub fn spawn_inventory_panel(
    inventory_entity: &Entity,
    inventory: &Inventory,
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
) -> Entity {
    let mut inventory_panel = InventoryPanel::new(inventory_entity, &inventory.dim);

    child_builder.spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                size: Size::new(Val::Px(inventory.dim.x as f32 * 64.0), Val::Px(inventory.dim.y as f32 * 64.0)),
                ..default()
            },
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.35).into(),
            ..default()
        })
        .insert(RelativeCursorPosition::default())
        .with_children(|child_builder| {
            for y in 0..inventory.dim.y { for x in 0..inventory.dim.x {
                let (slot_entity, key_item_pair) = spawn_inventory_slot(&IVec2::new(x, y), inventory, child_builder, asset_map);
                inventory_panel.slot_images.push(slot_entity);
                if let Some(key_item_pair) = key_item_pair {
                    inventory_panel.item_images.push(key_item_pair);
                }
            }}
        })
        .insert(inventory_panel)
        .id()
}

pub fn spawn_inventory_slot(
    coord: &IVec2,
    inventory: &Inventory,
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
) -> (Entity, Option<(IVec2, Entity)>) {
    let mut item_image: Option<Handle<Image>> = None;
    let mut slot_image: Handle<Image>;

    let index = inventory.index_from_coord(&coord);
    if inventory.active_mask.is_bit_on(index) {
        if inventory.items[index].origin == *coord {
            item_image = Some(asset_map.get_image(inventory.items[index].get_def().image));
        }

        slot_image = asset_map.get_image(ImageAsset::InventoryTile02);
    } else {
        slot_image = asset_map.get_image(ImageAsset::InventoryTile01);
    }

    let slot_entity = child_builder.spawn(ImageBundle {
            style: Style {
                position: UiRect {
                    left: Val::Px((coord.x * 64) as f32),
                    bottom: Val::Px((coord.y * 64) as f32),
                    ..default()
                },
                position_type: PositionType::Absolute,
                size: Size::all(Val::Px(64.0)),
                ..default()
            },
            image: slot_image.into(),
            ..default()
        })
        .insert(InventorySlotUI)
        .id();

    if let Some(item_image) = item_image {
        let item_entity = child_builder.spawn(ImageBundle {
                style: Style {
                    position: UiRect {
                        left: Val::Px((coord.x * 64) as f32),
                        bottom: Val::Px((coord.y * 64) as f32),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px((inventory.items[index].dim.x * 64) as f32), Val::Px((inventory.items[index].dim.y * 64) as f32)),
                    ..default()
                },
                image: item_image.into(),
                z_index: ZIndex::Global(99),
                ..default()
            })
            .insert(InventoryItemUI)
            .id();

        return (slot_entity, Some((inventory.items[index].origin, item_entity)));
    }

    return (slot_entity, None);
}