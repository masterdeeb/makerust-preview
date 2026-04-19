pub mod theme;
pub mod components;

use bevy::prelude::*;
use components::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Root Node (Full Screen)
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        background_color: BackgroundColor(theme::BG_BASE),
        ..default()
    }).with_children(|root| {
        
        // Sidebar
        spawn_sidebar(root);

        // Main Content Area
        spawn_main_content(root);

    });
}
