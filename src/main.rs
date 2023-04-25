#![allow(unused)]

use bevy::{
    prelude::*,
    window::*
};

mod game;
mod utils;
pub use game::*;
pub use utils::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: (1600.0, 900.0).into(),
                    present_mode: PresentMode::AutoNoVsync,
                    mode: WindowMode::Windowed,
                    title: "The Rage of Ikthillion".into(),
                    resizable: true,
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }))

        .add_plugin(GamePlugin)

        .run();
}
