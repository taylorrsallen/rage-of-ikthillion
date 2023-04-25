use crate::*;

//================================-================================-================================
pub struct EscapeMenuPlugin;
impl Plugin for EscapeMenuPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_systems((
                    despawn_escape_menu,
                ).in_schedule(OnExit(AppState::Gameplay)));
    }
}

//================================-================================-================================
#[derive(Component)]
pub struct EscapeMenuRoot;

//================================-================================-================================
pub fn despawn_escape_menu(
    mut commands: Commands,
    mut menu_states: ResMut<MenuStates>,
    escape_menu_query: Query<Entity, With<EscapeMenuRoot>>,
) {
    if let Ok(escape_menu_entity) = escape_menu_query.get_single() {
        commands.entity(escape_menu_entity).despawn_recursive();
        menu_states.escape = None;
    }
}

//================================-================================-================================
pub fn spawn_escape_menu(
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
        .insert(EscapeMenuRoot)
        .insert(Name::new("Escape Menu Root"))
        .with_children(|child_builder| {
            spawn_escape_menu_buttons(child_builder, &asset_map);
        })
        .id()
}

fn spawn_escape_menu_buttons(
    child_builder: &mut ChildBuilder,
    asset_map: &Res<AssetMap>,
) -> Entity {
    child_builder.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(30.0), Val::Percent(50.0)),
                ..default()
            },
            background_color: Color::rgb(0.1, 0.1, 0.1).into(),
            z_index: ZIndex::Global(1),
            ..default()
        })
        .insert(Name::new("Escape Menu Buttons"))
        .with_children(|child_builder| {
            spawn_button(UIButtonType::MainMenu, "MAIN MENU", child_builder, asset_map);
            spawn_button(UIButtonType::Quit, "QUIT", child_builder, asset_map);
        })
        .id()
}