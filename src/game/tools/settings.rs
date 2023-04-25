use crate::*;

//================================-================================-================================
pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.insert_resource(Settings::default());
    }
}

//================================-================================-================================
#[derive(Resource)]
pub struct Settings {
    pub look_sensitivity: f32,
    pub player_spawn: IVec3,
    pub inventory: Inventory,
    pub innocents_murdered: u32,
    pub evil_vanquished: u32,
    pub ikthillion_unraged: bool,
    pub victory_text_timer: Timer,
    pub victory_menu_offset: f32,
    pub player_died: bool,
    pub death_timer: Timer,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            look_sensitivity: 0.3,
            player_spawn: IVec3::ZERO,
            inventory: Inventory::new(&IVec2::ONE),
            innocents_murdered: 0,
            evil_vanquished: 0,
            ikthillion_unraged: false,
            victory_text_timer: Timer::from_seconds(15.0, TimerMode::Once),
            victory_menu_offset: 0.0,
            player_died: false,
            death_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }
}

//================================-================================-================================
// fn startup_settings(
//     mut commands: Commands,
// ) {
    // LOAD THE SETTINGS
//     commands.insert_resource(Settings::default());
// }