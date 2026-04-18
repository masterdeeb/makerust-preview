pub mod ui;
pub mod systems;

use bevy::prelude::*;
use ui::UiPlugin;
use systems::LogicPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, LogicPlugin));
    }
}

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_wasm() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pro Task Manager".into(),
                resolution: (1280.0, 720.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AppPlugin)
        .run();
}
