pub mod theme;

use bevy::prelude::*;
use crate::systems::core_logic::{TaskList, AddTaskEvent, ToggleTaskEvent, DeleteTaskEvent};
use theme::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, (
               update_task_list_ui.run_if(resource_changed::<TaskList>()),
               handle_add_button,
               handle_task_buttons,
           ));
    }
}

#[derive(Component)]
struct TaskListContainer;

#[derive(Component)]
struct AddTaskButton;

#[derive(Component)]
struct ToggleButton(uuid::Uuid);

#[derive(Component)]
struct DeleteButton(uuid::Uuid);

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
    pressed: Color,
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Main Container
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(40.0)),
            ..default()
        },
        background_color: COLOR_BACKGROUND.into(),
        ..default()
    }).with_children(|parent| {
        // Header
        parent.spawn(TextBundle::from_section(
            "Task Manager",
            TextStyle {
                font_size: 40.0,
                color: COLOR_TEXT,
                ..default()
            }
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(20.0)),
            ..default()
        }));

        // Add Task Button
        parent.spawn((
            ButtonBundle {
                style: Style {
                    padding: UiRect::axes(Val::Px(20.0), Val::Px(10.0)),
                    margin: UiRect::bottom(Val::Px(30.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: COLOR_PRIMARY.into(),
                ..default()
            },
            AddTaskButton,
        )).with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "+ Add New Task",
                TextStyle {
                    font_size: 20.0,
                    color: Color::WHITE,
                    ..default()
                }
            ));
        });

        // Task List Container
        parent.spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    width: Val::Px(600.0),
                    row_gap: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            TaskListContainer,
        ));
    });
}

fn update_task_list_ui(
    mut commands: Commands,
    task_list: Res<TaskList>,
    q_container: Query<Entity, With<TaskListContainer>>,
) {
    let Ok(container_entity) = q_container.get_single() else { return };

    // Clear existing tasks from UI
    commands.entity(container_entity).clear_children();

    // Rebuild the list
    commands.entity(container_entity).with_children(|parent| {
        if task_list.tasks.is_empty() {
            parent.spawn(TextBundle::from_section(
                "No tasks yet. Add one above!",
                TextStyle {
                    font_size: 20.0,
                    color: COLOR_TEXT_DIM,
                    ..default()
                }
            ).with_style(Style {
                align_self: AlignSelf::Center,
                margin: UiRect::top(Val::Px(20.0)),
                ..default()
            }));
            return;
        }

        for task in &task_list.tasks {
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(15.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: COLOR_PANEL.into(),
                ..default()
            }).with_children(|row| {
                // Task Title
                let text_color = if task.completed { COLOR_TEXT_DIM } else { COLOR_TEXT };
                let title = if task.completed { format!("✓ {}", task.title) } else { task.title.clone() };
                
                row.spawn(TextBundle::from_section(
                    title,
                    TextStyle {
                        font_size: 22.0,
                        color: text_color,
                        ..default()
                    }
                ));

                // Buttons Container
                row.spawn(NodeBundle {
                    style: Style {
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|btns| {
                    // Toggle Button
                    let (bg_norm, bg_hov, bg_press) = if task.completed {
                        (COLOR_TEXT_DIM, COLOR_TEXT_DIM, COLOR_TEXT_DIM)
                    } else {
                        (COLOR_SUCCESS, COLOR_SUCCESS_HOVER, COLOR_SUCCESS_PRESS)
                    };

                    btns.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::axes(Val::Px(15.0), Val::Px(8.0)),
                                ..default()
                            },
                            background_color: bg_norm.into(),
                            ..default()
                        },
                        ToggleButton(task.id),
                        ButtonColors { normal: bg_norm, hovered: bg_hov, pressed: bg_press },
                    )).with_children(|btn| {
                        btn.spawn(TextBundle::from_section(
                            if task.completed { "Undo" } else { "Done" },
                            TextStyle { font_size: 16.0, color: Color::WHITE, ..default() }
                        ));
                    });

                    // Delete Button
                    btns.spawn((
                        ButtonBundle {
                            style: Style {
                                padding: UiRect::axes(Val::Px(15.0), Val::Px(8.0)),
                                ..default()
                            },
                            background_color: COLOR_DANGER.into(),
                            ..default()
                        },
                        DeleteButton(task.id),
                        ButtonColors { normal: COLOR_DANGER, hovered: COLOR_DANGER_HOVER, pressed: COLOR_DANGER_PRESS },
                    )).with_children(|btn| {
                        btn.spawn(TextBundle::from_section(
                            "Delete",
                            TextStyle { font_size: 16.0, color: Color::WHITE, ..default() }
                        ));
                    });
                });
            });
        }
    });
}

fn handle_add_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<AddTaskButton>),
    >,
    mut add_task_ev: EventWriter<AddTaskEvent>,
) {
    for (interaction, mut bg) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg = COLOR_PRIMARY_PRESS.into();
                let random_tasks = ["Review Code", "Write Documentation", "Fix Bugs", "Design UI", "Team Meeting", "Refactor Logic"];
                let task_name = random_tasks[rand::random::<usize>() % random_tasks.len()].to_string();
                add_task_ev.send(AddTaskEvent(task_name));
            }
            Interaction::Hovered => {
                *bg = COLOR_PRIMARY_HOVER.into();
            }
            Interaction::None => {
                *bg = COLOR_PRIMARY.into();
            }
        }
    }
}

fn handle_task_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonColors, Option<&ToggleButton>, Option<&DeleteButton>),
        (Changed<Interaction>, Or<(With<ToggleButton>, With<DeleteButton>)>),
    >,
    mut toggle_ev: EventWriter<ToggleTaskEvent>,
    mut delete_ev: EventWriter<DeleteTaskEvent>,
) {
    for (interaction, mut bg, colors, toggle, delete) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg = colors.pressed.into();
                if let Some(t) = toggle { toggle_ev.send(ToggleTaskEvent(t.0)); }
                if let Some(d) = delete { delete_ev.send(DeleteTaskEvent(d.0)); }
            }
            Interaction::Hovered => {
                *bg = colors.hovered.into();
            }
            Interaction::None => {
                *bg = colors.normal.into();
            }
        }
    }
}
