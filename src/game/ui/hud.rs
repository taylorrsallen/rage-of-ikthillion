use crate::*;

//================================-================================-================================
pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems((
                    update_experience_bar,
                    update_stat_displays,
                    update_health_and_stamina_bars,
                    update_world_space_ui,
                ).in_set(OnUpdate(AppState::Gameplay)))
            .add_system(spawn_hud.in_schedule(OnEnter(AppState::Gameplay)))
            .add_system(despawn_hud.in_schedule(OnExit(AppState::Gameplay)));
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HudHealthBar;
#[derive(Component)]
pub struct HudHealthBarBG;
#[derive(Component)]
pub struct HudStaminaBar;
#[derive(Component)]
pub struct HudStaminaBarBG;

#[derive(Component)]
pub struct HudVitalityDisplay;
#[derive(Component)]
pub struct HudEnduranceDisplay;
#[derive(Component)]
pub struct HudStrengthDisplay;
#[derive(Component)]
pub struct HudDexterityDisplay;
#[derive(Component)]
pub struct HudLuckDisplay;

#[derive(Component)]
pub struct HudLevelDisplay;
#[derive(Component)]
pub struct HudExperienceBar;

//================================-================================-================================
fn update_health_and_stamina_bars(
    mut hud_health_bar_query: Query<&mut Style, (With<HudHealthBar>, Without<HudHealthBarBG>, Without<HudStaminaBar>, Without<HudStaminaBarBG>)>,
    mut hud_health_bar_bg_query: Query<&mut Style, (Without<HudHealthBar>, With<HudHealthBarBG>, Without<HudStaminaBar>, Without<HudStaminaBarBG>)>,
    mut hud_stamina_bar_query: Query<&mut Style, (Without<HudHealthBar>, Without<HudHealthBarBG>, With<HudStaminaBar>, Without<HudStaminaBarBG>)>,
    mut hud_stamina_bar_bg_query: Query<&mut Style, (Without<HudHealthBar>, Without<HudHealthBarBG>, Without<HudStaminaBar>, With<HudStaminaBarBG>)>,
    player_stats_query: Query<&Stats, With<Player>>,
) {
    let mut health_bar = hud_health_bar_query.single_mut();
    let mut health_bar_bg = hud_health_bar_bg_query.single_mut();
    let mut stamina_bar = hud_stamina_bar_query.single_mut();
    let mut stamina_bar_bg = hud_stamina_bar_bg_query.single_mut();
    if let Ok(player_stats) = player_stats_query.get_single() {
        let max_health = 5.0 + (player_stats.vitality as f32 * 1.25) + (player_stats.endurance as f32 * 1.1) + player_stats.strength as f32;
        let health_percent = player_stats.health / max_health;
        health_bar.size = Size::new(Val::Px(max_health * health_percent), Val::Percent(1.5));
        health_bar_bg.size = Size::new(Val::Px(max_health), Val::Percent(1.5));
    
        let max_stamina = 30.0 + player_stats.vitality as f32 + (player_stats.endurance as f32 * 1.25);
        let stamina_percent = player_stats.stamina / max_stamina;
        stamina_bar.size = Size::new(Val::Px(max_stamina * stamina_percent), Val::Percent(1.5));
        stamina_bar_bg.size = Size::new(Val::Px(max_stamina), Val::Percent(1.5));
    }
}

fn update_stat_displays(
    mut hud_vitality_query: Query<&mut Text, (With<HudVitalityDisplay>, Without<HudEnduranceDisplay>, Without<HudStrengthDisplay>, Without<HudDexterityDisplay>, Without<HudLuckDisplay>)>,
    mut hud_endurance_query: Query<&mut Text, (Without<HudVitalityDisplay>, With<HudEnduranceDisplay>, Without<HudStrengthDisplay>, Without<HudDexterityDisplay>, Without<HudLuckDisplay>)>,
    mut hud_strength_query: Query<&mut Text, (Without<HudVitalityDisplay>, Without<HudEnduranceDisplay>, With<HudStrengthDisplay>, Without<HudDexterityDisplay>, Without<HudLuckDisplay>)>,
    mut hud_dexterity_query: Query<&mut Text, (Without<HudVitalityDisplay>, Without<HudEnduranceDisplay>, Without<HudStrengthDisplay>, With<HudDexterityDisplay>, Without<HudLuckDisplay>)>,
    mut hud_luck_query: Query<&mut Text, (Without<HudVitalityDisplay>, Without<HudEnduranceDisplay>, Without<HudStrengthDisplay>, Without<HudDexterityDisplay>, With<HudLuckDisplay>)>,
    player_stats_query: Query<&Stats, With<Player>>,
) {
    let mut vitality = hud_vitality_query.single_mut();
    let mut endurance = hud_endurance_query.single_mut();
    let mut strength = hud_strength_query.single_mut();
    let mut dexterity = hud_dexterity_query.single_mut();
    let mut luck = hud_luck_query.single_mut();
    if let Ok(player_stats) = player_stats_query.get_single() {
        vitality.sections[0].value = player_stats.vitality.to_string();
        endurance.sections[0].value = player_stats.endurance.to_string();
        strength.sections[0].value = player_stats.strength.to_string();
        dexterity.sections[0].value = player_stats.dexterity.to_string();
        luck.sections[0].value = player_stats.luck.to_string();
    }
}

fn update_experience_bar(
    mut hud_level_display_query: Query<&mut Text, (With<HudLevelDisplay>, Without<HudExperienceBar>)>,
    mut hud_experience_bar_query: Query<&mut Style, (Without<HudLevelDisplay>, With<HudExperienceBar>)>,
    player_stats_query: Query<&Stats, With<Player>>,
) {
    let mut level_display = hud_level_display_query.single_mut();
    let mut experience_bar = hud_experience_bar_query.single_mut();
    if let Ok(player_stats) = player_stats_query.get_single() {
        // level display
        level_display.sections[0].value = player_stats.level.to_string();
    
        // experience bar
        let experience_percent = (player_stats.experience / player_stats.level as f32) * 100.0;
        experience_bar.size = Size::new(Val::Percent(experience_percent), Val::Percent(1.0));
    }
}

fn spawn_hud(
    mut commands: Commands,
    asset_map: Res<AssetMap>,
) {
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
        .insert(HudRoot)
        .insert(Name::new("Hud Root"))
        .with_children(|child_builder| {
            spawn_reticle(child_builder);
            spawn_health_and_stamina_bar(child_builder, &asset_map);
            spawn_stat_displays(child_builder, &asset_map);
            spawn_level_display(child_builder, &asset_map);
            spawn_experience_bar(child_builder, &asset_map);
            spawn_ikthillion_marker(child_builder, &asset_map);
        });
}

fn despawn_hud(
    mut commands: Commands,
    hud_query: Query<Entity, With<HudRoot>>,
) {
    let entity = hud_query.single();
    commands.entity(entity).despawn_recursive();
}

//================================-================================-================================
fn spawn_reticle(
    child_builder: &mut ChildBuilder,
) -> Entity {
    child_builder.spawn(NodeBundle {
            style: Style {
                align_self: AlignSelf::Center,
                size: Size::all(Val::Px(3.0)),
                ..default()
            },
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.35).into(),
            ..default()
        })
        .id()
}

//================================-================================-================================
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct WorldSpaceUI;

#[derive(Component)]
pub struct IkthillionTracker;

fn spawn_ikthillion_marker(
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
) {
    let icon_size = 4.0;
    let icon_right_offset = 3.0;
    let icon_bottom_offset = 3.0;
    let icon_margin = 3.5;
    let text_offset = 2.5;

    child_builder.spawn(ImageBundle {
            style: Style {
                position: UiRect {
                    left: Val::Percent(icon_right_offset + icon_margin * 1 as f32),
                    bottom: Val::Percent(icon_bottom_offset),
                    ..default()
                },
                position_type: PositionType::Absolute,
                size: Size::height(Val::Percent(icon_size)),
                ..default()
            },
            image: asset_map.get_image(ImageAsset::IkthillionMarker).into(),
            ..default()
        })
        .insert(WorldSpaceUI);
}

fn update_world_space_ui(
    mut text_query: Query<(&mut Style, &CalculatedSize, &WorldSpaceUI)>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    transforms_query: Query<&GlobalTransform, With<IkthillionTracker>>,
) {
    if let Ok((camera_transform, camera)) = camera_query.get_single() {
        if let Some(target_size) = camera.logical_viewport_size() {
            for (mut text_style, text_calculated_size, world_space_ui_text) in text_query.iter_mut() {
                if let Ok(follow_transform) = transforms_query.get_single() {
                    if let Some(text_position) = camera.world_to_viewport(camera_transform, follow_transform.translation()) {
                        text_style.position.left = Val::Px(text_position.x - (text_calculated_size.size.x * 0.5));
                        text_style.position.top = Val::Px(target_size.y - text_position.y);
                    }
                }
            }
        }
    }
}

//================================-================================-================================
fn spawn_stat_displays(
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
) {
    let icon_size = 4.0;
    let icon_right_offset = 3.0;
    let icon_bottom_offset = 3.0;
    let icon_margin = 3.5;
    let text_offset = 2.5;

    child_builder.spawn(ImageBundle {
            style: Style {
                position: UiRect {
                    left: Val::Percent(icon_right_offset + icon_margin * 1 as f32),
                    bottom: Val::Percent(icon_bottom_offset),
                    ..default()
                },
                position_type: PositionType::Absolute,
                size: Size::height(Val::Percent(icon_size)),
                ..default()
            },
            image: asset_map.get_image(ImageAsset::Backpack).into(),
            ..default()
        });
    child_builder.spawn(TextBundle {
            style: Style {
                position: UiRect {
                    left: Val::Percent((icon_right_offset + icon_margin * 1 as f32) + icon_size * 0.25),
                    bottom: Val::Percent(icon_bottom_offset - text_offset * 1.25),
                    ..default()
                },
                position_type: PositionType::Absolute,
                size: Size::height(Val::Percent(icon_size)),
                ..default()
            },
            text: Text::from_section(
                "TAB",
                TextStyle {
                    font: asset_map.get_font(FontAsset::HackBold),
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                }
            ).with_alignment(TextAlignment::Center),
            ..default()
        });

    let (vit_style, vit_image, vit_text_style, vit_name_style, vit_text) = build_stat_display(child_builder, asset_map, "VIT", ImageAsset::Vitality, icon_size, icon_right_offset, icon_bottom_offset, icon_margin, 5, text_offset);
    child_builder.spawn(ImageBundle {
            style: vit_style,
            image: vit_image,
            ..default()
        });
    child_builder.spawn(TextBundle {
            style: vit_text_style,
            text: vit_text.clone(),
            ..default()
        })
        .insert(HudVitalityDisplay);
    child_builder.spawn(TextBundle {
        style: vit_name_style,
        text: vit_text,
        ..default()
    });

    let (end_style, end_image, end_text_style, end_name_style, end_text) = build_stat_display(child_builder, asset_map, "END", ImageAsset::Endurance, icon_size, icon_right_offset, icon_bottom_offset, icon_margin, 4, text_offset);
    child_builder.spawn(ImageBundle {
            style: end_style,
            image: end_image,
            ..default()
        });
    child_builder.spawn(TextBundle {
            style: end_text_style,
            text: end_text.clone(),
            ..default()
        })
        .insert(HudEnduranceDisplay);
    child_builder.spawn(TextBundle {
        style: end_name_style,
        text: end_text,
        ..default()
    });

    let (str_style, str_image, str_text_style, str_name_style, str_text) = build_stat_display(child_builder, asset_map, "STR", ImageAsset::Strength, icon_size, icon_right_offset, icon_bottom_offset, icon_margin, 3, text_offset);
    child_builder.spawn(ImageBundle {
            style: str_style,
            image: str_image,
            ..default()
        });
    child_builder.spawn(TextBundle {
            style: str_text_style,
            text: str_text.clone(),
            ..default()
        })
        .insert(HudStrengthDisplay);
    child_builder.spawn(TextBundle {
        style: str_name_style,
        text: str_text,
        ..default()
    });

    let (dex_style, dex_image, dex_text_style, dex_name_style, dex_text) = build_stat_display(child_builder, asset_map, "DEX", ImageAsset::Dexterity, icon_size, icon_right_offset, icon_bottom_offset, icon_margin, 2, text_offset);
    child_builder.spawn(ImageBundle {
            style: dex_style,
            image: dex_image,
            ..default()
        });
    child_builder.spawn(TextBundle {
            style: dex_text_style,
            text: dex_text.clone(),
            ..default()
        })
        .insert(HudDexterityDisplay);
    child_builder.spawn(TextBundle {
        style: dex_name_style,
        text: dex_text,
        ..default()
    });

    let (luk_style, luk_image, luk_text_style, luk_name_style, luk_text) = build_stat_display(child_builder, asset_map, "LUK", ImageAsset::Luck, icon_size, icon_right_offset, icon_bottom_offset, icon_margin, 1, text_offset);
    child_builder.spawn(ImageBundle {
            style: luk_style,
            image: luk_image,
            ..default()
        });
    child_builder.spawn(TextBundle {
            style: luk_text_style,
            text: luk_text.clone(),
            ..default()
        })
        .insert(HudLuckDisplay);
    child_builder.spawn(TextBundle {
            style: luk_name_style,
            text: luk_text,
            ..default()
        });
}

fn build_stat_display(
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
    name: &str,
    image: ImageAsset,
    icon_size: f32,
    icon_right_offset: f32,
    icon_bottom_offset: f32,
    icon_margin: f32,
    icon_num: u32,
    text_offset: f32,
) -> (Style, UiImage, Style, Style, Text) {
    (
        Style {
            position: UiRect {
                right: Val::Percent(icon_right_offset + icon_margin * icon_num as f32),
                bottom: Val::Percent(icon_bottom_offset),
                ..default()
            },
            position_type: PositionType::Absolute,
            size: Size::height(Val::Percent(icon_size)),
            ..default()
        },
        asset_map.get_image(image).into(),
        Style {
            position: UiRect {
                right: Val::Percent((icon_right_offset + icon_margin * icon_num as f32) + icon_size * 0.25),
                bottom: Val::Percent(icon_bottom_offset + text_offset),
                ..default()
            },
            position_type: PositionType::Absolute,
            size: Size::height(Val::Percent(icon_size)),
            ..default()
        },
        Style {
            position: UiRect {
                right: Val::Percent((icon_right_offset + icon_margin * icon_num as f32) + icon_size * 0.25),
                bottom: Val::Percent(icon_bottom_offset - text_offset * 1.25),
                ..default()
            },
            position_type: PositionType::Absolute,
            size: Size::height(Val::Percent(icon_size)),
            ..default()
        },
        Text::from_section(
            name,
            TextStyle {
                font: asset_map.get_font(FontAsset::HackBoldItalic),
                font_size: 18.0,
                color: Color::WHITE,
                ..default()
            }
        ).with_alignment(TextAlignment::Center)
    )
}

//================================-================================-================================
fn spawn_health_and_stamina_bar(
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
) {
    child_builder.spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Percent(1.5),
                    top: Val::Percent(3.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(10.0), Val::Percent(1.5)),
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..default()
        })
        .insert(HudHealthBarBG);
    
    child_builder.spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Percent(1.5),
                    top: Val::Percent(3.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Percent(10.0), Val::Percent(1.5)),
                ..default()
            },
            background_color: Color::RED.into(),
            ..default()
        })
        .insert(HudHealthBar);

    child_builder.spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Percent(1.5),
                    top: Val::Percent(5.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(10.0), Val::Percent(1.5)),
                ..default()
            },
            background_color: Color::DARK_GRAY.into(),
            ..default()
        })
        .insert(HudStaminaBarBG);;
    
    child_builder.spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Percent(1.5),
                    top: Val::Percent(5.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Percent(10.0), Val::Percent(1.5)),
                ..default()
            },
            background_color: Color::SEA_GREEN.into(),
            ..default()
        })
        .insert(HudStaminaBar);
}

//================================-================================-================================
fn spawn_experience_bar(
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
) -> Entity {
    child_builder.spawn(NodeBundle {
        style: Style {
            position: UiRect {
                left: Val::Percent(0.0),
                bottom: Val::Percent(0.0),
                ..default()
            },
            position_type: PositionType::Absolute,
            size: Size::new(Val::Percent(100.0), Val::Percent(1.0)),
            ..default()
        },
        background_color: Color::DARK_GRAY.into(),
        ..default()
    });
    
    child_builder.spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Percent(0.0),
                    bottom: Val::Percent(0.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Row,
                size: Size::new(Val::Percent(100.0), Val::Percent(1.0)),
                ..default()
            },
            background_color: Color::YELLOW_GREEN.into(),
            ..default()
        })
        .insert(HudExperienceBar)
        .id()
}

//================================-================================-================================
fn spawn_level_display(
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
) -> Entity {
    child_builder.spawn(NodeBundle {
            style: Style {
                position: UiRect {
                    left: Val::Percent(47.5),
                    bottom: Val::Percent(1.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                // align_self: AlignSelf::Center,
                size: Size::new(Val::Percent(5.0), Val::Percent(5.0)),
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .with_children(|child_builder| {
            child_builder.spawn(TextBundle {
                    style: Style {
                        position: UiRect {
                            top: Val::Percent(10.0),
                            ..default()
                        },
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_section("1", TextStyle {
                        font: asset_map.get_font(FontAsset::HackRegular),
                        font_size: 32.0,
                        color: Color::WHITE,
                    }).with_alignment(TextAlignment::Center),
                    ..default()
                })
                .insert(HudLevelDisplay);
        })
        .id()
}