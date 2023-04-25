use crate::*;

mod asset_map;
mod audio_player;
mod input;
mod lifetime;
mod move_to;
mod path_anim;
mod settings;
mod state;
mod tasks;
pub use asset_map::*;
pub use audio_player::*;
pub use input::*;
pub use lifetime::*;
pub use move_to::*;
pub use path_anim::*;
pub use settings::*;
pub use state::*;
pub use tasks::*;

//================================-================================-================================
pub struct ToolsPlugin;
impl Plugin for ToolsPlugin {
    fn build(
        &self,
        app: &mut App,
    ) {
        app.add_plugin(AssetMapPlugin)
            .add_plugin(AudioPlayerPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(LifetimePlugin)
            .add_plugin(MoveToPlugin)
            .add_plugin(PathAnimPlugin)
            .add_plugin(SettingsPlugin)
            .add_plugin(StatePlugin)
            .add_plugin(TasksPlugin);
    }
}