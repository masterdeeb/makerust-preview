use bevy::prelude::*;

pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        // Register systems like: app.add_systems(Update, my_logic_system);
    }
}