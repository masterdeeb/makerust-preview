pub mod core_logic;

use bevy::prelude::*;

pub struct LogicPlugin;

impl Plugin for LogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(core_logic::CoreLogicPlugin);
    }
}