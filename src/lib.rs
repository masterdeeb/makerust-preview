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