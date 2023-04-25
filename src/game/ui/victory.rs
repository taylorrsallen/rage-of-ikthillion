use crate::*;

pub struct VictoryEvent;
#[derive(Component)]
pub struct VictoryMenuRoot;
#[derive(Component)]
pub struct InnocentsKilledUI;
#[derive(Component)]
pub struct EvilVanquishedUI;

pub fn update_victory_menu(
    mut victory_menu_query: Query<&mut Style, With<VictoryMenuRoot>>,
    mut innocents_query: Query<&mut Text, (With<InnocentsKilledUI>, Without<EvilVanquishedUI>)>,
    mut evils_query: Query<&mut Text, (Without<InnocentsKilledUI>, With<EvilVanquishedUI>)>,
    mut settings: ResMut<Settings>,
    time: Res<Time>,
) {
    if settings.ikthillion_unraged {
        settings.victory_text_timer.tick(time.delta());
        if settings.victory_text_timer.finished() {
            settings.victory_menu_offset += time.delta_seconds() * 2.0;
            let mut victory_menu = victory_menu_query.single_mut();
            victory_menu.position.top = Val::Percent(40.0 - settings.victory_menu_offset);
        }
    
        if let Ok(mut innocents_text) = innocents_query.get_single_mut() {
            innocents_text.sections[0].value = "INNOCENTS MURDERED: ".to_string() + &settings.innocents_murdered.to_string();
        }
        if let Ok(mut evils_text) = evils_query.get_single_mut() {
            evils_text.sections[0].value = "EVIL VANQUISHED: ".to_string() + &settings.evil_vanquished.to_string()
        }
    }
}

pub fn receive_victory_event(
    mut commands: Commands,
    mut victory_events: EventReader<VictoryEvent>,
    mut menu_states: ResMut<MenuStates>,
    mut settings: ResMut<Settings>,
    asset_map: Res<AssetMap>,
) {
    for victory in victory_events.iter() {
        settings.ikthillion_unraged = true;
        commands.spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    top: Val::Percent(40.0),
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
        .insert(VictoryMenuRoot)
        .insert(Name::new("Victory Menu Root"))
        .with_children(|child_builder| {
            child_builder.spawn(TextBundle {
                text: Text::from_section("YOU HAVE SAVED ALL IKTHIA", TextStyle {
                    font: asset_map.get_font(FontAsset::HackBold),
                    font_size: 48.0,
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
            });

            child_builder.spawn(TextBundle {
                text: Text::from_section("INNOCENTS MURDERED: ".to_string() + &settings.innocents_murdered.to_string(), TextStyle {
                    font: asset_map.get_font(FontAsset::HackRegular),
                    font_size: 48.0,
                    color: Color::WHITE,
                }),
                ..default()
            })
            .insert(InnocentsKilledUI);
            child_builder.spawn(TextBundle {
                text: Text::from_section("EVIL VANQUISHED: ".to_string() + &settings.evil_vanquished.to_string(), TextStyle {
                    font: asset_map.get_font(FontAsset::HackRegular),
                    font_size: 48.0,
                    color: Color::WHITE,
                }),
                style: Style {
                    margin: UiRect {
                        bottom: Val::Percent(30.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .insert(EvilVanquishedUI);

            child_builder.spawn(TextBundle {
                    text: Text::from_section("THANKS FOR PLAYING!", TextStyle {
                        font: asset_map.get_font(FontAsset::HackItalic),
                        font_size: 36.0,
                        color: Color::WHITE,
                    }),
                    ..default()
                });
        })
        .id();
    }
}