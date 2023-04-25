use crate::*;
use bevy::{ui::RelativeCursorPosition, app::AppExit};

mod death;
mod escape_menu;
mod hud;
mod inventory_menu;
mod main_menu;
mod victory;
pub use death::*;
pub use escape_menu::*;
pub use hud::*;
pub use inventory_menu::*;
pub use main_menu::*;
pub use victory::*;

//================================-================================-================================
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_event::<DeathEvent>()
            .add_event::<VictoryEvent>()
            .insert_resource(MenuStates::default())
            .add_systems((
                    update_buttons,
                    play_button_sounds,
                ))
            .add_systems((
                    toggle_menus,
                    toggle_cursor,
                    receive_victory_event,
                    update_victory_menu,
                    receive_death_event,
                    update_death_state,
                ).in_set(OnUpdate(AppState::Gameplay)))
            .add_plugin(EscapeMenuPlugin)
            .add_plugin(HudPlugin)
            .add_plugin(InventoryMenuPlugin);
    }
}

//================================-================================-================================
#[derive(Default, Resource)]
pub struct MenuStates {
    escape: Option<Entity>,
    inventory: Option<Entity>,
    stats: Option<Entity>,
    victory: Option<Entity>,
    death: Option<Entity>,
    pub menu_open: bool,
}

struct Menu {

}

//================================-================================-================================
#[derive(Component)]
pub enum UIButtonType {
    Play,
    Quit,
    MainMenu,
}

//================================-================================-================================
pub fn toggle_menus(
    mut commands: Commands,
    mut menu_states: ResMut<MenuStates>,
    mut ui_sound_events: EventWriter<UISoundEvent>,
    action_state_query: Query<&ActionState<InputAction>>,
    asset_map: Res<AssetMap>,
) {
    if let Ok(action_state) = action_state_query.get_single() {
        if action_state.just_pressed(InputAction::MenuEscape) {
            if let Some(inventory_menu) = menu_states.inventory {
                ui_sound_events.send(UISoundEvent::new(SoundAsset::Cloth01, 1.0));
                commands.entity(inventory_menu).despawn_recursive();
                menu_states.inventory = None;
            } else if let Some(stats_menu) = menu_states.stats {
                commands.entity(stats_menu).despawn_recursive();
                menu_states.stats = None;
            } else if let Some(escape_menu) = menu_states.escape {
                commands.entity(escape_menu).despawn_recursive();
                menu_states.escape = None;
            } else {
                menu_states.escape = Some(spawn_escape_menu(&mut commands, &asset_map));
            }
        }
    
        if menu_states.escape == None && menu_states.inventory == None && menu_states.stats == None {
            menu_states.menu_open = false;
        } else {
            menu_states.menu_open = true;
        }
    }
}

fn update_buttons(
    mut button_query: Query<(&UIButtonType, &Interaction, &RelativeCursorPosition, &mut BackgroundColor)>,
    mut next_state: ResMut<NextState<AppState>>,
    mut exit_events: EventWriter<AppExit>,
    asset_map: Res<AssetMap>,
) {
    for (button, interaction, relative_cursor_position, mut color) in button_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if let Some(position) = relative_cursor_position.normalized {
                    if position.x < 0.0 || position.x > 1.0 || position.y < 0.0 || position.y > 1.0 {
                        *color = Color::rgb(0.15, 0.15, 0.15).into();
                    } else {
                        *color = Color::rgb(0.35, 0.75, 0.35).into();
                        match button {
                            UIButtonType::Play => {
                                next_state.set(AppState::Gameplay);
                            }
                            UIButtonType::Quit => {
                                exit_events.send(AppExit);
                            }
                            UIButtonType::MainMenu => {
                                next_state.set(AppState::MainMenu);
                            }
                            _ => {
                                continue;
                            }
                        }
                    }
                } else {
                    *color = Color::rgb(0.15, 0.15, 0.15).into();
                }
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.20, 0.20, 0.20).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn play_button_sounds(
    mut commands: Commands,
    mut ui_sound_events: EventWriter<UISoundEvent>,
    button_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    asset_map: Res<AssetMap>,
) {
    for interaction in button_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                ui_sound_events.send(UISoundEvent::new(SoundAsset::Tick01, 0.6));
            }
            Interaction::Hovered => {
                ui_sound_events.send(UISoundEvent::new(SoundAsset::Tick00, 3.0));
            }
            Interaction::None => {
                
            }
        }
    }
}

fn toggle_cursor(
    mut windows_query: Query<&mut Window>,
    menu_states: Res<MenuStates>,
) {
    let mut window = windows_query.single_mut();
    if window.focused {
        if menu_states.escape != None || menu_states.inventory != None || menu_states.stats != None || menu_states.victory != None || menu_states.death != None {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        } else {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
    } else {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}
//================================-================================-================================
pub fn spawn_button(
    button_type: UIButtonType,
    button_text: &str,
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
) -> Entity {
    child_builder.spawn(ButtonBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                margin: UiRect {
                    bottom: Val::Percent(5.0),
                    ..default()
                },
                size: Size::new(Val::Percent(30.0), Val::Percent(10.0)),
                ..default()
            },
            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..default()
        })
        .insert(button_type)
        .insert(RelativeCursorPosition::default())
        .with_children(|child_builder| {
            child_builder.spawn(TextBundle {
                text: Text::from_section(button_text.to_string(), TextStyle {
                    font: asset_map.get_font(FontAsset::HackRegular),
                    font_size: 32.0,
                    color: Color::WHITE,
                }),
                ..default()
            });
        })
        .id()
}