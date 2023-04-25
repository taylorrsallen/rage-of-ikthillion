use crate::*;
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        clear_color::ClearColorConfig,
    },
    ui::RelativeCursorPosition,
};

//================================-================================-================================
#[derive(Component)]
pub struct MainMenuCamera;

#[derive(Component)]
pub struct MainMenuRoot;

pub fn spawn_main_menu(
    commands: &mut Commands,
    asset_map: &Res<AssetMap>,
) {
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
        .insert(MainMenuRoot)
        .insert(Name::new("Main Menu Root"))
        .with_children(|child_builder| {
            spawn_logo(child_builder, asset_map);
            spawn_button(UIButtonType::Play, "PLAY", child_builder, asset_map);
            spawn_button(UIButtonType::Quit, "QUIT", child_builder, asset_map);
        });
}

fn spawn_logo(
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
) {
    child_builder.spawn(NodeBundle {
        style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect {
                bottom: Val::Percent(10.0),
                ..default()
            },
            size: Size::new(Val::Percent(80.0), Val::Percent(25.0)),
            ..default()
        },
        // background_color: Color::rgb(0.75, 0.15, 0.15).into(),
        ..default()
    })
    .with_children(|child_builder| {
        child_builder.spawn(TextBundle {
            text: Text::from_section("RAGE OF IKTHILLION".to_string(), TextStyle {
                font: asset_map.get_font(FontAsset::HackBold),
                font_size: 96.0,
                color: Color::WHITE,
            }),
            ..default()
        });
    });
}