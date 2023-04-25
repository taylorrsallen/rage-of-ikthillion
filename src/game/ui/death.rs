use crate::*;

pub struct DeathEvent;
#[derive(Component)]
pub struct DeathMenuRoot;
#[derive(Component)]
pub struct DeathText;

pub fn update_death_state(
    mut settings: ResMut<Settings>,
    mut next_state: ResMut<NextState<AppState>>,
    mut death_text_query: Query<&mut Text, With<DeathText>>,
    time: Res<Time>,
) {
    if settings.player_died {
        if let Ok(mut death_text) = death_text_query.get_single_mut() {
            // death_text.sections[0].style.font_size += 1.0 * time.delta_seconds();
        }
        settings.death_timer.tick(time.delta());
        if settings.death_timer.finished() {
            next_state.set(AppState::MainMenu);
        }
    }
}

pub fn receive_death_event(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    mut menu_states: ResMut<MenuStates>,
    mut settings: ResMut<Settings>,
    asset_map: Res<AssetMap>,
) {
    for death in death_events.iter() {
        settings.player_died = true;
        settings.death_timer.reset();
        let death_menu = commands.spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    top: Val::Percent(30.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .insert(DeathMenuRoot)
        .insert(Name::new("Death Menu Root"))
        .with_children(|child_builder| {
            child_builder.spawn(TextBundle {
                text: Text::from_section("DEATH", TextStyle {
                    font: asset_map.get_font(FontAsset::HackBold),
                    font_size: 96.0,
                    color: Color::WHITE,
                }),
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(3.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .insert(DeathText);
            
            // spawn_button(UIButtonType::Respawn, "Respawn", child_builder, &asset_map);
            // spawn_button(UIButtonType::MainMenu, "Main Menu", child_builder, &asset_map);
        })
        .id();
    }
}