pub mod theme;
pub mod components;
pub mod interactions;
pub mod render;

use bevy::prelude::*;
use theme::*;
use components::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, (
               interactions::button_hover_system,
               interactions::handle_add_task_click,
               interactions::handle_toggle_task_click,
               interactions::handle_delete_task_click,
               render::render_task_list,
           ));
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
        background_color: BackgroundColor(BG_BASE),
        ..default()
    }).with_children(|root| {
        
        // Sidebar
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Px(250.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(24.0)),
                border: UiRect::right(Val::Px(BORDER_WIDTH)),
                gap: Size::all(Val::Px(16.0)),
                ..default()
            },
            background_color: BackgroundColor(BG_SURFACE),
            border_color: BorderColor(BORDER),
            ..default()
        }).with_children(|sidebar| {
            // App Logo / Title
            sidebar.spawn(TextBundle::from_section(
                "TaskPro.",
                TextStyle {
                    font_size: 28.0,
                    color: ACCENT,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::bottom(Val::Px(32.0)),
                ..default()
            }));

            // Dummy Navigation Items
            spawn_nav_item(sidebar, "Inbox", true);
            spawn_nav_item(sidebar, "Today", false);
            spawn_nav_item(sidebar, "Upcoming", false);
        });

        // Main Content Area
        root.spawn(NodeBundle {
            style: Style {
                flex_grow: 1.0,
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).with_children(|main| {
            
            // Header
            main.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    padding: UiRect::axes(Val::Px(40.0), Val::Px(32.0)),
                    border: UiRect::bottom(Val::Px(BORDER_WIDTH)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                border_color: BorderColor(BORDER),
                ..default()
            }).with_children(|header| {
                header.spawn(TextBundle::from_section(
                    "Inbox",
                    TextStyle {
                        font_size: 32.0,
                        color: TEXT_MAIN,
                        ..default()
                    },
                ));

                // Add Task Button
                header.spawn((ButtonBundle {
                    style: Style {
                        padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                        background_color: ACCENT.into(),
                        ..default()
                    },
                    background_color: BackgroundColor(ACCENT),
                    ..default()
                }, AddTaskButton)).with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "+ Add Task",
                        TextStyle {
                            font_size: 16.0,
                            color: BG_BASE, // Dark text on light accent
                            ..default()
                        },
                    ));
                });
            });

            // Task List Container (Scrollable area placeholder)
            main.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::axes(Val::Px(40.0), Val::Px(20.0)),
                    ..default()
                },
                ..default()
            }, TaskListContainer));
        });
    });
}

fn spawn_nav_item(parent: &mut ChildBuilder, text: &str, active: bool) {
    let bg_color = if active { BG_OVERLAY } else { Color::NONE };
    let text_color = if active { TEXT_MAIN } else { TEXT_MUTED };
    let border_color = if active { ACCENT } else { Color::NONE };

    parent.spawn(ButtonBundle {
        style: Style {
            width: Val::Percent(100.0),
            padding: UiRect::axes(Val::Px(16.0), Val::Px(12.0)),
            border: UiRect::left(Val::Px(3.0)),
            ..default()
        },
        background_color: BackgroundColor(bg_color),
        border_color: BorderColor(border_color),
        ..default()
    }).with_children(|btn| {
        btn.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 16.0,
                color: text_color,
                ..default()
            },
        ));
    });
}
