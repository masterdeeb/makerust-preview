pub mod theme;
pub mod components;

use bevy::prelude::*;
use theme::*;
use components::*;
use crate::systems::core_logic::{Task, TaskStore};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, (button_interaction_system, render_tasks_system));
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Root Node
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        background_color: BG_APP.into(),
        ..default()
    }).with_children(|root| {
        
        // --- SIDEBAR ---
        root.spawn(NodeBundle {
            style: Style {
                width: Val::Px(260.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            background_color: BG_SIDEBAR.into(),
            ..default()
        }).with_children(|sidebar| {
            // App Logo / Title
            sidebar.spawn(TextBundle::from_section(
                "✓ TaskFlow",
                TextStyle {
                    font_size: 28.0,
                    color: ACCENT_PRIMARY,
                    ..default()
                },
            ).with_style(Style { margin: UiRect::bottom(Val::Px(40.0)), ..default() }));

            // Menu Item
            spawn_sidebar_menu_item(sidebar, "All Tasks", true);
            spawn_sidebar_menu_item(sidebar, "Today", false);
            spawn_sidebar_menu_item(sidebar, "Completed", false);
        });

        // --- MAIN CONTENT ---
        root.spawn(NodeBundle {
            style: Style {
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(40.0)),
                ..default()
            },
            ..default()
        }).with_children(|main| {
            
            // Header Row
            main.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
                ..default()
            }).with_children(|header| {
                header.spawn(TextBundle::from_section(
                    "All Tasks",
                    TextStyle {
                        font_size: 36.0,
                        color: TEXT_MAIN,
                        ..default()
                    },
                ));

                // Add Task Button
                header.spawn((ButtonBundle {
                    style: Style {
                        padding: UiRect::axes(Val::Px(20.0), Val::Px(12.0)),
                        border_radius: BorderRadius::all(RADIUS_BUTTON),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: ACCENT_PRIMARY.into(),
                    ..default()
                }, ButtonAction::AddTask)).with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "+ New Task",
                        TextStyle {
                            font_size: 16.0,
                            color: BG_APP,
                            ..default()
                        },
                    ));
                });
            });

            // Task List Container
            main.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    flex_grow: 1.0,
                    overflow: Overflow::clip_y(),
                    ..default()
                },
                ..default()
            }, TaskListContainer));
        });
    });
}

fn spawn_sidebar_menu_item(parent: &mut ChildBuilder, text: &str, active: bool) {
    let bg_color = if active { BG_CARD } else { Color::NONE };
    let text_color = if active { TEXT_MAIN } else { TEXT_MUTED };

    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Percent(100.0),
            padding: UiRect::axes(Val::Px(16.0), Val::Px(12.0)),
            margin: UiRect::bottom(Val::Px(8.0)),
            border_radius: BorderRadius::all(RADIUS_BUTTON),
            ..default()
        },
        background_color: bg_color.into(),
        ..default()
    }, ButtonAction::SidebarMenu)).with_children(|btn| {
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

// --- SYSTEMS ---

fn render_tasks_system(
    mut commands: Commands,
    task_store: Res<TaskStore>,
    container_query: Query<Entity, With<TaskListContainer>>,
) {
    if !task_store.is_changed() {
        return;
    }

    let Ok(container_entity) = container_query.get_single() else { return };

    // Clear existing tasks in UI
    commands.entity(container_entity).despawn_descendants();

    // Rebuild tasks
    commands.entity(container_entity).with_children(|parent| {
        if task_store.tasks.is_empty() {
            parent.spawn(TextBundle::from_section(
                "No tasks yet. Enjoy your day!",
                TextStyle { font_size: 18.0, color: TEXT_MUTED, ..default() }
            ).with_style(Style { margin: UiRect::top(Val::Px(20.0)), ..default() }));
            return;
        }

        for task in &task_store.tasks {
            spawn_task_card(parent, task);
        }
    });
}

fn spawn_task_card(parent: &mut ChildBuilder, task: &Task) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            padding: UiRect::all(PADDING_CARD),
            margin: UiRect::bottom(Val::Px(12.0)),
            border_radius: BorderRadius::all(RADIUS_CARD),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        background_color: BG_CARD.into(),
        ..default()
    }).with_children(|card| {
        // Left side: Checkbox + Title
        card.spawn(NodeBundle {
            style: Style { align_items: AlignItems::Center, ..default() },
            ..default()
        }).with_children(|left| {
            // Checkbox
            let checkbox_color = if task.completed { SUCCESS } else { BG_APP };
            let border_color = if task.completed { SUCCESS } else { TEXT_MUTED };
            
            left.spawn((ButtonBundle {
                style: Style {
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(6.0)),
                    margin: UiRect::right(Val::Px(16.0)),
                    ..default()
                },
                border_color: border_color.into(),
                background_color: checkbox_color.into(),
                ..default()
            }, ButtonAction::ToggleTask(task.entity)));

            // Title
            let text_color = if task.completed { TEXT_MUTED } else { TEXT_MAIN };
            left.spawn(TextBundle::from_section(
                &task.title,
                TextStyle {
                    font_size: 18.0,
                    color: text_color,
                    ..default()
                },
            ));
        });

        // Right side: Delete Button
        card.spawn((ButtonBundle {
            style: Style {
                padding: UiRect::axes(Val::Px(12.0), Val::Px(8.0)),
                border_radius: BorderRadius::all(RADIUS_BUTTON),
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        }, ButtonAction::DeleteTask(task.entity))).with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "Delete",
                TextStyle {
                    font_size: 14.0,
                    color: DANGER,
                    ..default()
                },
            ));
        });
    });
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut task_store: ResMut<TaskStore>,
) {
    for (interaction, mut color, action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match action {
                    ButtonAction::AddTask => {
                        let count = task_store.tasks.len() + 1;
                        task_store.add_task(format!("New Task #{}", count));
                    }
                    ButtonAction::ToggleTask(entity) => {
                        task_store.toggle_task(*entity);
                    }
                    ButtonAction::DeleteTask(entity) => {
                        task_store.delete_task(*entity);
                    }
                    _ => {}
                }
            }
            Interaction::Hovered => {
                match action {
                    ButtonAction::AddTask => *color = ACCENT_PRIMARY_HOVER.into(),
                    ButtonAction::DeleteTask(_) => *color = BG_CARD_HOVER.into(),
                    ButtonAction::SidebarMenu => *color = BG_CARD.into(),
                    _ => {}
                }
            }
            Interaction::None => {
                match action {
                    ButtonAction::AddTask => *color = ACCENT_PRIMARY.into(),
                    ButtonAction::DeleteTask(_) => *color = Color::NONE.into(),
                    ButtonAction::SidebarMenu => *color = Color::NONE.into(), // Simplified for demo
                    _ => {}
                }
            }
        }
    }
}
